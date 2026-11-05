use keepsave_core::{
    CoreError, CustomField, EntryData, ExposeSecret, KdfParams, SecretString, Vault,
};

const PASSWORD: &str = "correct horse battery staple";

fn new_vault() -> Vault {
    Vault::create(
        "Personal",
        SecretString::from(PASSWORD),
        KdfParams::insecure_fast(),
    )
}

fn sample_entry(title: &str) -> EntryData {
    EntryData {
        title: title.into(),
        username: "jdoe".into(),
        password: SecretString::from("hunter2"),
        url: "https://example.com".into(),
        notes: "some notes".into(),
        tags: vec!["work".into()],
        custom_fields: vec![
            CustomField {
                key: "PIN".into(),
                value: SecretString::from("1234"),
                protected: true,
            },
            CustomField {
                key: "Recovery email".into(),
                value: SecretString::from("me@example.com"),
                protected: false,
            },
        ],
    }
}

fn assert_same(a: &EntryData, b: &EntryData) {
    assert_eq!(a.title, b.title);
    assert_eq!(a.username, b.username);
    assert_eq!(a.password.expose_secret(), b.password.expose_secret());
    assert_eq!(a.url, b.url);
    assert_eq!(a.notes, b.notes);
    assert_eq!(a.tags, b.tags);
    assert_eq!(a.custom_fields.len(), b.custom_fields.len());
    for (x, y) in a.custom_fields.iter().zip(&b.custom_fields) {
        assert_eq!(x.key, y.key);
        assert_eq!(x.value.expose_secret(), y.value.expose_secret());
        assert_eq!(x.protected, y.protected);
    }
}

#[test]
fn save_and_reopen_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("personal.kdbx");

    let mut vault = new_vault();
    let group = vault.add_group(vault.root_group(), "Email").unwrap();
    let data = sample_entry("Gmail");
    let id = vault.add_entry(group, &data).unwrap();
    vault.save(&path).unwrap();

    assert!(!dir.path().join("personal.kdbx.tmp").exists());

    let reopened = Vault::open(&path, SecretString::from(PASSWORD)).unwrap();
    assert_eq!(reopened.name(), "Personal");
    assert_eq!(reopened.kdf_params(), Some(KdfParams::insecure_fast()));
    let groups = reopened.child_groups(reopened.root_group()).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "Email");
    assert_eq!(groups[0].entry_count, 1);
    assert_same(&reopened.entry(id).unwrap(), &data);
}

#[test]
fn wrong_password_is_rejected() {
    let bytes = new_vault().to_bytes().unwrap();
    let result = Vault::from_bytes(&bytes, SecretString::from("nope"));
    assert!(matches!(result, Err(CoreError::InvalidKey)));
}

#[test]
fn updates_keep_history_and_can_be_restored() {
    let mut vault = new_vault();
    let original = sample_entry("Bank");
    let id = vault.add_entry(vault.root_group(), &original).unwrap();

    // Saving identical data is a no-op.
    vault.update_entry(id, &original).unwrap();
    assert!(vault.entry_history(id).unwrap().is_empty());

    let mut changed = original.clone();
    changed.password = SecretString::from("new-password");
    changed.custom_fields.retain(|field| field.key != "PIN");
    vault.update_entry(id, &changed).unwrap();

    let history = vault.entry_history(id).unwrap();
    assert_eq!(history.len(), 1);
    assert_same(&history[0].data, &original);
    assert_same(&vault.entry(id).unwrap(), &changed);

    vault.restore_history(id, 0).unwrap();
    assert_same(&vault.entry(id).unwrap(), &original);
    assert_eq!(vault.entry_history(id).unwrap().len(), 2);
}

#[test]
fn history_is_capped_at_ten_versions() {
    let mut vault = new_vault();
    let mut data = sample_entry("Busy");
    let id = vault.add_entry(vault.root_group(), &data).unwrap();
    for i in 0..15 {
        data.notes = format!("edit {i}");
        vault.update_entry(id, &data).unwrap();
    }
    let history = vault.entry_history(id).unwrap();
    assert_eq!(history.len(), 10);
    assert_eq!(history[0].data.notes, "edit 13");
}

#[test]
fn deleting_goes_through_the_recycle_bin() {
    let mut vault = new_vault();
    let id = vault
        .add_entry(vault.root_group(), &sample_entry("Old"))
        .unwrap();
    let uuid = id.uuid();

    vault.delete_entry(id).unwrap();
    let bin = vault.recycle_bin().expect("bin created on first delete");
    assert_eq!(vault.entries_in(bin).unwrap().len(), 1);
    assert!(
        vault.search("old").is_empty(),
        "bin is excluded from search"
    );

    vault.delete_entry(id).unwrap();
    assert!(vault.entry(id).is_none());
    assert!(vault.database().deleted_objects.contains_key(&uuid));
}

