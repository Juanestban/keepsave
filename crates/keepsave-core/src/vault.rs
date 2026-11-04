use std::{fs, io::Write, path::Path};

use keepass::{
    Database, DatabaseKey,
    config::DatabaseConfig,
    db::{EntryId, GroupId, History, Times},
};
use secrecy::{ExposeSecret, SecretString};

use crate::{CoreError, EntryData, EntrySummary, GroupSummary, HistoryItem, KdfParams, Result};

const GENERATOR: &str = "KeepSave";
const RECYCLE_BIN_NAME: &str = "Recycle Bin";
/// KeePass built-in icon index for the recycle bin.
const RECYCLE_BIN_ICON: usize = 43;
const DEFAULT_HISTORY_MAX_ITEMS: isize = 10;
const DEFAULT_HISTORY_MAX_SIZE: isize = 6 * 1024 * 1024;

/// An unlocked KDBX4 vault together with the master password needed to save it.
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
        Ok(parent
            .groups()
            .filter_map(|child| self.group(child.id()))
            .collect())
    }

    pub fn add_group(&mut self, parent: GroupId, name: &str) -> Result<GroupId> {
        let mut parent = self.db.group_mut(parent).ok_or(CoreError::GroupNotFound)?;
        Ok(parent
            .add_group()
            .edit(|group| group.name = name.into())
            .id())
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
                group
                    .track_changes()
                    .remove()
                    .map_err(|_| CoreError::RootGroup)?;
            }
            return Ok(());
        }

        let bin = self.ensure_recycle_bin();
        let mut group = self.db.group_mut(id).ok_or(CoreError::GroupNotFound)?;
        group
            .track_changes()
            .move_to(bin)
            .map_err(|_| CoreError::GroupNotFound)
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

    fn is_in_recycle_bin(&self, group: GroupId) -> bool {
        let Some(bin) = self.recycle_bin() else {
            return false;
        };
        let mut current = Some(group);
        while let Some(id) = current {
            if id == bin {
                return true;
            }
            current = self
                .db
                .group(id)
                .and_then(|group| group.parent().map(|parent| parent.id()));
        }
        false
    }

    // ---- entries ----

    pub fn entries_in(&self, group: GroupId) -> Result<Vec<EntrySummary>> {
        let group = self.db.group(group).ok_or(CoreError::GroupNotFound)?;
        let mut entries: Vec<EntrySummary> = group
            .entries()
            .map(|entry| EntrySummary::from_ref(&entry))
            .collect();
        entries.sort_by_key(|entry| entry.title.to_lowercase());
        Ok(entries)
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
        let group = self
            .db
            .entry(id)
            .ok_or(CoreError::EntryNotFound)?
            .parent()
            .id();

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
        entry
            .move_to(target)
            .map_err(|_| CoreError::GroupNotFound)?;
        entry.times.location_changed = Some(Times::now());
        Ok(())
    }

    pub fn entry_history(&self, id: EntryId) -> Result<Vec<HistoryItem>> {
        let entry = self.db.entry(id).ok_or(CoreError::EntryNotFound)?;
        Ok(entry
            .history
            .iter()
            .flat_map(|history| history.get_entries())
            .map(|version| HistoryItem {
                modified: version.times.last_modification,
                data: EntryData::from_entry(version),
            })
            .collect())
    }

    /// Makes history version `index` the current one; the current version is
    /// kept in history, so nothing is lost.
    pub fn restore_history(&mut self, id: EntryId, index: usize) -> Result<()> {
        let data = self
            .entry_history(id)?
            .into_iter()
            .nth(index)
            .ok_or(CoreError::HistoryNotFound)?
            .data;
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
                [&data.title, &data.username, &data.url, &data.notes]
                    .into_iter()
                    .chain(data.tags.iter())
                    .any(|text| text.to_lowercase().contains(&query))
            })
            .map(|entry| EntrySummary::from_ref(&entry))
            .collect();
        results.sort_by_key(|entry| entry.title.to_lowercase());
        results
    }

    /// `None` means unlimited, matching KeePass' negative value.
    fn history_max_items(&self) -> Option<usize> {
        let max = self
            .db
            .meta
            .history_max_items
            .unwrap_or(DEFAULT_HISTORY_MAX_ITEMS);
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
