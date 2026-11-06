use std::{fs, io::Write, path::Path};

use keepass::{
  Database, DatabaseKey,
  config::DatabaseConfig,
  db::{
    EntryId, GroupId, History, Times,
    merge::{MergeEventType, MergeLog},
  },
};
use secrecy::{ExposeSecret, SecretString};

use crate::{CoreError, EntryData, EntrySummary, GroupSummary, HistoryItem, KdfParams, Result};

const GENERATOR: &str = "KeepSave";
const RECYCLE_BIN_NAME: &str = "Recycle Bin";
/// KeePass built-in icon index for the recycle bin.
const RECYCLE_BIN_ICON: usize = 43;
const DEFAULT_HISTORY_MAX_ITEMS: isize = 10;
const DEFAULT_HISTORY_MAX_SIZE: isize = 6 * 1024 * 1024;

/// What a merge changed in the vault it was applied to.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MergeSummary {
  pub created: usize,
  pub updated: usize,
  pub moved: usize,
  pub deleted: usize,
}

impl MergeSummary {
  fn from_log(log: &MergeLog) -> Self {
    let mut summary = Self::default();
    for event in &log.events {
      match event.event_type {
        MergeEventType::Created => summary.created += 1,
        MergeEventType::Updated => summary.updated += 1,
        MergeEventType::LocationUpdated => summary.moved += 1,
        MergeEventType::Deleted => summary.deleted += 1,
        _ => summary.updated += 1,
      }
    }
    summary
  }

  pub fn total(&self) -> usize {
    self.created + self.updated + self.moved + self.deleted
  }

  pub fn is_empty(&self) -> bool {
    self.total() == 0
  }
}

/// An unlocked KDBX4 vault together with the master password needed to save it.
///
/// Cloning is how the app hands a snapshot to a background save.
#[derive(Clone)]
pub struct Vault {
  db: Database,
  password: SecretString,
}

impl Vault {
  pub fn create(name: &str, password: SecretString, kdf: KdfParams) -> Self {
    let mut config = DatabaseConfig::default();
    config.kdf_config = kdf.to_config();

    let mut db = Database::with_config(config);
    db.meta.generator = Some(GENERATOR.into());
    db.meta.database_name = Some(name.into());
    db.meta.recyclebin_enabled = Some(true);
    db.meta.history_max_items = Some(DEFAULT_HISTORY_MAX_ITEMS);
    db.meta.history_max_size = Some(DEFAULT_HISTORY_MAX_SIZE);
    db.meta.master_key_changed = Some(Times::now());
    db.root_mut().name = name.into();

    Self { db, password }
  }

  pub fn open(path: &Path, password: SecretString) -> Result<Self> {
    Self::from_bytes(&fs::read(path)?, password)
  }

  pub fn from_bytes(bytes: &[u8], password: SecretString) -> Result<Self> {
    let db = Database::parse(bytes, key_for(&password))?;
    Ok(Self { db, password })
  }

  /// Decrypts another copy of this vault, e.g. the one on Drive, with this
  /// vault's master password. A copy with another password fails with
  /// [`CoreError::InvalidKey`].
  pub fn open_sibling(&self, bytes: &[u8]) -> Result<Self> {
    Self::from_bytes(bytes, self.password.clone())
  }

  /// Merges `other` (another copy of the same vault) into this one, KeePass
  /// style: by UUID and modification time, honoring deletions, and keeping
  /// the losing version of an entry in its history.
  pub fn merge_from(&mut self, other: &Vault) -> Result<MergeSummary> {
    let log = self.db.merge(&other.db).map_err(|error| CoreError::Merge(error.to_string()))?;
    // The merge leaves `meta` alone. If the bin was first created on the
    // other side, adopt it, or the next deletion would create a second bin.
    if self.recycle_bin().is_none()
      && let Some(bin) = other.recycle_bin()
      && self.db.group(bin).is_some()
    {
      self.db.meta.recyclebin_enabled = Some(true);
      self.db.meta.recyclebin_uuid = Some(bin.uuid());
      self.db.meta.recyclebin_changed = other.db.meta.recyclebin_changed;
    }
    // The merge combines both histories; keep them within the limit.
    if let Some(max) = self.history_max_items() {
      let ids: Vec<EntryId> = self.db.iter_all_entries().map(|entry| entry.id()).collect();
      for id in ids {
        if let Some(mut entry) = self.db.entry_mut(id)
          && let Some(history) = entry.history.as_mut()
        {
          trim_history(history, max);
        }
      }
    }
    Ok(MergeSummary::from_log(&log))
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    self.db.save(&mut buffer, key_for(&self.password))?;
    Ok(buffer)
  }

