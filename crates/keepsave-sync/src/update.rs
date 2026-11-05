//! Finds newer KeepSave releases on GitHub and downloads them, checked
//! against the release's `SHA256SUMS.txt`.

use std::{
  io::Write,
  path::{Path, PathBuf},
  time::Duration,
};

use reqwest::StatusCode;
use semver::Version;
use serde::Deserialize;
use sha2::{Digest, Sha256};

const LATEST_RELEASE: &str = "https://api.github.com/repos/Juanestban/keepsave/releases/latest";
const CHECKSUMS: &str = "SHA256SUMS.txt";

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
  #[error("could not reach GitHub: {0}")]
  Network(#[from] reqwest::Error),

  #[error("GitHub returned {0}")]
  Http(u16),

  #[error("unexpected release on GitHub: {0}")]
  Release(String),

  /// The release has no file for this kind of install.
  #[error("this release has no {0}")]
  MissingAsset(String),

  #[error("the download doesn't match its checksum")]
  Checksum,

  #[error(transparent)]
  Io(#[from] std::io::Error),
}

/// A published release newer than the running app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
  pub version: Version,
  /// The release page, for its notes.
  pub page_url: String,
  pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Asset {
  pub name: String,
  #[serde(rename = "browser_download_url")]
  pub url: String,
  pub size: u64,
}

impl Release {
  pub fn asset(&self, name: &str) -> Option<&Asset> {
    self.assets.iter().find(|asset| asset.name == name)
  }

  /// `keepsave-<version>-x64.msi`, as `scripts/package.ps1` names it.
  pub fn msi_name(&self) -> String {
    format!("keepsave-{}-x64.msi", self.version)
  }

  /// `keepsave-<version>-x64-portable.exe`.
  pub fn portable_name(&self) -> String {
    format!("keepsave-{}-x64-portable.exe", self.version)
  }
}

#[derive(Deserialize)]
struct GitHubRelease {
  tag_name: String,
  html_url: String,
  #[serde(default)]
  draft: bool,
  #[serde(default)]
  prerelease: bool,
  #[serde(default)]
  assets: Vec<Asset>,
}

/// The latest release if it is newer than `current`.
pub async fn check(current: &Version) -> Result<Option<Release>, UpdateError> {
  let response = http_client()?.get(LATEST_RELEASE).header("Accept", "application/vnd.github+json").send().await?;
  // No release published yet.
  if response.status() == StatusCode::NOT_FOUND {
    return Ok(None);
  }
  let response = check_status(response)?;
  newer_release(response.json().await?, current)
}

fn newer_release(release: GitHubRelease, current: &Version) -> Result<Option<Release>, UpdateError> {
  if release.draft || release.prerelease {
    return Ok(None);
  }
  let tag = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);
  let version = Version::parse(tag).map_err(|_| UpdateError::Release(release.tag_name.clone()))?;
  Ok((version > *current).then_some(Release { version, page_url: release.html_url, assets: release.assets }))
}

/// Downloads `name` from `release` to `dest`, calling `progress` with the
/// bytes so far and the total. The file is only left at `dest` if it
/// matches its line in `SHA256SUMS.txt`.
pub async fn download(release: &Release, name: &str, dest: &Path, progress: impl Fn(u64, u64)) -> Result<PathBuf, UpdateError> {
  let asset = release.asset(name).ok_or_else(|| UpdateError::MissingAsset(name.to_owned()))?;
  let sums = release.asset(CHECKSUMS).ok_or_else(|| UpdateError::MissingAsset(CHECKSUMS.to_owned()))?;
  let http = http_client()?;

  let sums = check_status(http.get(&sums.url).send().await?)?.text().await?;
  let expected = expected_hash(&sums, name).ok_or_else(|| UpdateError::MissingAsset(format!("{name} in {CHECKSUMS}")))?;

  if let Some(dir) = dest.parent() {
    std::fs::create_dir_all(dir)?;
  }
  let mut partial = dest.as_os_str().to_owned();
  partial.push(".part");
  let partial = PathBuf::from(partial);

  let mut response = check_status(http.get(&asset.url).send().await?)?;
  let total = response.content_length().unwrap_or(asset.size);
  let mut file = std::fs::File::create(&partial)?;
  let mut hasher = Sha256::new();
  let mut done = 0;
  while let Some(chunk) = response.chunk().await? {
    file.write_all(&chunk)?;
    hasher.update(&chunk);
    done += chunk.len() as u64;
    progress(done, total);
  }
  file.sync_all()?;
  drop(file);

  if hex(&hasher.finalize()) != expected {
    let _ = std::fs::remove_file(&partial);
    return Err(UpdateError::Checksum);
  }
  std::fs::rename(&partial, dest)?;
  Ok(dest.to_owned())
}

