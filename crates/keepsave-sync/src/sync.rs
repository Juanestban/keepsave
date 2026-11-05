//! One sync of a local vault with its copy on a [`RemoteStore`]: merge the
//! remote changes in, then upload what the remote doesn't have, keeping a
//! backup of what it replaces.

use chrono::{DateTime, NaiveDateTime, Utc};
use keepsave_core::Vault;

use crate::{RemoteFile, RemoteStore, Result, SyncError};

/// Backups kept per vault; older ones are deleted.
pub const MAX_BACKUPS: usize = 10;
const BACKUP_TIME_FORMAT: &str = "%Y-%m-%d %H-%M-%S";

/// What the local side knows about its remote copy.
#[derive(Debug, Clone)]
pub struct SyncLink {
  pub file_id: String,
  /// The remote `version` at the last sync.
  pub version: Option<String>,
}

pub struct SyncOutcome {
  /// The remote file as it is now; its version is the new sync point.
  pub file: RemoteFile,
  /// The remote copy, decrypted, when it had changed. The caller merges it
  /// into the open vault so edits made during the sync are kept.
  pub remote: Option<Vault>,
  pub uploaded: bool,
}

/// Syncs `snapshot` with its remote copy. `local_changed` says whether the
/// snapshot has changes the remote doesn't have yet.
pub async fn sync_vault<S: RemoteStore>(store: &S, snapshot: Vault, link: &SyncLink, local_changed: bool, now: DateTime<Utc>) -> Result<SyncOutcome> {
  let remote_file = match store.metadata(&link.file_id).await {
    Ok(file) if file.trashed => return Err(SyncError::RemoteMissing),
    Ok(file) => file,
    Err(SyncError::Http { status: 404, .. }) => return Err(SyncError::RemoteMissing),
    Err(error) => return Err(error),
  };
  let remote_changed = remote_file.version.is_none() || remote_file.version != link.version;
  if !remote_changed && !local_changed {
    return Ok(SyncOutcome { file: remote_file, remote: None, uploaded: false });
  }

  let (to_upload, remote) = if remote_changed {
    let bytes = store.download(&link.file_id).await?;
    let (merged, remote) = blocking(move || {
      // Decrypting re-derives the key, which takes about a second.
      let remote = snapshot.open_sibling(&bytes)?;
      let mut merged = snapshot;
      merged.merge_from(&remote)?;
      Ok((merged, remote))
    })
    .await?;
    (local_changed.then_some(merged), Some(remote))
  } else {
    (Some(snapshot), None)
  };

  let Some(vault) = to_upload else {
    return Ok(SyncOutcome { file: remote_file, remote, uploaded: false });
  };
  let stem = backup_stem(&remote_file.name);
  store.backup(&link.file_id, &backup_name(&stem, now)).await?;
  let bytes = blocking(move || Ok(vault.to_bytes()?)).await?;
  let file = store.update(&link.file_id, bytes).await?;
  // The upload already succeeded; a failed cleanup only leaves extra copies.
  let _ = prune_backups(store, &stem).await;
  Ok(SyncOutcome { file, remote, uploaded: true })
}

async fn blocking<T: Send + 'static>(task: impl FnOnce() -> Result<T> + Send + 'static) -> Result<T> {
  tokio::task::spawn_blocking(task).await.map_err(std::io::Error::other)?
}

/// `Personal.kdbx` → `Personal`.
fn backup_stem(name: &str) -> String {
  let lower = name.to_ascii_lowercase();
  match lower.strip_suffix(".kdbx") {
    Some(stem) => name[..stem.len()].to_owned(),
    None => name.to_owned(),
  }
}

/// `Personal 2026-11-04 21-30-05.kdbx`: sorts by time, and is unambiguous
/// even when another vault's name starts the same way.
fn backup_name(stem: &str, now: DateTime<Utc>) -> String {
  format!("{stem} {}.kdbx", now.format(BACKUP_TIME_FORMAT))
}

/// When `name` is a backup of the vault `stem`, its timestamp.
fn backup_time(stem: &str, name: &str) -> Option<NaiveDateTime> {
  let rest = name.strip_prefix(stem)?.strip_prefix(' ')?.strip_suffix(".kdbx")?;
  NaiveDateTime::parse_from_str(rest, BACKUP_TIME_FORMAT).ok()
}

