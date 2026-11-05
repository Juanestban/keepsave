pub type Result<T, E = SyncError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
  #[error("Google Drive isn't set up in this build")]
  NotConfigured,

  #[error("access was not granted in the browser")]
  Denied,

  #[error("sign-in took too long")]
  TimedOut,

  /// The refresh token was revoked or expired; the user must connect again.
  #[error("Google no longer accepts this connection")]
  Reauthorize,

  #[error("could not reach Google: {0}")]
  Network(#[from] reqwest::Error),

  #[error("Google returned {status}: {message}")]
  Http { status: u16, message: String },

  #[error("unexpected response from Google: {0}")]
  Protocol(String),

  #[error("credential store: {0}")]
  Keyring(String),

  #[error(transparent)]
  Io(#[from] std::io::Error),
}
