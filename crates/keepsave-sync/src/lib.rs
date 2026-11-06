//! Google Drive for KeepSave: OAuth sign-in (loopback redirect with PKCE),
//! the refresh token in the OS credential store, a small Drive REST client
//! behind the [`RemoteStore`] trait, and the sync that merges and uploads a
//! vault. [`update`] finds and downloads new KeepSave releases.

mod drive;
mod error;
mod oauth;
mod sync;
mod token_store;
pub mod update;

use std::future::Future;

use secrecy::SecretString;
use serde::Deserialize;

pub use drive::{BACKUPS_FOLDER_NAME, DriveClient, FOLDER_NAME};
pub use error::{Result, SyncError};
pub use oauth::BrowserPage;
pub use sync::{MAX_BACKUPS, SyncLink, SyncOutcome, sync_vault};
pub use token_store::{delete_client, delete_refresh_token, load_client, load_refresh_token, save_client};

/// The user's own Google OAuth client, set up in the app and kept in the OS
/// credential store. Builds never embed one: anything in a published `.exe`
/// can be read back. Not `Debug`: it holds the client secret.
#[derive(Clone)]
pub struct Credentials {
  pub client_id: String,
  pub client_secret: SecretString,
}

impl Credentials {
  /// A client typed or pasted by the user, if it looks like a Google
  /// desktop client: an ID ending in `.apps.googleusercontent.com` and a
  /// secret, both without spaces.
  pub fn parse(client_id: &str, client_secret: &str) -> Option<Self> {
    let client_id = client_id.trim();
    let client_secret = client_secret.trim();
    let prefix = client_id.strip_suffix(GOOGLE_CLIENT_SUFFIX)?;
    let valid = |text: &str| !text.is_empty() && !text.contains(char::is_whitespace);
    (valid(prefix) && valid(client_secret)).then(|| Self { client_id: client_id.to_owned(), client_secret: SecretString::from(client_secret) })
  }
}

/// Every Google OAuth client ID ends with this.
pub const GOOGLE_CLIENT_SUFFIX: &str = ".apps.googleusercontent.com";

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
  fn clients_are_checked_before_saving() {
    use secrecy::ExposeSecret;
    let parsed = Credentials::parse(
      "  123-abc.apps.googleusercontent.com ",
      " GOCSPX-x 
",
    )
    .unwrap();
    assert_eq!(parsed.client_id, "123-abc.apps.googleusercontent.com");
    assert_eq!(parsed.client_secret.expose_secret(), "GOCSPX-x");
    assert!(Credentials::parse(".apps.googleusercontent.com", "s").is_none());
    assert!(Credentials::parse("123-abc", "s").is_none());
    assert!(Credentials::parse("1 2.apps.googleusercontent.com", "s").is_none());
    assert!(Credentials::parse("123.apps.googleusercontent.com", "").is_none());
  }

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
