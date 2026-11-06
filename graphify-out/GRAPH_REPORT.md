# Graph Report - keepsave  (2026-11-06)

## Corpus Check
- Corpus is ~44,419 words - fits in a single context window. You may not need a graph.

## Summary
- 1387 nodes · 3513 edges · 62 communities (50 shown, 12 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 205 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Core Entry Model
- Release & Conventions Docs
- App Config
- RemoteStore & Sync
- Windows Hello Quick Unlock
- OAuth PKCE Flow
- Drive Setup Guide
- Secure Clipboard
- Fonts & Theme
- Widgets
- Main Screen Actions
- Password Generator Core
- Google Client Credentials
- OS Session Lock
- Drive State
- Worker & Progress
- Update Check & Download
- Worker Events
- Entry Detail View
- Create Vault Screen
- Main Screen Layout
- Navigation Requests
- Drive REST Client
- i18n Bundles
- Drive Import Screen
- Main Screen State
- Icon Script
- Generator Modal
- Dialogs
- App Shell
- Drive API Types
- Background Shader
- Main Screen Handlers
- Drive Setup Modal
- Unlock Screen
- Autosave
- Entry Drafts
- Update Install
- Updater States
- Updater
- Entry List
- Settings Tabs
- Unlock Flow
- Background
- Drive Import Stages
- Detail Modes
- Main Dialogs
- ActionButton
- wgpu Resources
- App Module
- Error Messages
- CI Workflow
- Screens
- Field Names
- Entry Fields
- Generator Pick
- Workspace Crates
- Scope & Backlog

## God Nodes (most connected - your core abstractions)
1. `Action` - 65 edges
2. `ScreenCtx` - 64 edges
3. `Nav` - 63 edges
4. `Vault` - 61 edges
5. `MainScreen` - 59 edges
6. `Event` - 46 edges
7. `RemoteFile` - 42 edges
8. `I18n` - 37 edges
9. `DriveClient` - 36 edges
10. `Worker` - 35 edges

## Surprising Connections (you probably didn't know these)
- `Updates (README)` --semantically_similar_to--> `Q24 Updates (GitHub latest release check, banner, SHA-256, MSI/portable install)`  [INFERRED] [semantically similar]
  README.md → docs/DESIGN.md
- `Keyboard shortcuts (README)` --semantically_similar_to--> `Q19 KeePass-compatible keyboard shortcuts`  [INFERRED] [semantically similar]
  README.md → docs/DESIGN.md
- `M7 Installer & updates (1.1.0)` --implements--> `Release workflow (tag v*)`  [INFERRED]
  ROADMAP.md → .github/workflows/release.yml
- `Publish GitHub Release (prerelease on '-' suffix)` --shares_data_with--> `Q24 Updates (GitHub latest release check, banner, SHA-256, MSI/portable install)`  [INFERRED]
  .github/workflows/release.yml → docs/DESIGN.md
- `Dependency direction app -> sync -> core` --conceptually_related_to--> `Q14 Architecture (Cargo workspace, tokio worker, channels)`  [INFERRED]
  CLAUDE.md → docs/DESIGN.md

## Import Cycles
- 1-file cycle: `crates/keepsave-app/src/hello.rs -> crates/keepsave-app/src/hello.rs`
- 1-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/create.rs`
- 1-file cycle: `crates/keepsave-app/src/session.rs -> crates/keepsave-app/src/session.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/sync.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/token_store.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/update.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/update.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 3-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/token_store.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Bring-your-own Google OAuth client setup steps** — docs_google_setup_create_project, docs_google_setup_enable_drive_api, docs_google_setup_consent_screen, docs_google_setup_create_oauth_client, docs_google_setup_give_credentials, docs_google_setup_connect [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, pending upload, drive_links)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_q13_sync_behavior, docs_design_merge_engine, docs_design_pending_upload, docs_design_drive_links [INFERRED 0.85]
- **Release and update pipeline (tag -> CI checks -> MSI/portable + SHA256SUMS -> in-app updater)** — _github_workflows_release_release_workflow, _github_workflows_release_package_step, _github_workflows_release_publish_release, docs_design_q20_packaging, docs_design_q24_updates, readme_releasing [INFERRED 0.85]

## Communities (62 total, 12 thin omitted)

### Community 0 - "Core Entry Model"
Cohesion: 0.05
Nodes (42): CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem, is_reserved(), a_copy_with_another_password_is_rejected(), an_edit_after_a_remote_deletion_keeps_the_entry() (+34 more)

### Community 1 - "Release & Conventions Docs"
Cohesion: 0.05
Nodes (42): Package step (WiX 5.0.2 + scripts/package.ps1), Publish GitHub Release (prerelease on '-' suffix), Release workflow (tag v*), Tag matches Cargo.toml version check, CLAUDE.md project guidance, KeepSave Design (DESIGN.md), Drive in the UI (sidebar states), drive_links in config.toml (+34 more)

### Community 2 - "App Config"
Cohesion: 0.08
Nodes (29): Config, drive_links_round_trip_and_replace(), DriveLink, MAX_RECENT_VAULTS, missing_keys_take_their_defaults(), quick_unlock_is_per_vault_and_forgotten_with_it(), recent_vaults_are_unique_ordered_and_capped(), round_trip_and_missing_file() (+21 more)

### Community 3 - "RemoteStore & Sync"
Cohesion: 0.10
Nodes (26): RemoteFile, RemoteStore, a_deleted_remote_file_is_reported(), a_remote_copy_with_another_password_is_an_error(), add(), at(), backup_name(), backup_names_round_trip() (+18 more)

### Community 4 - "Windows Hello Quick Unlock"
Cohesion: 0.06
Nodes (25): available(), bound_to_its_vault(), check(), cipher(), ciphertext_hides_the_password(), CREDENTIAL, HelloError, Cancelled (+17 more)

### Community 5 - "OAuth PKCE Flow"
Cohesion: 0.08
Nodes (35): Url, access_from(), AccessToken, AUTH_URL, authorization_url(), authorization_url_asks_for_offline_drive_file_access(), authorize(), browser_pages_are_escaped() (+27 more)

### Community 6 - "Drive Setup Guide"
Cohesion: 0.06
Nodes (21): BODY_HEIGHT, COPIED_TIME, FULL_GUIDE, SCOPE, Step, STEP_TIME, STEPS, WIDTH (+13 more)

### Community 7 - "Secure Clipboard"
Cohesion: 0.06
Nodes (22): Pending, SecureClipboard, CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io (+14 more)

### Community 8 - "Fonts & Theme"
Cohesion: 0.08
Nodes (32): ICONS, install(), INTER_REGULAR, INTER_SEMIBOLD, SEMIBOLD, ACCENT, ACCENT_HOVER, apply() (+24 more)

### Community 9 - "Widgets"
Cohesion: 0.12
Nodes (15): field_label(), focus_stroke(), glass_panel(), icon_button(), icon_link(), input_frame(), label_row(), multiline_field() (+7 more)

### Community 10 - "Main Screen Actions"
Cohesion: 0.06
Nodes (35): Action, CancelDrive, CancelEdit, CloseDetail, CloseGenerator, CloseWindow, CommitGroupName, ConnectDrive (+27 more)

### Community 11 - "Password Generator Core"
Cohesion: 0.09
Nodes (24): AMBIGUOUS, CharsetOptions, DIGITS, entropy_matches_pool_size(), estimate_entropy_bits(), estimate_grows_with_length_and_classes(), exclude_ambiguous_removes_them(), generate_passphrase() (+16 more)

### Community 12 - "Google Client Credentials"
Cohesion: 0.10
Nodes (20): load_client(), BACKUPS_FOLDER_NAME, FOLDER_NAME, clients_are_checked_before_saving(), Credentials, GOOGLE_CLIENT_SUFFIX, MAX_BACKUPS, ACCOUNT (+12 more)

### Community 13 - "OS Session Lock"
Cohesion: 0.08
Nodes (6): CTX, LOCK_REQUESTED, run(), start(), wide(), window_proc()

### Community 14 - "Drive State"
Cohesion: 0.07
Nodes (22): browser_pages(), DriveState, Connected, Connecting, Disconnected, NotSetUp, error_message(), file_details() (+14 more)

### Community 15 - "Worker & Progress"
Cohesion: 0.10
Nodes (5): Progress, DriveConnected, with_kdbx_extension(), Worker, write_atomically()

### Community 16 - "Update Check & Download"
Cohesion: 0.12
Nodes (21): Asset, check(), check_status(), CHECKSUMS, download(), expected_hash(), github(), GitHubRelease (+13 more)

### Community 17 - "Worker Events"
Cohesion: 0.08
Nodes (20): Event, DriveDisconnected, DriveDownloaded, DriveFilesListed, DriveImportPathPicked, DriveUploaded, HelloChecked, NewVaultPathPicked (+12 more)

### Community 18 - "Entry Detail View"
Cohesion: 0.15
Nodes (22): COLUMN_GAP, copy_button(), custom_field_ui(), DraftField, editor_footer(), editor_header(), EDITOR_WIDTH, FIELD_GAP (+14 more)

### Community 19 - "Create Vault Screen"
Cohesion: 0.13
Nodes (11): CreateVault, Stage, Creating, Editing, Picking, emblem(), title(), Welcome (+3 more)

### Community 20 - "Main Screen Layout"
Cohesion: 0.09
Nodes (20): AUTOSAVE_DELAY, browsable_url(), build_lists(), DETAIL_WIDTH, GAP, GroupKind, Bin, Group (+12 more)

### Community 21 - "Navigation Requests"
Cohesion: 0.07
Nodes (27): Nav, CancelDrive, ChangeDriveClient, CheckForUpdates, ConnectDrive, Create, DisconnectDrive, DownloadUpdate (+19 more)

### Community 22 - "Drive REST Client"
Cohesion: 0.16
Nodes (3): check(), DriveClient, http_client()

### Community 23 - "i18n Bundles"
Cohesion: 0.11
Nodes (11): arguments_are_interpolated_without_isolation_marks(), both_languages_have_the_same_keys(), bundle_for(), bundles_parse(), ENGLISH_FTL, keys(), Language, .ALL (+3 more)

### Community 24 - "Drive Import Screen"
Cohesion: 0.17
Nodes (10): DriveImport, file_row(), status_line(), drive_row(), drive_section(), group_row(), ROW_HEIGHT, show() (+2 more)

### Community 25 - "Main Screen State"
Cohesion: 0.19
Nodes (4): DriveSync, MainScreen, Toast, OpenVault

### Community 26 - "Icon Script"
Cohesion: 0.17
Nodes (8): bmp(), ico(), keyhole(), main(), png(), render(), rounded_box(), shade()

### Community 27 - "Generator Modal"
Cohesion: 0.16
Nodes (6): Generator, Mode, Characters, Passphrase, show_tool(), strength_meter()

### Community 28 - "Dialogs"
Cohesion: 0.19
Nodes (16): buttons(), confirm_button(), danger_button(), Outcome, Cancel, Confirm, Open, show() (+8 more)

### Community 30 - "Drive API Types"
Cohesion: 0.12
Nodes (11): API, ApiError, ApiErrorBody, FILE_FIELDS, FileList, FilePage, FOLDER_MIME, multipart_body_has_metadata_then_file() (+3 more)

### Community 31 - "Background Shader"
Cohesion: 0.15
Nodes (4): BackgroundCallback, FRAME_INTERVAL, UNFOCUSED_FRAME_INTERVAL, UNIFORMS_SIZE

### Community 32 - "Main Screen Handlers"
Cohesion: 0.24
Nodes (3): Regions, error_message(), ScreenCtx

### Community 33 - "Drive Setup Modal"
Cohesion: 0.22
Nodes (7): copy_chip(), DriveSetup, numbered(), Request, Close, Save, stepper()

### Community 34 - "Unlock Screen"
Cohesion: 0.15
Nodes (8): link_pair(), CardAction, Forget, None, Select, PASSWORD_ID, SHAKE_TIME, vault_card()

### Community 35 - "Autosave"
Cohesion: 0.25
Nodes (4): a_failed_save_is_retried_only_on_change_or_request(), Lock, Autosave, autosave_waits_then_saves_the_latest_generation()

### Community 36 - "Entry Drafts"
Cohesion: 0.27
Nodes (6): a_new_draft_is_unchanged_until_typed_into(), an_edit_draft_tracks_changes_against_the_vault(), custom_field_names_are_validated(), Draft, same_data(), vault()

### Community 37 - "Update Install"
Cohesion: 0.23
Nodes (8): cleanup(), encode_command(), Install, Msi, Portable, same_path(), sibling(), update_dir()

### Community 38 - "Updater States"
Cohesion: 0.18
Nodes (9): State, Available, CheckFailed, Checking, Downloading, Failed, Idle, Ready (+1 more)

### Community 40 - "Entry List"
Cohesion: 0.40
Nodes (6): empty_state(), entry_row(), ROW_HEIGHT, show(), Row, avatar()

### Community 41 - "Settings Tabs"
Cohesion: 0.22
Nodes (9): Settings, Tab, About, .ALL, Appearance, Drive, General, Language (+1 more)

### Community 44 - "Drive Import Stages"
Cohesion: 0.29
Nodes (7): Stage, Downloading, Failed, Idle, Listing, Picking, Ready

### Community 45 - "Detail Modes"
Cohesion: 0.32
Nodes (5): Detail, Edit, View, EntryView, Loaded

### Community 46 - "Main Dialogs"
Cohesion: 0.25
Nodes (6): Dialog, CloseUnsaved, DeleteForever, Discard, EmptyBin, GroupName

### Community 50 - "Error Messages"
Cohesion: 0.38
Nodes (3): I18n, hello_error_message(), error_message()

### Community 51 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 52 - "Screens"
Cohesion: 0.33
Nodes (6): Screen, Create, DriveImport, Main, Unlock, Welcome

### Community 53 - "Field Names"
Cohesion: 0.47
Nodes (4): FieldName, Duplicate, Empty, Reserved

### Community 54 - "Entry Fields"
Cohesion: 0.50
Nodes (4): Field, Custom, Password, Username

### Community 55 - "Generator Pick"
Cohesion: 0.67
Nodes (3): Pick, Copy, Use

### Community 56 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

## Knowledge Gaps
- **164 isolated node(s):** `Welcome`, `Unlock`, `DriveImport`, `NotSetUp`, `Disconnected` (+159 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 440 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Event` connect `Worker Events` to `Main Screen Handlers`, `Unlock Screen`, `RemoteStore & Sync`, `Windows Hello Quick Unlock`, `Secure Clipboard`, `Drive Import Stages`, `Drive State`, `Worker & Progress`, `Update Check & Download`, `App Module`, `Create Vault Screen`, `Main Screen Layout`, `Drive REST Client`, `Drive Import Screen`, `Main Screen State`, `App Shell`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **What connects `Welcome`, `Unlock`, `DriveImport` to the rest of the system?**
  _164 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core Entry Model` be split into smaller, more focused modules?**
  _Cohesion score 0.050614250614250615 - nodes in this community are weakly interconnected._
- **Why does `ScreenCtx` connect `Main Screen Handlers` to `App Config`, `Windows Hello Quick Unlock`, `Drive Setup Guide`, `Drive State`, `Worker & Progress`, `Entry Detail View`, `Create Vault Screen`, `Main Screen Layout`, `Drive Import Screen`, `Main Screen State`, `Generator Modal`, `Dialogs`, `Unlock Screen`, `Autosave`, `Entry List`, `Unlock Flow`, `Drive Import Stages`, `App Module`, `Error Messages`?**
  _High betweenness centrality (0.057) - this node is a cross-community bridge._
- **Should `Release & Conventions Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05144230769230769 - nodes in this community are weakly interconnected._
- **Why does `Nav` connect `Navigation Requests` to `Main Screen Handlers`, `App Config`, `Autosave`, `RemoteStore & Sync`, `Unlock Screen`, `Drive Setup Guide`, `Drive Import Stages`, `App Module`, `Worker Events`, `Create Vault Screen`, `Main Screen Layout`, `i18n Bundles`, `Drive Import Screen`, `Main Screen State`, `App Shell`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Should `App Config` be split into smaller, more focused modules?**
  _Cohesion score 0.08287961282516637 - nodes in this community are weakly interconnected._