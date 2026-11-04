use chrono::NaiveDateTime;
use keepass::db::{Entry, EntryId, EntryRef, GroupId, Value, fields};
use secrecy::{ExposeSecret, SecretString};

/// Editable contents of an entry, detached from the database.
#[derive(Debug, Clone, Default)]
pub struct EntryData {
    pub title: String,
    pub username: String,
    pub password: SecretString,
    pub url: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Clone)]
pub struct CustomField {
    pub key: String,
    pub value: SecretString,
    pub protected: bool,
}

/// What a list row needs. Holds no secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntrySummary {
    pub id: EntryId,
    pub group: GroupId,
    pub title: String,
    pub username: String,
    pub url: String,
    pub modified: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupSummary {
    pub id: GroupId,
    pub name: String,
    pub entry_count: usize,
    pub is_recycle_bin: bool,
}

/// A past version of an entry, newest first in [`crate::Vault::entry_history`].
#[derive(Debug, Clone)]
pub struct HistoryItem {
    pub modified: Option<NaiveDateTime>,
    pub data: EntryData,
}

/// Fields KeepSave edits itself; everything else on an entry is a custom field,
/// except `otp`, which is preserved untouched.
fn is_reserved(key: &str) -> bool {
    fields::KNOWN_FIELDS.contains(&key) || key == fields::OTP
}

impl EntryData {
    pub(crate) fn from_entry(entry: &Entry) -> Self {
        let text = |key| entry.get(key).unwrap_or_default().to_owned();
        let mut custom_fields: Vec<CustomField> = entry
            .fields
            .iter()
            .filter(|(key, _)| !is_reserved(key))
            .map(|(key, value)| CustomField {
                key: key.clone(),
                value: SecretString::from(value.get().as_str()),
                protected: value.is_protected(),
            })
            .collect();
        custom_fields.sort_by(|a, b| a.key.cmp(&b.key));

        Self {
            title: text(fields::TITLE),
            username: text(fields::USERNAME),
            password: SecretString::from(entry.get(fields::PASSWORD).unwrap_or_default()),
            url: text(fields::URL),
            notes: text(fields::NOTES),
            tags: entry.tags.clone(),
            custom_fields,
        }
    }

    /// Writes this data into `entry`, removing custom fields that are gone.
    pub(crate) fn apply_to(&self, entry: &mut Entry) {
        entry.set_unprotected(fields::TITLE, self.title.as_str());
        entry.set_unprotected(fields::USERNAME, self.username.as_str());
        entry.set_protected(fields::PASSWORD, self.password.expose_secret());
        entry.set_unprotected(fields::URL, self.url.as_str());
        entry.set_unprotected(fields::NOTES, self.notes.as_str());
        entry.tags = self.tags.clone();

        entry.fields.retain(|key, _| {
            is_reserved(key) || self.custom_fields.iter().any(|field| &field.key == key)
        });
        for field in &self.custom_fields {
            let value = field.value.expose_secret();
            let value = if field.protected {
                Value::protected(value)
            } else {
                Value::unprotected(value)
            };
            entry.set(field.key.as_str(), value);
        }
    }

    /// True when applying `self` to `entry` would change nothing.
    pub(crate) fn matches(&self, entry: &Entry) -> bool {
        let current = Self::from_entry(entry);
        let same_fields = self.custom_fields.len() == current.custom_fields.len()
            && self.custom_fields.iter().all(|field| {
                current.custom_fields.iter().any(|other| {
                    other.key == field.key
                        && other.protected == field.protected
                        && other.value.expose_secret() == field.value.expose_secret()
                })
            });

        same_fields
            && self.title == current.title
            && self.username == current.username
            && self.password.expose_secret() == current.password.expose_secret()
            && self.url == current.url
            && self.notes == current.notes
            && self.tags == current.tags
    }
}

impl EntrySummary {
    pub(crate) fn from_ref(entry: &EntryRef<'_>) -> Self {
        Self {
            id: entry.id(),
            group: entry.parent().id(),
            title: entry.get_title().unwrap_or_default().to_owned(),
            username: entry.get_username().unwrap_or_default().to_owned(),
            url: entry.get_url().unwrap_or_default().to_owned(),
            modified: entry.times.last_modification,
        }
    }
}
