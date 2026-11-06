# Graph Report - keepsave  (2026-11-05)

## Corpus Check
- 52 files · ~42,104 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 7 file(s) not represented in the graph (top: (none) 2, .ftl 2, .wxs 1)

## Summary
- 1312 nodes · 3337 edges · 61 communities (46 shown, 15 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 197 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d8c6e0f0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- detail.rs
- sync.rs
- Design Decisions and Docs
- app.rs
- Fonts and Entry Point
- Password Generator
- background.rs
- Core Errors
- i18n.rs
- Background Worker
- MainScreen
- dialogs.rs
- CI Workflow
- Updater
- make_icon.py
- Workspace Crates
- Workflow Rules
- RemoteFile
- Google Setup Guide
- session.rs
- Action
- settings.rs
- CoreError
- screens/mod.rs
- main/generator.rs
- unlock.rs
- widgets.rs
- oauth.rs
- main/mod.rs
- DriveClient
- EntryData
- Nav
- src/vault.rs
- app.rs
- keepsave-sync/src/drive.rs
- Updater
- DriveSetup
- Unlock
- .tick
- SyncError
- ScreenCtx
- Vault
- keepsave-sync/src/update.rs
- CoreError
- CreateVault
- Dialog
- ActionButton<'a>
- Field
- keepsave-app/src/update.rs
- I18n
- Install
- SecretString
- Credentials
- .to_data
- MergeSummary
- Screen
- keepsave-app/src/update.rs

## God Nodes (most connected - your core abstractions)
1. `Action` - 65 edges
2. `ScreenCtx` - 62 edges
3. `Nav` - 61 edges
4. `Vault` - 60 edges
5. `MainScreen` - 58 edges
6. `RemoteFile` - 42 edges
7. `Event` - 40 edges
8. `I18n` - 36 edges
9. `DriveClient` - 36 edges
10. `icon_button()` - 34 edges

## Surprising Connections (you probably didn't know these)
- `Never block the UI thread` --semantically_similar_to--> `Q14 - Cargo workspace architecture (core/sync/app)`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Q20 - Packaging (portable .exe, LTO; later MSI/auto-update)` --semantically_similar_to--> `Post-v1 backlog`  [INFERRED] [semantically similar]
  docs/DESIGN.md → ROADMAP.md
- `Secrets handling convention (secrecy/zeroize, keyring)` --semantically_similar_to--> `Q10 - Memory hygiene and locking (secrecy/zeroize, auto-lock, clipboard clear)`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Build-time credential embedding (KEEPSAVE_GOOGLE_CLIENT_ID/SECRET via .env)` --references--> `M4 - Google Drive connection (todo)`  [INFERRED]
  docs/GOOGLE_SETUP.md → ROADMAP.md
- `Inter font (SIL OFL 1.1)` --conceptually_related_to--> `Q15 - i18n (Fluent es/en, sys-locale) and assets (Inter, Phosphor)`  [INFERRED]
  README.md → docs/DESIGN.md

## Import Cycles
- 1-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/create.rs`
- 1-file cycle: `crates/keepsave-app/src/session.rs -> crates/keepsave-app/src/session.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/update.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/update.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 2-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/sync.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/token_store.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/update.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Cargo workspace crates (core/sync/app)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, backups)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_keepass_style_merge, docs_design_q13_sync_behavior, docs_design_q17_new_pc_flow, docs_design_keepsave_sync [INFERRED 0.85]
- **Security controls (Argon2id, memory hygiene, keyring secrets, drive.file OAuth)** — readme_security_model, docs_design_q9_argon2id_kdf, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_q11_oauth [INFERRED 0.85]

## Communities (61 total, 15 thin omitted)

### Community 0 - "detail.rs"
Cohesion: 0.12
Nodes (33): a_new_draft_is_unchanged_until_typed_into(), an_edit_draft_tracks_changes_against_the_vault(), COLUMN_GAP, copy_button(), custom_field_names_are_validated(), custom_field_ui(), Detail, Edit (+25 more)

### Community 1 - "sync.rs"
Cohesion: 0.19
Nodes (22): a_deleted_remote_file_is_reported(), a_remote_copy_with_another_password_is_an_error(), add(), at(), backup_name(), backup_names_round_trip(), backup_stem(), backup_time() (+14 more)

### Community 2 - "Design Decisions and Docs"
Cohesion: 0.06
Nodes (36): CLAUDE.md (Claude Code guidance), KeepSave password manager, config.toml, KeepSave Design (DESIGN.md), GOOGLE_SETUP.md, keepsave-app crate, keepsave-core crate, keepsave-sync crate (+28 more)

### Community 3 - "app.rs"
Cohesion: 0.20
Nodes (3): ENTRANCE_LIFT, ENTRANCE_TIME, Background

### Community 4 - "Fonts and Entry Point"
Cohesion: 0.08
Nodes (32): ICONS, install(), INTER_REGULAR, INTER_SEMIBOLD, SEMIBOLD, ACCENT, ACCENT_HOVER, apply() (+24 more)