  /// Writes to a sibling temp file first and renames it over `path`, so a
  /// crash mid-write never leaves a truncated vault behind.
  pub fn save(&self, path: &Path) -> Result<()> {
    let bytes = self.to_bytes()?;
    let mut tmp_name = path.as_os_str().to_owned();
    tmp_name.push(".tmp");
    let tmp_path = Path::new(&tmp_name);

    let mut file = fs::File::create(tmp_path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    fs::rename(tmp_path, path)?;
    Ok(())
  }

  pub fn change_password(&mut self, password: SecretString) {
    self.password = password;
    self.db.meta.master_key_changed = Some(Times::now());
  }

  /// A copy of the master password, e.g. to seal it for a quick unlock.
  pub fn master_password(&self) -> SecretString {
    self.password.clone()
  }

  pub fn verify_password(&self, candidate: &SecretString) -> bool {
    self.password.expose_secret() == candidate.expose_secret()
  }

  pub fn name(&self) -> &str {
    self.db.meta.database_name.as_deref().unwrap_or_default()
  }

  pub fn kdf_params(&self) -> Option<KdfParams> {
    KdfParams::from_config(&self.db.config.kdf_config)
  }

  /// Low-level access for merge and sync.
  pub fn database(&self) -> &Database {
    &self.db
  }

  // ---- groups ----

  pub fn root_group(&self) -> GroupId {
    self.db.root().id()
  }

  pub fn recycle_bin(&self) -> Option<GroupId> {
    self.db.recycle_bin().map(|bin| bin.id())
  }

  pub fn group(&self, id: GroupId) -> Option<GroupSummary> {
    self.db.group(id).map(|group| GroupSummary {
      id,
      name: group.name.clone(),
      entry_count: group.entries().count(),
      is_recycle_bin: Some(id) == self.recycle_bin(),
    })
  }

  pub fn child_groups(&self, parent: GroupId) -> Result<Vec<GroupSummary>> {
    let parent = self.db.group(parent).ok_or(CoreError::GroupNotFound)?;
    Ok(parent.groups().filter_map(|child| self.group(child.id())).collect())
  }

  pub fn add_group(&mut self, parent: GroupId, name: &str) -> Result<GroupId> {
    let mut parent = self.db.group_mut(parent).ok_or(CoreError::GroupNotFound)?;
    Ok(parent.add_group().edit(|group| group.name = name.into()).id())
  }

  pub fn rename_group(&mut self, id: GroupId, name: &str) -> Result<()> {
    let mut group = self.db.group_mut(id).ok_or(CoreError::GroupNotFound)?;
    group.name = name.into();
    group.times.last_modification = Some(Times::now());
    Ok(())
  }

  /// Moves the group to the recycle bin, or deletes it for good if it is
  /// already there. Deleting the recycle bin itself empties it.
  pub fn delete_group(&mut self, id: GroupId) -> Result<()> {
    if id == self.root_group() {
      return Err(CoreError::RootGroup);
    }
    if self.db.group(id).is_none() {
      return Err(CoreError::GroupNotFound);
    }
    if Some(id) == self.recycle_bin() {
      self.empty_recycle_bin();
      return Ok(());
    }
    if self.is_in_recycle_bin(id) {
      if let Some(mut group) = self.db.group_mut(id) {
        group.track_changes().remove().map_err(|_| CoreError::RootGroup)?;
      }
      return Ok(());
    }

    let bin = self.ensure_recycle_bin();
    let mut group = self.db.group_mut(id).ok_or(CoreError::GroupNotFound)?;
    group.track_changes().move_to(bin).map_err(|_| CoreError::GroupNotFound)
  }

  pub fn empty_recycle_bin(&mut self) {
    let Some(bin) = self.db.recycle_bin() else {
      return;
    };
    let groups: Vec<GroupId> = bin.groups().map(|group| group.id()).collect();
    let entries: Vec<EntryId> = bin.entries().map(|entry| entry.id()).collect();

    for id in entries {
      if let Some(mut entry) = self.db.entry_mut(id) {
        entry.track_changes().remove();
      }
    }
    for id in groups {
      if let Some(mut group) = self.db.group_mut(id) {
        // Only the root group refuses removal, and it is never in the bin.
        let _ = group.track_changes().remove();
      }
    }
  }

  fn ensure_recycle_bin(&mut self) -> GroupId {
    if let Some(bin) = self.recycle_bin() {
      return bin;
    }
    let id = self
      .db
      .root_mut()
      .add_group()
      .edit(|group| {
        group.name = RECYCLE_BIN_NAME.into();
        group.enable_autotype = Some(false);
        group.enable_searching = Some(false);
        group.set_icon_builtin(RECYCLE_BIN_ICON);
      })
      .id();
    self.db.meta.recyclebin_enabled = Some(true);
    self.db.meta.recyclebin_uuid = Some(id.uuid());
    self.db.meta.recyclebin_changed = Some(Times::now());
    id
  }

  /// True for the recycle bin and every group inside it.
  pub fn is_in_recycle_bin(&self, group: GroupId) -> bool {
    let Some(bin) = self.recycle_bin() else {
      return false;
    };
    let mut current = Some(group);
    while let Some(id) = current {
      if id == bin {
        return true;
      }
      current = self.db.group(id).and_then(|group| group.parent().map(|parent| parent.id()));
    }
    false
  }

  // ---- entries ----

  pub fn entries_in(&self, group: GroupId) -> Result<Vec<EntrySummary>> {
    let group = self.db.group(group).ok_or(CoreError::GroupNotFound)?;
    let mut entries: Vec<EntrySummary> = group.entries().map(|entry| EntrySummary::from_ref(&entry)).collect();
    entries.sort_by_key(|entry| entry.title.to_lowercase());
    Ok(entries)
  }

  /// Entries in `group` and in every group below it, sorted by title.
  pub fn entries_under(&self, group: GroupId) -> Result<Vec<EntrySummary>> {
    self.db.group(group).ok_or(CoreError::GroupNotFound)?;
    let mut entries = Vec::new();
    let mut pending = vec![group];
    while let Some(id) = pending.pop() {
      let Some(current) = self.db.group(id) else {
        continue;
      };
      entries.extend(current.entries().map(|entry| EntrySummary::from_ref(&entry)));
      pending.extend(current.groups().map(|child| child.id()));
    }
    entries.sort_by_key(|entry| entry.title.to_lowercase());
    Ok(entries)
  }

  pub fn entry_summary(&self, id: EntryId) -> Option<EntrySummary> {
    self.db.entry(id).map(|entry| EntrySummary::from_ref(&entry))
  }

  pub fn entry(&self, id: EntryId) -> Option<EntryData> {
    self.db.entry(id).map(|entry| EntryData::from_entry(&entry))
  }

  pub fn add_entry(&mut self, group: GroupId, data: &EntryData) -> Result<EntryId> {
    let mut group = self.db.group_mut(group).ok_or(CoreError::GroupNotFound)?;
    Ok(group.add_entry().edit(|entry| data.apply_to(entry)).id())
  }

  /// Saves `data` over the entry, pushing the previous version into its
  /// history. Does nothing if `data` matches what is stored.
  pub fn update_entry(&mut self, id: EntryId, data: &EntryData) -> Result<()> {
    let max_items = self.history_max_items();
    let mut entry = self.db.entry_mut(id).ok_or(CoreError::EntryNotFound)?;
    if data.matches(&entry) {
      return Ok(());
    }
    {
      let mut tracked = entry.track_changes();
      data.apply_to(&mut tracked);
      tracked.times.last_modification = Some(Times::now());
    }
    if let (Some(max), Some(history)) = (max_items, entry.history.as_mut()) {
      trim_history(history, max);
    }
    self.db.remove_unused_attachments();
    Ok(())
  }

  /// Moves the entry to the recycle bin, or deletes it for good if it is
  /// already there.
  pub fn delete_entry(&mut self, id: EntryId) -> Result<()> {
    let group = self.db.entry(id).ok_or(CoreError::EntryNotFound)?.parent().id();

    if self.is_in_recycle_bin(group) {
      if let Some(mut entry) = self.db.entry_mut(id) {
        entry.track_changes().remove();
      }
      return Ok(());
    }

    let bin = self.ensure_recycle_bin();
    self.move_entry(id, bin)
  }

  pub fn move_entry(&mut self, id: EntryId, target: GroupId) -> Result<()> {
    let mut entry = self.db.entry_mut(id).ok_or(CoreError::EntryNotFound)?;
    entry.move_to(target).map_err(|_| CoreError::GroupNotFound)?;
    entry.times.location_changed = Some(Times::now());
    Ok(())
  }

  pub fn entry_history(&self, id: EntryId) -> Result<Vec<HistoryItem>> {
    let entry = self.db.entry(id).ok_or(CoreError::EntryNotFound)?;
    Ok(
      entry
        .history
        .iter()
        .flat_map(|history| history.get_entries())
        .map(|version| HistoryItem { modified: version.times.last_modification, data: EntryData::from_entry(version) })
        .collect(),
    )
  }

  /// Makes history version `index` the current one; the current version is
  /// kept in history, so nothing is lost.
  pub fn restore_history(&mut self, id: EntryId, index: usize) -> Result<()> {
    let data = self.entry_history(id)?.into_iter().nth(index).ok_or(CoreError::HistoryNotFound)?.data;
    self.update_entry(id, &data)
  }

  /// Case-insensitive search over title, username, URL, notes and tags.
  /// Entries in the recycle bin are skipped.
  pub fn search(&self, query: &str) -> Vec<EntrySummary> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
      return Vec::new();
    }
    let mut results: Vec<EntrySummary> = self
      .db
      .iter_all_entries()
      .filter(|entry| !self.is_in_recycle_bin(entry.parent().id()))
      .filter(|entry| {
        let data = EntryData::from_entry(entry);
        [&data.title, &data.username, &data.url, &data.notes].into_iter().chain(data.tags.iter()).any(|text| text.to_lowercase().contains(&query))
      })
      .map(|entry| EntrySummary::from_ref(&entry))
      .collect();
    results.sort_by_key(|entry| entry.title.to_lowercase());
    results
  }

  /// `None` means unlimited, matching KeePass' negative value.
  fn history_max_items(&self) -> Option<usize> {
    let max = self.db.meta.history_max_items.unwrap_or(DEFAULT_HISTORY_MAX_ITEMS);
    usize::try_from(max).ok()
  }
}

