//! The Google refresh token and the user's OAuth client in the OS
//! credential store (Windows Credential Manager, macOS Keychain, Secret
//! Service on Linux).

use keyring::{Entry, Error};
use secrecy::{ExposeSecret, SecretString};

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, Zeroizing};

use crate::{Credentials, Result, SyncError};

const SERVICE: &str = "KeepSave";
const ACCOUNT: &str = "google-drive";
const CLIENT_ACCOUNT: &str = "google-client";

fn entry() -> Result<Entry> {
  Entry::new(SERVICE, ACCOUNT).map_err(keyring_error)
}

fn client_entry() -> Result<Entry> {
  Entry::new(SERVICE, CLIENT_ACCOUNT).map_err(keyring_error)
}

/// How the client is stored: one JSON entry, so ID and secret can't drift
/// apart.
#[derive(Serialize, Deserialize)]
struct StoredClient {
  client_id: String,
  client_secret: String,
}

impl Drop for StoredClient {
  fn drop(&mut self) {
    self.client_secret.zeroize();
  }
}

fn keyring_error(error: Error) -> SyncError {
  SyncError::Keyring(error.to_string())
}

/// The stored token, or `None` if Drive was never connected.
pub fn load_refresh_token() -> Result<Option<SecretString>> {
  match entry()?.get_password() {
    Ok(token) => Ok(Some(SecretString::from(token))),
    Err(Error::NoEntry) => Ok(None),
    Err(error) => Err(keyring_error(error)),
  }
}

pub(crate) fn save_refresh_token(token: &SecretString) -> Result<()> {
  entry()?.set_password(token.expose_secret()).map_err(keyring_error)
}

pub fn delete_refresh_token() -> Result<()> {
  match entry()?.delete_credential() {
    Ok(()) | Err(Error::NoEntry) => Ok(()),
    Err(error) => Err(keyring_error(error)),
  }
}

/// The OAuth client the user set up in KeepSave, if any.
pub fn load_client() -> Result<Option<Credentials>> {
  let json = match client_entry()?.get_password() {
    Ok(json) => Zeroizing::new(json),
    Err(Error::NoEntry) => return Ok(None),
    Err(error) => return Err(keyring_error(error)),
  };
  let stored: StoredClient = serde_json::from_str(&json).map_err(|error| SyncError::Keyring(error.to_string()))?;
  Ok(Some(Credentials { client_id: stored.client_id.clone(), client_secret: SecretString::from(stored.client_secret.as_str()) }))
}

pub fn save_client(credentials: &Credentials) -> Result<()> {
  let stored = StoredClient { client_id: credentials.client_id.clone(), client_secret: credentials.client_secret.expose_secret().to_owned() };
  let json = Zeroizing::new(serde_json::to_string(&stored).map_err(|error| SyncError::Keyring(error.to_string()))?);
  client_entry()?.set_password(&json).map_err(keyring_error)
}

pub fn delete_client() -> Result<()> {
  match client_entry()?.delete_credential() {
    Ok(()) | Err(Error::NoEntry) => Ok(()),
    Err(error) => Err(keyring_error(error)),
  }
}
