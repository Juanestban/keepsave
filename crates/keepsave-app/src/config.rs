//! User settings in `config.toml` under the platform config dir, e.g.
//! `%APPDATA%\KeepSave\config.toml`. Never holds secrets.

use std::{
  fs, io,
  path::{Path, PathBuf},
};

use directories::BaseDirs;
use keepsave_sync::RemoteFile;
use serde::{Deserialize, Serialize};

const MAX_RECENT_VAULTS: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
  /// Language code (`en`, `es`). `None` follows the OS.
  pub language: Option<String>,
  /// Most recently used first.
  pub recent_vaults: Vec<PathBuf>,
  /// Lock after this long without input; 0 turns it off.
  pub lock_after_minutes: u32,
  pub lock_on_minimize: bool,
  pub lock_on_os_sleep: bool,
  /// Clear copied secrets after this long; 0 keeps them.
  pub clipboard_clear_seconds: u32,
  /// Local vaults that have a copy on Google Drive. Kept last: TOML writes
  /// arrays of tables after plain values.
  pub drive_links: Vec<DriveLink>,
}

/// A local vault and its file on Drive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriveLink {
  pub path: PathBuf,
  pub file_id: String,
  pub name: String,
  /// Drive's `version` and `modifiedTime` when last synced, so later syncs
  /// can tell whether the remote file changed.
  pub version: Option<String>,
  pub modified_time: Option<String>,
  /// The local vault has changes Drive doesn't have yet.
  #[serde(default)]
  pub pending: bool,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      language: None,
      recent_vaults: Vec::new(),
      lock_after_minutes: 5,
      lock_on_minimize: true,
      lock_on_os_sleep: true,
      clipboard_clear_seconds: 15,
      drive_links: Vec::new(),
    }
  }
}

impl Config {
  pub fn default_path() -> Option<PathBuf> {
    BaseDirs::new().map(|dirs| dirs.config_dir().join("KeepSave").join("config.toml"))
  }

  /// Loads the config, falling back to defaults when it is missing or
  /// unreadable, so a broken file never keeps the app from starting.
  pub fn load_from(path: &Path) -> Self {
    fs::read_to_string(path).ok().and_then(|text| toml::from_str(&text).ok()).unwrap_or_default()
  }

  pub fn save_to(&self, path: &Path) -> io::Result<()> {
    if let Some(dir) = path.parent() {
      fs::create_dir_all(dir)?;
    }
    let text = toml::to_string_pretty(self).map_err(io::Error::other)?;
    let mut tmp_name = path.as_os_str().to_owned();
    tmp_name.push(".tmp");
    fs::write(&tmp_name, text)?;
    fs::rename(&tmp_name, path)
  }

  /// Moves `path` to the front of the recent list.
  pub fn remember_vault(&mut self, path: &Path) {
    self.recent_vaults.retain(|known| known != path);
    self.recent_vaults.insert(0, path.to_owned());
    self.recent_vaults.truncate(MAX_RECENT_VAULTS);
  }

  pub fn forget_vault(&mut self, path: &Path) {
    self.recent_vaults.retain(|known| known != path);
  }

  pub fn drive_link(&self, path: &Path) -> Option<&DriveLink> {
    self.drive_links.iter().find(|link| link.path == path)
  }

  /// Links `path` to `file`, replacing any earlier link of that vault.
  pub fn link_drive(&mut self, path: &Path, file: &RemoteFile) {
    self.unlink_drive(path);
    self.drive_links.push(DriveLink {
      path: path.to_owned(),
      file_id: file.id.clone(),
      name: file.name.clone(),
      version: file.version.clone(),
      modified_time: file.modified_time.clone(),
      pending: false,
    });
  }

  /// Records a finished sync: Drive now holds `file`.
  pub fn drive_synced(&mut self, path: &Path, file: &RemoteFile, pending: bool) {
    if let Some(link) = self.drive_links.iter_mut().find(|link| link.path == path) {
      link.version = file.version.clone();
      link.modified_time = file.modified_time.clone();
      link.pending = pending;
    }
  }

  pub fn set_drive_pending(&mut self, path: &Path, pending: bool) {
    if let Some(link) = self.drive_links.iter_mut().find(|link| link.path == path) {
      link.pending = pending;
    }
  }

  pub fn unlink_drive(&mut self, path: &Path) {
    self.drive_links.retain(|link| link.path != path);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn round_trip_and_missing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested").join("config.toml");
    assert_eq!(Config::load_from(&path), Config::default());

    let mut config = Config { language: Some("es".into()), ..Config::default() };
    config.remember_vault(Path::new("C:/vaults/personal.kdbx"));
    config.save_to(&path).unwrap();
    assert_eq!(Config::load_from(&path), config);
  }

  #[test]
  fn missing_keys_take_their_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    fs::write(
      &path,
      "language = \"es\"
lock_after_minutes = 2
",
    )
    .unwrap();
    let config = Config::load_from(&path);
    assert_eq!(config.language.as_deref(), Some("es"));
    assert_eq!(config.lock_after_minutes, 2);
    assert!(config.lock_on_minimize);
    assert_eq!(config.clipboard_clear_seconds, 15);
  }

  #[test]
  fn drive_links_round_trip_and_replace() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let file = |id: &str, version: Option<&str>| RemoteFile {
      id: id.into(),
      name: "Personal.kdbx".into(),
      size: None,
      modified_time: None,
      version: version.map(Into::into),
      trashed: false,
    };

    let mut config = Config::default();
    config.remember_vault(Path::new("C:/v/Personal.kdbx"));
    config.link_drive(Path::new("C:/v/Personal.kdbx"), &file("a", Some("3")));
    config.link_drive(Path::new("C:/v/Personal.kdbx"), &file("b", None));
    config.save_to(&path).unwrap();

    let loaded = Config::load_from(&path);
    assert_eq!(loaded, config);
    assert_eq!(loaded.drive_links.len(), 1);
    assert_eq!(loaded.drive_link(Path::new("C:/v/Personal.kdbx")).unwrap().file_id, "b");

    let vault = Path::new("C:/v/Personal.kdbx");
    config.set_drive_pending(vault, true);
    assert!(config.drive_link(vault).unwrap().pending);
    config.drive_synced(vault, &file("b", Some("9")), false);
    let link = config.drive_link(vault).unwrap();
    assert_eq!((link.version.as_deref(), link.pending), (Some("9"), false));

    config.unlink_drive(vault);
    assert!(config.drive_links.is_empty());
  }

  #[test]
  fn corrupt_file_falls_back_to_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    fs::write(&path, "recent_vaults = 42").unwrap();
    assert_eq!(Config::load_from(&path), Config::default());
  }

  #[test]
  fn recent_vaults_are_unique_ordered_and_capped() {
    let mut config = Config::default();
    for i in 0..10 {
      config.remember_vault(Path::new(&format!("v{i}.kdbx")));
    }
    config.remember_vault(Path::new("v5.kdbx"));
    assert_eq!(config.recent_vaults.len(), MAX_RECENT_VAULTS);
    assert_eq!(config.recent_vaults[0], Path::new("v5.kdbx"));
    assert_eq!(config.recent_vaults.iter().filter(|p| *p == Path::new("v5.kdbx")).count(), 1);

    config.forget_vault(Path::new("v5.kdbx"));
    assert!(!config.recent_vaults.iter().any(|p| p == Path::new("v5.kdbx")));
  }
}