fn key_for(password: &SecretString) -> DatabaseKey {
  DatabaseKey::new().with_password(password.expose_secret())
}

fn trim_history(history: &mut History, max: usize) {
  if history.get_entries().len() <= max {
    return;
  }
  let mut trimmed = History::default();
  // `add_entry` inserts at the front, so add the newest last.
  for version in history.get_entries().iter().take(max).rev() {
    trimmed.add_entry(version.clone());
  }
  *history = trimmed;
}

#[cfg(test)]
mod merge_tests {
  use chrono::{Duration, NaiveDateTime};

  use super::*;

  fn base() -> Vault {
    Vault::create("Shared", SecretString::from("pw"), KdfParams::insecure_fast())
  }

  fn entry(title: &str) -> EntryData {
    EntryData { title: title.into(), password: SecretString::from("secret"), ..EntryData::default() }
  }

  /// Timestamps have one-second resolution, so tests set them explicitly.
  fn at(seconds: i64) -> NaiveDateTime {
    Times::now() + Duration::seconds(seconds)
  }

  fn set_modified(vault: &mut Vault, id: EntryId, time: NaiveDateTime) {
    vault.db.entry_mut(id).unwrap().times.last_modification = Some(time);
  }

  fn titles(vault: &Vault) -> Vec<String> {
    let mut titles: Vec<String> = vault.db.iter_all_entries().map(|entry| entry.get_title().unwrap_or_default().to_owned()).collect();
    titles.sort();
    titles
  }