#[test]
fn summaries_and_recycle_bin_membership() {
    let mut vault = new_vault();
    let root = vault.root_group();
    let id = vault.add_entry(root, &sample_entry("Mail")).unwrap();

    let summary = vault.entry_summary(id).unwrap();
    assert_eq!(summary.title, "Mail");
    assert_eq!(summary.group, root);
    assert!(!vault.is_in_recycle_bin(root));

    vault.delete_entry(id).unwrap();
    let bin = vault.recycle_bin().unwrap();
    assert_eq!(vault.entry_summary(id).unwrap().group, bin);
    assert!(vault.is_in_recycle_bin(bin));
}

#[test]
fn entries_under_includes_nested_groups() {
    let mut vault = new_vault();
    let root = vault.root_group();
    let work = vault.add_group(root, "Work").unwrap();
    let servers = vault.add_group(work, "Servers").unwrap();
    vault.add_entry(work, &sample_entry("VPN")).unwrap();
    vault.add_entry(servers, &sample_entry("Bastion")).unwrap();
    vault.add_entry(root, &sample_entry("Mail")).unwrap();

    let titles: Vec<String> = vault
        .entries_under(work)
        .unwrap()
        .into_iter()
        .map(|entry| entry.title)
        .collect();
    assert_eq!(titles, ["Bastion", "VPN"]);
}

#[test]
fn a_clone_saves_independently() {
    let mut vault = new_vault();
    let root = vault.root_group();
    vault.add_entry(root, &sample_entry("Before")).unwrap();
    let snapshot = vault.clone();
    vault.add_entry(root, &sample_entry("After")).unwrap();

    let reopened =
        Vault::from_bytes(&snapshot.to_bytes().unwrap(), SecretString::from(PASSWORD)).unwrap();
    let titles: Vec<String> = reopened
        .entries_in(reopened.root_group())
        .unwrap()
        .into_iter()
        .map(|entry| entry.title)
        .collect();
    assert_eq!(titles, ["Before"]);
}

#[test]
fn deleting_groups_and_emptying_the_bin() {
    let mut vault = new_vault();
    let root = vault.root_group();
    assert!(matches!(
        vault.delete_group(root),
        Err(CoreError::RootGroup)
    ));

    let group = vault.add_group(root, "Games").unwrap();
    let entry = vault.add_entry(group, &sample_entry("Steam")).unwrap();
    vault.delete_group(group).unwrap();

    let bin = vault.recycle_bin().unwrap();
    let in_bin = vault.child_groups(bin).unwrap();
    assert_eq!(in_bin.len(), 1);
    assert_eq!(in_bin[0].name, "Games");

    vault.delete_group(bin).unwrap();
    assert!(vault.child_groups(bin).unwrap().is_empty());
    assert!(vault.entry(entry).is_none());
    assert!(
        vault.recycle_bin().is_some(),
        "emptying keeps the bin itself"
    );
}

#[test]
fn search_is_case_insensitive_across_fields() {
    let mut vault = new_vault();
    let root = vault.root_group();
    vault.add_entry(root, &sample_entry("GitHub")).unwrap();
    let mut other = sample_entry("Netflix");
    other.username = "family".into();
    other.url = "https://netflix.com".into();
    other.notes = String::new();
    other.tags.clear();
    vault.add_entry(root, &other).unwrap();

    assert_eq!(vault.search("github").len(), 1);
    assert_eq!(vault.search("FAMILY")[0].title, "Netflix");
    assert_eq!(vault.search("work").len(), 1, "matches tags");
    assert_eq!(vault.search("example.com").len(), 1);
    assert!(vault.search("   ").is_empty());
}

#[test]
fn changing_the_password_applies_on_next_save() {
    let mut vault = new_vault();
    vault.change_password(SecretString::from("new master"));
    let bytes = vault.to_bytes().unwrap();
    assert!(Vault::from_bytes(&bytes, SecretString::from(PASSWORD)).is_err());
    assert!(Vault::from_bytes(&bytes, SecretString::from("new master")).is_ok());
}

/// Slow: run with `cargo test --release -p keepsave-core -- --ignored`.
#[test]
#[ignore]
fn calibrated_unlock_takes_about_one_second() {
    let params = KdfParams::calibrate(KdfParams::DEFAULT_UNLOCK_TIME).unwrap();
    let bytes = Vault::create("Timing", SecretString::from(PASSWORD), params)
        .to_bytes()
        .unwrap();

    let started = std::time::Instant::now();
    Vault::from_bytes(&bytes, SecretString::from(PASSWORD)).unwrap();
    let elapsed = started.elapsed();
    println!("{params:?} -> unlock in {elapsed:?}");
    assert!(elapsed.as_secs_f64() > 0.4 && elapsed.as_secs_f64() < 2.5);
}
