//! Core of KeepSave: the vault model on top of KDBX4, key derivation and the
//! password generator. This crate has no UI, network or async dependencies.

mod entry;
mod error;
pub mod generator;
mod kdf;
mod vault;

pub use entry::{CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem};
pub use error::{CoreError, Result};
pub use kdf::KdfParams;
pub use keepass::db::{EntryId, GroupId};
pub use vault::Vault;

/// Re-exported so callers can build secrets without depending on `secrecy`.
pub use secrecy::{ExposeSecret, SecretString};
