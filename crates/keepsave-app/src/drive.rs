//! Whether Google Drive is available and connected, and how its errors and
//! files read in the UI.

use std::sync::Arc;

use chrono::{DateTime, Local};
use keepsave_sync::{BrowserPage, Credentials, DriveClient, RemoteFile, SyncError};

use crate::i18n::I18n;

pub enum DriveState {
  /// No OAuth client yet: Drive offers the setup guide.
  NotSetUp,
  /// `error` explains why the last attempt or connection ended, if it did.
  Disconnected {
    error: Option<String>,
  },
  /// Waiting for the user in the browser.
  Connecting,
  Connected(Arc<DriveClient>),
}

/// The user's OAuth client from the credential store, if they set one up.
pub fn load_client() -> Option<Credentials> {
  // An unreadable credential store is treated as no client: the guide
  // can store a new one.
  keepsave_sync::load_client().ok().flatten()
}

impl DriveState {
  /// Reconnects with the refresh token stored by an earlier session. Makes
  /// no network request.
  pub fn startup(credentials: Option<&Credentials>, i18n: &I18n) -> Self {
    let Some(credentials) = credentials else {
      return DriveState::NotSetUp;
    };
    let client = keepsave_sync::load_refresh_token().and_then(|token| token.map(|token| DriveClient::new(credentials.clone(), token)).transpose());
    match client {
      Ok(Some(client)) => DriveState::Connected(Arc::new(client)),
      Ok(None) => DriveState::Disconnected { error: None },
      Err(error) => DriveState::Disconnected { error: Some(error_message(i18n, &error)) },
    }
  }

  pub fn client(&self) -> Option<&Arc<DriveClient>> {
    match self {
      DriveState::Connected(client) => Some(client),
      _ => None,
    }
  }
}

/// What the browser tab says after signing in, and after declining.
pub fn browser_pages(i18n: &I18n) -> (BrowserPage, BrowserPage) {
  let page = |title, message| BrowserPage { title: i18n.tr(title), message: i18n.tr(message) };
  (page("drive-browser-success-title", "drive-browser-success-body"), page("drive-browser-denied-title", "drive-browser-denied-body"))
}

pub fn error_message(i18n: &I18n, error: &SyncError) -> String {
  match error {
    SyncError::NotConfigured => i18n.tr("drive-unavailable"),
    SyncError::Denied => i18n.tr("drive-error-denied"),
    SyncError::TimedOut => i18n.tr("drive-error-timeout"),
    SyncError::Reauthorize => i18n.tr("drive-error-reauthorize"),
    SyncError::Network(_) => i18n.tr("drive-error-network"),
    SyncError::RemoteMissing => i18n.tr("drive-error-missing"),
    SyncError::Vault(keepsave_core::CoreError::InvalidKey) => i18n.tr("drive-error-password"),
    other => i18n.tr_arg("error-generic", "details", other.to_string()),
  }
}

/// Errors worth retrying on their own: the network or Google's servers.
pub fn is_transient(error: &SyncError) -> bool {
  matches!(error, SyncError::Network(_)) || matches!(error, SyncError::Http { status, .. } if *status >= 500 || *status == 429)
}

/// "1.2 MB · 2026-11-04 10:00", or whatever part is known.
pub fn file_details(file: &RemoteFile) -> String {
  let size = file.size_bytes().map(format_size);
  let date = file
    .modified_time
    .as_deref()
    .and_then(|time| DateTime::parse_from_rfc3339(time).ok())
    .map(|time| time.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string());
  [size, date].into_iter().flatten().collect::<Vec<_>>().join(" \u{b7} ")
}

fn format_size(bytes: u64) -> String {
  const KB: f64 = 1024.0;
  let bytes_f = bytes as f64;
  if bytes_f < KB {
    format!("{bytes} B")
  } else if bytes_f < KB * KB {
    format!("{:.1} KB", bytes_f / KB)
  } else {
    format!("{:.1} MB", bytes_f / (KB * KB))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sizes_are_human_readable() {
    assert_eq!(format_size(512), "512 B");
    assert_eq!(format_size(2048), "2.0 KB");
    assert_eq!(format_size(3 * 1024 * 1024 + 100_000), "3.1 MB");
  }

  #[test]
  fn file_details_skip_unknown_parts() {
    let mut file =
      RemoteFile { id: "a".into(), name: "v.kdbx".into(), size: Some("2048".into()), modified_time: None, version: None, trashed: false };
    assert_eq!(file_details(&file), "2.0 KB");
    file.modified_time = Some("2026-11-04T10:00:00Z".into());
    assert!(file_details(&file).starts_with("2.0 KB \u{b7} 2026-11-0"));
  }
}