### Community 5 - "Password Generator"
Cohesion: 0.09
Nodes (24): AMBIGUOUS, CharsetOptions, DIGITS, entropy_matches_pool_size(), estimate_entropy_bits(), estimate_grows_with_length_and_classes(), exclude_ambiguous_removes_them(), generate_passphrase() (+16 more)

### Community 6 - "background.rs"
Cohesion: 0.08
Nodes (6): BackgroundCallback, FRAME_INTERVAL, paint_fallback(), Resources, UNFOCUSED_FRAME_INTERVAL, UNIFORMS_SIZE

### Community 7 - "Core Errors"
Cohesion: 0.09
Nodes (11): Pending, SecureClipboard, calibrate_respects_bounds(), config_round_trip(), default_parallelism(), KdfParams, .DEFAULT_MEMORY_BYTES, .DEFAULT_UNLOCK_TIME (+3 more)

### Community 8 - "i18n.rs"
Cohesion: 0.13
Nodes (9): both_languages_have_the_same_keys(), bundle_for(), ENGLISH_FTL, keys(), Language, .ALL, English, Spanish (+1 more)

### Community 9 - "Background Worker"
Cohesion: 0.06
Nodes (25): Stage, Downloading, Failed, Idle, Listing, Picking, Ready, Event (+17 more)

### Community 10 - "MainScreen"
Cohesion: 0.19
Nodes (3): MainScreen, error_message(), OpenVault

### Community 11 - "dialogs.rs"
Cohesion: 0.20
Nodes (15): buttons(), confirm_button(), danger_button(), Outcome, Cancel, Confirm, Open, show() (+7 more)

### Community 12 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 14 - "make_icon.py"
Cohesion: 0.17
Nodes (8): bmp(), ico(), keyhole(), main(), png(), render(), rounded_box(), shade()

### Community 15 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

### Community 17 - "RemoteFile"
Cohesion: 0.17
Nodes (4): RemoteFile, RemoteStore, FakeStore, State

### Community 20 - "session.rs"
Cohesion: 0.08
Nodes (6): CTX, LOCK_REQUESTED, run(), start(), wide(), window_proc()

### Community 21 - "Action"
Cohesion: 0.06
Nodes (35): Action, CancelDrive, CancelEdit, CloseDetail, CloseGenerator, CloseWindow, CommitGroupName, ConnectDrive (+27 more)

### Community 22 - "settings.rs"
Cohesion: 0.06
Nodes (46): Config, drive_links_round_trip_and_replace(), DriveLink, MAX_RECENT_VAULTS, missing_keys_take_their_defaults(), recent_vaults_are_unique_ordered_and_capped(), round_trip_and_missing_file(), icon() (+38 more)

### Community 23 - "CoreError"
Cohesion: 0.28
Nodes (15): a_clone_saves_independently(), assert_same(), changing_the_password_applies_on_next_save(), deleting_goes_through_the_recycle_bin(), deleting_groups_and_emptying_the_bin(), entries_under_includes_nested_groups(), history_is_capped_at_ten_versions(), new_vault() (+7 more)

### Community 24 - "screens/mod.rs"
Cohesion: 0.17
Nodes (11): empty_state(), entry_row(), ROW_HEIGHT, show(), drive_row(), drive_section(), group_row(), ROW_HEIGHT (+3 more)

### Community 25 - "main/generator.rs"
Cohesion: 0.14
Nodes (13): Generator, Mode, Characters, Passphrase, Pick, Copy, Use, show_tool() (+5 more)

### Community 26 - "unlock.rs"
Cohesion: 0.11
Nodes (14): DriveImport, file_row(), status_line(), emblem(), link_pair(), title(), CardAction, Forget (+6 more)

### Community 27 - "widgets.rs"
Cohesion: 0.13
Nodes (12): focus_stroke(), glass_card(), glass_panel(), icon_link(), input_frame(), label_row(), multiline_field(), paint_highlight() (+4 more)

### Community 28 - "oauth.rs"
Cohesion: 0.08
Nodes (36): browser_pages(), Url, access_from(), AccessToken, AUTH_URL, authorization_url(), authorization_url_asks_for_offline_drive_file_access(), authorize() (+28 more)

### Community 29 - "main/mod.rs"
Cohesion: 0.08
Nodes (23): AUTOSAVE_DELAY, browsable_url(), build_lists(), DETAIL_WIDTH, DriveSync, GAP, GroupKind, Bin (+15 more)

### Community 31 - "EntryData"
Cohesion: 0.14
Nodes (7): same_data(), CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem, is_reserved()

### Community 32 - "Nav"
Cohesion: 0.08
Nodes (25): Nav, CancelDrive, ChangeDriveClient, CheckForUpdates, ConnectDrive, Create, DisconnectDrive, DownloadUpdate (+17 more)