  #[test]
  fn entries_added_on_both_sides_are_kept() {
    let base = base();
    let (mut local, mut remote) = (base.clone(), base.clone());
    local.add_entry(local.root_group(), &entry("Local")).unwrap();
    remote.add_entry(remote.root_group(), &entry("Remote")).unwrap();

    let summary = local.merge_from(&remote).unwrap();
    assert_eq!(summary.created, 1);
    assert_eq!(titles(&local), ["Local", "Remote"]);
  }

  #[test]
  fn the_newer_edit_wins_and_the_other_goes_to_history() {
    let mut base = base();
    let id = base.add_entry(base.root_group(), &entry("Mail")).unwrap();
    set_modified(&mut base, id, at(-100));
    let (mut local, mut remote) = (base.clone(), base.clone());

    local.update_entry(id, &EntryData { title: "Mail (local)".into(), ..entry("") }).unwrap();
    set_modified(&mut local, id, at(-50));
    remote.update_entry(id, &EntryData { title: "Mail (remote)".into(), ..entry("") }).unwrap();
    set_modified(&mut remote, id, at(-10));

    local.merge_from(&remote).unwrap();
    assert_eq!(local.entry(id).unwrap().title, "Mail (remote)");
    let history: Vec<String> = local.entry_history(id).unwrap().into_iter().map(|item| item.data.title).collect();
    assert!(history.contains(&"Mail (local)".to_owned()), "{history:?}");

    // The other way round, the local edit stays and the remote one is filed.
    let (mut local, mut remote) = (base.clone(), base.clone());
    local.update_entry(id, &EntryData { title: "Newer".into(), ..entry("") }).unwrap();
    set_modified(&mut local, id, at(-10));
    remote.update_entry(id, &EntryData { title: "Older".into(), ..entry("") }).unwrap();
    set_modified(&mut remote, id, at(-50));
    local.merge_from(&remote).unwrap();
    assert_eq!(local.entry(id).unwrap().title, "Newer");
    let history: Vec<String> = local.entry_history(id).unwrap().into_iter().map(|item| item.data.title).collect();
    assert!(history.contains(&"Older".to_owned()), "{history:?}");
  }