/// The hash on `name`'s line of a `sha256sum`-style file, lowercase.
fn expected_hash(sums: &str, name: &str) -> Option<String> {
  sums.lines().find_map(|line| {
    let (hash, file) = line.trim().split_once(char::is_whitespace)?;
    // `sha256sum -b` marks binary files with a `*`.
    (file.trim_start().trim_start_matches('*') == name).then(|| hash.to_ascii_lowercase())
  })
}

fn hex(bytes: &[u8]) -> String {
  bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn check_status(response: reqwest::Response) -> Result<reqwest::Response, UpdateError> {
  let status = response.status();
  if status.is_success() { Ok(response) } else { Err(UpdateError::Http(status.as_u16())) }
}

/// Unlike Drive's client, no limit on the whole request: a download can take
/// minutes on a slow connection. A stalled one still times out.
fn http_client() -> Result<reqwest::Client, UpdateError> {
  let _ = rustls::crypto::ring::default_provider().install_default();
  Ok(
    reqwest::Client::builder()
      .connect_timeout(Duration::from_secs(30))
      .read_timeout(Duration::from_secs(60))
      .user_agent(concat!("KeepSave/", env!("CARGO_PKG_VERSION")))
      .build()?,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  fn github(tag: &str) -> GitHubRelease {
    GitHubRelease {
      tag_name: tag.into(),
      html_url: "https://github.com/Juanestban/keepsave/releases/tag/x".into(),
      draft: false,
      prerelease: false,
      assets: Vec::new(),
    }
  }

  #[test]
  fn only_newer_published_releases_count() {
    let current = Version::new(1, 0, 0);
    assert_eq!(newer_release(github("v1.1.0"), &current).unwrap().unwrap().version, Version::new(1, 1, 0));
    assert!(newer_release(github("v1.0.0"), &current).unwrap().is_none());
    assert!(newer_release(github("v0.9.0"), &current).unwrap().is_none());
    assert!(newer_release(GitHubRelease { prerelease: true, ..github("v2.0.0") }, &current).unwrap().is_none());
    assert!(newer_release(GitHubRelease { draft: true, ..github("v2.0.0") }, &current).unwrap().is_none());
    assert!(newer_release(github("latest"), &current).is_err());
    // A running pre-release updates to its final version.
    assert!(newer_release(github("v1.1.0"), &Version::parse("1.1.0-rc.1").unwrap()).unwrap().is_some());
  }

  #[test]
  fn releases_parse_from_github_json() {
    let json = r#"{"tag_name":"v1.2.0","html_url":"https://x","draft":false,"prerelease":false,
      "assets":[{"name":"SHA256SUMS.txt","browser_download_url":"https://x/s","size":10,"state":"uploaded"}]}"#;
    let release = newer_release(serde_json::from_str(json).unwrap(), &Version::new(1, 0, 0)).unwrap().unwrap();
    assert_eq!(release.asset("SHA256SUMS.txt").unwrap().url, "https://x/s");
    assert_eq!(release.msi_name(), "keepsave-1.2.0-x64.msi");
    assert_eq!(release.portable_name(), "keepsave-1.2.0-x64-portable.exe");
  }

  #[test]
  fn checksums_are_found_by_file_name() {
    let sums = "ABCDEF  keepsave-1.2.0-x64.msi\n0123 *keepsave-1.2.0-x64-portable.exe\n";
    assert_eq!(expected_hash(sums, "keepsave-1.2.0-x64.msi").as_deref(), Some("abcdef"));
    assert_eq!(expected_hash(sums, "keepsave-1.2.0-x64-portable.exe").as_deref(), Some("0123"));
    assert_eq!(expected_hash(sums, "other.exe"), None);
    assert_eq!(hex(&[0x0a, 0xff]), "0aff");
  }
}