### Community 33 - "src/vault.rs"
Cohesion: 0.20
Nodes (17): an_edit_after_a_remote_deletion_keeps_the_entry(), at(), base(), DEFAULT_HISTORY_MAX_ITEMS, DEFAULT_HISTORY_MAX_SIZE, entries_added_on_both_sides_are_kept(), entry(), GENERATOR (+9 more)

### Community 35 - "keepsave-sync/src/drive.rs"
Cohesion: 0.12
Nodes (12): API, ApiError, ApiErrorBody, check(), FILE_FIELDS, FileList, FilePage, FOLDER_MIME (+4 more)

### Community 36 - "Updater"
Cohesion: 0.22
Nodes (9): State, Available, CheckFailed, Checking, Downloading, Failed, Idle, Ready (+1 more)

### Community 37 - "DriveSetup"
Cohesion: 0.22
Nodes (7): copy_chip(), DriveSetup, numbered(), Request, Close, Save, stepper()

### Community 40 - ".tick"
Cohesion: 0.25
Nodes (4): a_failed_save_is_retried_only_on_change_or_request(), Lock, Autosave, autosave_waits_then_saves_the_latest_generation()

### Community 41 - "SyncError"
Cohesion: 0.07
Nodes (21): DriveState, Connected, Connecting, Disconnected, NotSetUp, error_message(), file_details(), is_transient() (+13 more)

### Community 44 - "keepsave-sync/src/update.rs"
Cohesion: 0.10
Nodes (22): StoredClient, Asset, check(), check_status(), CHECKSUMS, download(), expected_hash(), github() (+14 more)

### Community 45 - "CoreError"
Cohesion: 0.17
Nodes (11): CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io, Kdf, Merge (+3 more)

### Community 46 - "CreateVault"
Cohesion: 0.25
Nodes (5): CreateVault, Stage, Creating, Editing, Picking

### Community 47 - "Dialog"
Cohesion: 0.25
Nodes (6): Dialog, CloseUnsaved, DeleteForever, Discard, EmptyBin, GroupName

### Community 49 - "Field"
Cohesion: 0.50
Nodes (4): Field, Custom, Password, Username

### Community 50 - "keepsave-app/src/update.rs"
Cohesion: 0.24
Nodes (4): actions(), header(), Progress, progress_bar()

### Community 51 - "I18n"
Cohesion: 0.29
Nodes (4): arguments_are_interpolated_without_isolation_marks(), bundles_parse(), I18n, error_message()

### Community 52 - "Install"
Cohesion: 0.23
Nodes (8): cleanup(), encode_command(), Install, Msi, Portable, same_path(), sibling(), update_dir()

### Community 55 - "Credentials"
Cohesion: 0.11
Nodes (19): load_client(), BACKUPS_FOLDER_NAME, FOLDER_NAME, clients_are_checked_before_saving(), Credentials, GOOGLE_CLIENT_SUFFIX, MAX_BACKUPS, ACCOUNT (+11 more)

### Community 56 - ".to_data"
Cohesion: 0.47
Nodes (4): FieldName, Duplicate, Empty, Reserved

### Community 60 - "Screen"
Cohesion: 0.33
Nodes (6): Screen, Create, DriveImport, Main, Unlock, Welcome

### Community 61 - "keepsave-app/src/update.rs"
Cohesion: 0.07
Nodes (12): BODY_HEIGHT, COPIED_TIME, FULL_GUIDE, SCOPE, Step, STEP_TIME, STEPS, WIDTH (+4 more)

## Knowledge Gaps
- **158 isolated node(s):** `Welcome`, `Unlock`, `DriveImport`, `NotSetUp`, `Disconnected` (+153 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 418 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Nav` connect `Nav` to `app.rs`, `app.rs`, `Unlock`, `.tick`, `Background Worker`, `MainScreen`, `ScreenCtx`, `i18n.rs`, `CreateVault`, `RemoteFile`, `main/mod.rs`, `keepsave-app/src/update.rs`, `settings.rs`, `screens/mod.rs`, `main/generator.rs`, `unlock.rs`, `keepsave-app/src/update.rs`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **What connects `Welcome`, `Unlock`, `DriveImport` to the rest of the system?**
  _158 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `detail.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1226215644820296 - nodes in this community are weakly interconnected._
- **Why does `Action` connect `Action` to `detail.rs`, `.tick`, `MainScreen`, `dialogs.rs`, `ScreenCtx`, `Dialog`, `Field`, `screens/mod.rs`, `main/generator.rs`, `main/mod.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `Design Decisions and Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05658381808566896 - nodes in this community are weakly interconnected._
- **Why does `Vault` connect `Vault` to `detail.rs`, `src/vault.rs`, `sync.rs`, `Core Errors`, `Background Worker`, `MainScreen`, `SecretString`, `EntryId`, `CoreError`, `main/mod.rs`, `EntryData`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Should `Fonts and Entry Point` be split into smaller, more focused modules?**
  _Cohesion score 0.07549361207897794 - nodes in this community are weakly interconnected._