async fn prune_backups<S: RemoteStore>(store: &S, stem: &str) -> Result<()> {
  let mut backups: Vec<(NaiveDateTime, RemoteFile)> =
    store.list_backups().await?.into_iter().filter_map(|file| Some((backup_time(stem, &file.name)?, file))).collect();
  backups.sort_by_key(|(time, _)| std::cmp::Reverse(*time));
  for (_, file) in backups.into_iter().skip(MAX_BACKUPS) {
    store.delete(&file.id).await?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::{
    collections::BTreeMap,
    sync::{Mutex, MutexGuard},
  };

  use chrono::TimeZone;
  use keepsave_core::{CoreError, EntryData, KdfParams, SecretString};

  use super::*;

  /// An in-memory store: files by id, with a version bumped on each write.
  #[derive(Default)]
  struct FakeStore {
    state: Mutex<State>,
  }

  #[derive(Default)]
  struct State {
    next_id: u64,
    files: BTreeMap<String, (RemoteFile, Vec<u8>, bool)>,
  }

  impl FakeStore {
    fn state(&self) -> MutexGuard<'_, State> {
      self.state.lock().unwrap()
    }

    fn put(&self, name: &str, bytes: Vec<u8>, backup: bool) -> RemoteFile {
      let mut state = self.state();
      state.next_id += 1;
      let file = RemoteFile {
        id: format!("f{}", state.next_id),
        name: name.into(),
        size: None,
        modified_time: None,
        version: Some("1".into()),
        trashed: false,
      };
      state.files.insert(file.id.clone(), (file.clone(), bytes, backup));
      file
    }

    fn bytes(&self, id: &str) -> Vec<u8> {
      self.state().files[id].1.clone()
    }

    fn backups(&self) -> Vec<String> {
      self.state().files.values().filter(|(_, _, backup)| *backup).map(|(file, _, _)| file.name.clone()).collect()
    }
  }

  fn missing() -> SyncError {
    SyncError::Http { status: 404, message: "not found".into() }
  }

  impl RemoteStore for FakeStore {
    async fn list_vaults(&self) -> Result<Vec<RemoteFile>> {
      Ok(self.state().files.values().filter(|(_, _, backup)| !backup).map(|(file, _, _)| file.clone()).collect())
    }

    async fn download(&self, id: &str) -> Result<Vec<u8>> {
      self.state().files.get(id).map(|(_, bytes, _)| bytes.clone()).ok_or_else(missing)
    }

    async fn create(&self, name: &str, bytes: Vec<u8>) -> Result<RemoteFile> {
      Ok(self.put(name, bytes, false))
    }

    async fn update(&self, id: &str, bytes: Vec<u8>) -> Result<RemoteFile> {
      let mut state = self.state();
      let (file, stored, _) = state.files.get_mut(id).ok_or_else(missing)?;
      let version: u64 = file.version.as_deref().unwrap_or("0").parse().unwrap();
      file.version = Some((version + 1).to_string());
      *stored = bytes;
      Ok(file.clone())
    }

    async fn metadata(&self, id: &str) -> Result<RemoteFile> {
      self.state().files.get(id).map(|(file, _, _)| file.clone()).ok_or_else(missing)
    }

    async fn backup(&self, id: &str, name: &str) -> Result<RemoteFile> {
      let bytes = self.download(id).await?;
      Ok(self.put(name, bytes, true))
    }

    async fn list_backups(&self) -> Result<Vec<RemoteFile>> {
      Ok(self.state().files.values().filter(|(_, _, backup)| *backup).map(|(file, _, _)| file.clone()).collect())
    }

    async fn delete(&self, id: &str) -> Result<()> {
      self.state().files.remove(id).map(|_| ()).ok_or_else(missing)
    }
  }

  fn vault() -> Vault {
    Vault::create("Personal", SecretString::from("pw"), KdfParams::insecure_fast())
  }

  fn add(vault: &mut Vault, title: &str) {
    vault.add_entry(vault.root_group(), &EntryData { title: title.into(), ..EntryData::default() }).unwrap();
  }

  fn titles(vault: &Vault) -> Vec<String> {
    let mut titles: Vec<String> = vault.search("").into_iter().map(|entry| entry.title).collect();
    titles.extend(vault.entries_in(vault.root_group()).unwrap().into_iter().map(|entry| entry.title));
    titles.sort();
    titles
  }

  fn at(second: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 4, 21, 30, second).unwrap()
  }

  /// A store holding `vault` as `Personal.kdbx`, and the link to it.
  fn published(vault: &Vault) -> (FakeStore, SyncLink) {
    let store = FakeStore::default();
    let file = store.put("Personal.kdbx", vault.to_bytes().unwrap(), false);
    (store, SyncLink { file_id: file.id, version: file.version })
  }

  #[tokio::test]
  async fn nothing_to_do_when_neither_side_changed() {
    let local = vault();
    let (store, link) = published(&local);
    let outcome = sync_vault(&store, local, &link, false, at(0)).await.unwrap();
    assert!(!outcome.uploaded && outcome.remote.is_none());
    assert!(store.backups().is_empty());
  }

  #[tokio::test]
  async fn local_changes_are_uploaded_after_a_backup() {
    let mut local = vault();
    let (store, link) = published(&local);
    add(&mut local, "New");

    let outcome = sync_vault(&store, local.clone(), &link, true, at(5)).await.unwrap();
    assert!(outcome.uploaded && outcome.remote.is_none());
    assert_eq!(outcome.file.version.as_deref(), Some("2"));
    assert_eq!(store.backups(), ["Personal 2026-11-04 21-30-05.kdbx"]);
    let uploaded = local.open_sibling(&store.bytes(&link.file_id)).unwrap();
    assert_eq!(titles(&uploaded), ["New"]);
  }

  #[tokio::test]
  async fn remote_changes_come_back_without_an_upload() {
    let local = vault();
    let (store, link) = published(&local);
    let mut other_pc = local.clone();
    add(&mut other_pc, "From the other PC");
    store.update(&link.file_id, other_pc.to_bytes().unwrap()).await.unwrap();

    let outcome = sync_vault(&store, local, &link, false, at(0)).await.unwrap();
    assert!(!outcome.uploaded);
    assert_eq!(titles(&outcome.remote.unwrap()), ["From the other PC"]);
    assert!(store.backups().is_empty());
  }

  #[tokio::test]
  async fn both_sides_changed_uploads_the_merge() {
    let mut local = vault();
    let (store, link) = published(&local);
    let mut other_pc = local.clone();
    add(&mut other_pc, "Remote");
    store.update(&link.file_id, other_pc.to_bytes().unwrap()).await.unwrap();
    add(&mut local, "Local");

    let outcome = sync_vault(&store, local.clone(), &link, true, at(0)).await.unwrap();
    assert!(outcome.uploaded);
    let uploaded = local.open_sibling(&store.bytes(&link.file_id)).unwrap();
    assert_eq!(titles(&uploaded), ["Local", "Remote"]);
    assert_eq!(store.backups().len(), 1);
  }

  #[tokio::test]
  async fn only_the_latest_backups_of_this_vault_are_kept() {
    let mut local = vault();
    let (store, mut link) = published(&local);
    store.put("Personal 2 2026-01-01 00-00-00.kdbx", Vec::new(), true);
    for second in 0..12 {
      add(&mut local, &format!("e{second}"));
      let outcome = sync_vault(&store, local.clone(), &link, true, at(second)).await.unwrap();
      link.version = outcome.file.version;
    }
    let backups = store.backups();
    let ours: Vec<&String> = backups.iter().filter(|name| name.starts_with("Personal 2026")).collect();
    assert_eq!(ours.len(), MAX_BACKUPS);
    assert!(!backups.contains(&"Personal 2026-11-04 21-30-00.kdbx".to_owned()), "the oldest is gone");
    assert!(backups.contains(&"Personal 2 2026-01-01 00-00-00.kdbx".to_owned()), "another vault's backup stays");
  }

  #[tokio::test]
  async fn a_remote_copy_with_another_password_is_an_error() {
    let local = vault();
    let (store, link) = published(&local);
    let stranger = Vault::create("Personal", SecretString::from("other"), KdfParams::insecure_fast());
    store.update(&link.file_id, stranger.to_bytes().unwrap()).await.unwrap();

    let error = sync_vault(&store, local, &link, true, at(0)).await.err().unwrap();
    assert!(matches!(error, SyncError::Vault(CoreError::InvalidKey)));
    assert!(store.backups().is_empty(), "nothing was overwritten");
  }

  #[tokio::test]
  async fn a_deleted_remote_file_is_reported() {
    let local = vault();
    let (store, link) = published(&local);
    store.delete(&link.file_id).await.unwrap();
    let error = sync_vault(&store, local, &link, true, at(0)).await.err().unwrap();
    assert!(matches!(error, SyncError::RemoteMissing));
  }

  #[test]
  fn backup_names_round_trip() {
    assert_eq!(backup_stem("Personal.KDBX"), "Personal");
    let name = backup_name("Personal", at(7));
    assert_eq!(backup_time("Personal", &name), Some(at(7).naive_utc()));
    assert_eq!(backup_time("Personal", "Personal 2 2026-11-04 21-30-07.kdbx"), None);
  }
}
