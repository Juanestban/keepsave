use keepass::db::{DatabaseOpenError, DatabaseSaveError};

pub type Result<T, E = CoreError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
  #[error("wrong master password or damaged file")]
  InvalidKey,

  #[error("could not read the vault: {0}")]
  Open(#[source] DatabaseOpenError),

  #[error("could not write the vault: {0}")]
  Save(#[from] DatabaseSaveError),

  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error("entry not found")]
  EntryNotFound,

  #[error("group not found")]
  GroupNotFound,

  #[error("the root group cannot be moved or deleted")]
  RootGroup,

  #[error("history version not found")]
  HistoryNotFound,

  #[error("key derivation failed: {0}")]
  Kdf(String),
}

impl From<DatabaseOpenError> for CoreError {
  fn from(err: DatabaseOpenError) -> Self {
    match err {
      DatabaseOpenError::Key(_) => CoreError::InvalidKey,
      other => CoreError::Open(other),
    }
  }
}
