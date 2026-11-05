//! The Google refresh token in the OS credential store (Windows Credential
//! Manager, macOS Keychain, Secret Service on Linux).

use keyring::{Entry, Error};
use secrecy::{ExposeSecret, SecretString};

use crate::{Result, SyncError};

const SERVICE: &str = "KeepSave";
const ACCOUNT: &str = "google-drive";

fn entry() -> Result<Entry> {
  Entry::new(SERVICE, ACCOUNT).map_err(keyring_error)
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
