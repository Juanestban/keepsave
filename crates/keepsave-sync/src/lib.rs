//! Google Drive for KeepSave: OAuth sign-in (loopback redirect with PKCE),
//! the refresh token in the OS credential store, a small Drive REST client
//! behind the [`RemoteStore`] trait, and the sync that merges and uploads a
//! vault.

mod drive;
mod error;
mod oauth;
mod sync;
mod token_store;

use std::future::Future;

use secrecy::SecretString;
use serde::Deserialize;

pub use drive::{BACKUPS_FOLDER_NAME, DriveClient, FOLDER_NAME};
pub use error::{Result, SyncError};
pub use oauth::BrowserPage;
pub use sync::{MAX_BACKUPS, SyncLink, SyncOutcome, sync_vault};
pub use token_store::{delete_refresh_token, load_refresh_token};

/// The OAuth client of this build. Not `Debug`: it holds the client secret.
#[derive(Clone)]
pub struct Credentials {
  pub client_id: String,
  pub client_secret: SecretString,
}

impl Credentials {
  /// The client embedded at build time, if the build had one.
  pub fn embedded() -> Option<Self> {
    let client_id = option_env!("KEEPSAVE_GOOGLE_CLIENT_ID")?;
    let client_secret = option_env!("KEEPSAVE_GOOGLE_CLIENT_SECRET")?;
    Some(Self { client_id: client_id.to_owned(), client_secret: SecretString::from(client_secret) })
  }
}

/// A vault file stored remotely.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteFile {
  pub id: String,
  pub name: String,
  /// Bytes, as Drive reports int64 values: a decimal string.
  #[serde(default)]
  pub size: Option<String>,
  /// RFC 3339.
  #[serde(default)]
  pub modified_time: Option<String>,
  /// Increases on every change to the file.
  #[serde(default)]
  pub version: Option<String>,
  #[serde(default)]
  pub trashed: bool,
}

impl RemoteFile {
  pub fn size_bytes(&self) -> Option<u64> {
    self.size.as_deref()?.parse().ok()
  }
}

/// Where vaults are kept remotely. Drive implements it; tests can use a
/// fake.
pub trait RemoteStore: Send + Sync {
  /// The `.kdbx` files in the KeepSave folder, by name.
  fn list_vaults(&self) -> impl Future<Output = Result<Vec<RemoteFile>>> + Send;
  fn download(&self, id: &str) -> impl Future<Output = Result<Vec<u8>>> + Send;
  /// Creates a new file in the KeepSave folder.
  fn create(&self, name: &str, bytes: Vec<u8>) -> impl Future<Output = Result<RemoteFile>> + Send;
  /// Replaces the contents of an existing file.
  fn update(&self, id: &str, bytes: Vec<u8>) -> impl Future<Output = Result<RemoteFile>> + Send;
  fn metadata(&self, id: &str) -> impl Future<Output = Result<RemoteFile>> + Send;
  /// Copies the file into the backups folder under `name`.
  fn backup(&self, id: &str, name: &str) -> impl Future<Output = Result<RemoteFile>> + Send;
  /// Every file in the backups folder, of all vaults.
  fn list_backups(&self) -> impl Future<Output = Result<Vec<RemoteFile>>> + Send;
  fn delete(&self, id: &str) -> impl Future<Output = Result<()>> + Send;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn remote_files_parse_from_drive_json() {
    let json = r#"{"id":"abc","name":"Personal.kdbx","size":"2048","modifiedTime":"2026-11-04T10:00:00.000Z","version":"7"}"#;
    let file: RemoteFile = serde_json::from_str(json).unwrap();
    assert_eq!(file.id, "abc");
    assert_eq!(file.size_bytes(), Some(2048));
    assert_eq!(file.version.as_deref(), Some("7"));
    assert!(!file.trashed);

    let bare: RemoteFile = serde_json::from_str(r#"{"id":"x","name":"y"}"#).unwrap();
    assert_eq!(bare.size_bytes(), None);
  }
}