  #[test]
  fn permanent_deletions_propagate() {
    let mut base = base();
    let id = base.add_entry(base.root_group(), &entry("Old")).unwrap();
    set_modified(&mut base, id, at(-100));
    let (mut local, mut remote) = (base.clone(), base.clone());

    remote.delete_entry(id).unwrap(); // to the bin
    remote.delete_entry(id).unwrap(); // for good
    local.merge_from(&remote).unwrap();
    assert!(local.entry(id).is_none());
  }

  #[test]
  fn an_edit_after_a_remote_deletion_keeps_the_entry() {
    let mut base = base();
    let id = base.add_entry(base.root_group(), &entry("Keep")).unwrap();
    set_modified(&mut base, id, at(-100));
    let (mut local, mut remote) = (base.clone(), base.clone());

    remote.delete_entry(id).unwrap();
    remote.delete_entry(id).unwrap();
    for time in remote.db.deleted_objects.values_mut() {
      *time = Some(at(-60));
    }
    local.update_entry(id, &EntryData { title: "Keep (edited)".into(), ..entry("") }).unwrap();
    set_modified(&mut local, id, at(-10));

    local.merge_from(&remote).unwrap();
    assert_eq!(local.entry(id).unwrap().title, "Keep (edited)");
  }

  #[test]
  fn moves_to_the_recycle_bin_propagate() {
    let mut base = base();
    let id = base.add_entry(base.root_group(), &entry("Trash me")).unwrap();
    base.db.entry_mut(id).unwrap().times.location_changed = Some(at(-100));
    let (mut local, mut remote) = (base.clone(), base.clone());

    remote.delete_entry(id).unwrap();
    remote.db.entry_mut(id).unwrap().times.location_changed = Some(at(-10));
    local.merge_from(&remote).unwrap();

    let bin = local.recycle_bin().expect("the bin came over with the merge");
    assert_eq!(local.entry_summary(id).unwrap().group, bin);
  }

  #[test]
  fn merged_history_stays_within_the_limit() {
    let mut base = base();
    let id = base.add_entry(base.root_group(), &entry("v0")).unwrap();
    let (mut local, mut remote) = (base.clone(), base.clone());
    for (side, offset) in [(&mut local, 0), (&mut remote, 1)] {
      for version in 0..8 {
        side.update_entry(id, &EntryData { title: format!("v{offset}-{version}"), ..entry("") }).unwrap();
        set_modified(side, id, at(-200 + version * 10 + offset));
      }
    }
    local.merge_from(&remote).unwrap();
    assert!(local.entry_history(id).unwrap().len() <= 10);
  }

  #[test]
  fn a_copy_with_another_password_is_rejected() {
    let vault = base();
    let other = Vault::create("Shared", SecretString::from("different"), KdfParams::insecure_fast());
    let error = vault.open_sibling(&other.to_bytes().unwrap()).err().unwrap();
    assert!(matches!(error, CoreError::InvalidKey));
  }
}
