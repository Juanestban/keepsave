# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- 45 files · ~28,108 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 2, .ftl 2, .wgsl 1)

## Summary
- 991 nodes · 2368 edges · 47 communities (39 shown, 8 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 154 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c0b0c506`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- detail.rs
- Vault
- Design Decisions and Docs
- app.rs
- Fonts and Entry Point
- Password Generator
- Shader Background
- Core Errors
- Fluent i18n
- Background Worker
- MainScreen
- dialogs.rs
- CI Workflow
- create.rs
- .ui
- Workspace Crates
- Workflow Rules
- keepsave-sync/src/lib.rs
- Google Setup Guide
- session.rs
- Action
- config.rs
- screens/mod.rs
- main/generator.rs
- sidebar.rs
- .to_data
- widgets.rs
- oauth.rs
- main/mod.rs
- DriveClient
- EntryData
- Nav
- keepsave-sync/src/drive.rs
- keepsave-app/src/drive.rs
- drive_import.rs
- tests/vault.rs
- GroupId
- CoreError
- src/vault.rs
- .tick
- SyncError
- ScreenCtx
- RemoteFile
- Dialog
- ActionButton<'a>
- ButtonStyle

## God Nodes (most connected - your core abstractions)
1. `Action` - 55 edges
2. `ScreenCtx` - 55 edges
3. `Vault` - 52 edges
4. `MainScreen` - 47 edges
5. `Nav` - 39 edges
6. `Event` - 34 edges
7. `DriveClient` - 29 edges
8. `CoreError` - 26 edges
9. `I18n` - 25 edges
10. `Worker` - 25 edges

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
- 1-file cycle: `crates/keepsave-app/src/screens/main/generator.rs -> crates/keepsave-app/src/screens/main/generator.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 2-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 3-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 4-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 4-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 4-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 4-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Cargo workspace crates (core/sync/app)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, backups)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_keepass_style_merge, docs_design_q13_sync_behavior, docs_design_q17_new_pc_flow, docs_design_keepsave_sync [INFERRED 0.85]
- **Security controls (Argon2id, memory hygiene, keyring secrets, drive.file OAuth)** — readme_security_model, docs_design_q9_argon2id_kdf, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_q11_oauth [INFERRED 0.85]

## Communities (47 total, 8 thin omitted)

### Community 0 - "detail.rs"
Cohesion: 0.13
Nodes (28): a_new_draft_is_unchanged_until_typed_into(), an_edit_draft_tracks_changes_against_the_vault(), copy_button(), custom_field_names_are_validated(), custom_field_ui(), Detail, Edit, View (+20 more)

### Community 2 - "Design Decisions and Docs"
Cohesion: 0.06
Nodes (36): CLAUDE.md (Claude Code guidance), KeepSave password manager, config.toml, KeepSave Design (DESIGN.md), GOOGLE_SETUP.md, keepsave-app crate, keepsave-core crate, keepsave-sync crate (+28 more)

### Community 3 - "app.rs"
Cohesion: 0.10
Nodes (10): ENTRANCE_LIFT, ENTRANCE_TIME, KeepSaveApp, Screen, Create, DriveImport, Main, Unlock (+2 more)

### Community 4 - "Fonts and Entry Point"
Cohesion: 0.07
Nodes (32): ICONS, install(), INTER_REGULAR, INTER_SEMIBOLD, SEMIBOLD, ACCENT, ACCENT_HOVER, apply() (+24 more)

### Community 5 - "Password Generator"
Cohesion: 0.09
Nodes (24): AMBIGUOUS, CharsetOptions, DIGITS, entropy_matches_pool_size(), estimate_entropy_bits(), estimate_grows_with_length_and_classes(), exclude_ambiguous_removes_them(), generate_passphrase() (+16 more)

### Community 6 - "Shader Background"
Cohesion: 0.08
Nodes (6): BackgroundCallback, FRAME_INTERVAL, paint_fallback(), Resources, UNFOCUSED_FRAME_INTERVAL, UNIFORMS_SIZE

### Community 7 - "Core Errors"
Cohesion: 0.09
Nodes (11): Pending, SecureClipboard, calibrate_respects_bounds(), config_round_trip(), default_parallelism(), KdfParams, .DEFAULT_MEMORY_BYTES, .DEFAULT_UNLOCK_TIME (+3 more)

### Community 8 - "Fluent i18n"
Cohesion: 0.11
Nodes (12): arguments_are_interpolated_without_isolation_marks(), both_languages_have_the_same_keys(), bundle_for(), bundles_parse(), ENGLISH_FTL, I18n, keys(), Language (+4 more)

### Community 9 - "Background Worker"
Cohesion: 0.08
Nodes (15): Event, DriveConnected, DriveDisconnected, DriveDownloaded, DriveFilesListed, DriveImportPathPicked, DriveUploaded, NewVaultPathPicked (+7 more)

### Community 10 - "MainScreen"
Cohesion: 0.13
Nodes (10): Discard, UnlinkDrive, browsable_url(), Field, Custom, Password, Username, MainScreen (+2 more)

### Community 11 - "dialogs.rs"
Cohesion: 0.24
Nodes (12): buttons(), confirm_button(), danger_button(), Outcome, Cancel, Confirm, Open, show() (+4 more)

### Community 12 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 13 - "create.rs"
Cohesion: 0.13
Nodes (5): CreateVault, Stage, Creating, Editing, Picking

### Community 14 - ".ui"
Cohesion: 0.15
Nodes (8): CardAction, Forget, None, Select, PASSWORD_ID, SHAKE_TIME, Unlock, vault_card()

### Community 15 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

### Community 17 - "keepsave-sync/src/lib.rs"
Cohesion: 0.20
Nodes (8): FOLDER_NAME, ACCOUNT, delete_refresh_token(), entry(), keyring_error(), load_refresh_token(), save_refresh_token(), SERVICE

### Community 20 - "session.rs"
Cohesion: 0.08
Nodes (6): CTX, LOCK_REQUESTED, run(), start(), wide(), window_proc()

### Community 21 - "Action"
Cohesion: 0.08
Nodes (28): Action, CancelDrive, CancelEdit, CloseDetail, CloseWindow, CommitGroupName, ConnectDrive, Copy (+20 more)

### Community 22 - "config.rs"
Cohesion: 0.11
Nodes (10): Config, drive_links_round_trip_and_replace(), DriveLink, MAX_RECENT_VAULTS, missing_keys_take_their_defaults(), recent_vaults_are_unique_ordered_and_capped(), round_trip_and_missing_file(), dotenv_value() (+2 more)

### Community 23 - "screens/mod.rs"
Cohesion: 0.24
Nodes (7): emblem(), link_pair(), title(), language_switch(), Welcome, centered_column(), glass_card()

### Community 24 - "main/generator.rs"
Cohesion: 0.14
Nodes (5): Generator, Mode, Characters, Passphrase, strength_meter()

### Community 25 - "sidebar.rs"
Cohesion: 0.17
Nodes (11): empty_state(), entry_row(), ROW_HEIGHT, show(), drive_row(), drive_section(), group_row(), ROW_HEIGHT (+3 more)

### Community 26 - ".to_data"
Cohesion: 0.24
Nodes (5): FieldName, Duplicate, Empty, Reserved, same_data()

### Community 27 - "widgets.rs"
Cohesion: 0.13
Nodes (13): focus_stroke(), glass_panel(), icon_link(), input_frame(), link(), multiline_field(), paint_highlight(), PANEL_SHADOW (+5 more)

### Community 28 - "oauth.rs"
Cohesion: 0.07
Nodes (36): browser_pages(), Url, access_from(), AccessToken, AUTH_URL, authorization_url(), authorization_url_asks_for_offline_drive_file_access(), authorize() (+28 more)

### Community 29 - "main/mod.rs"
Cohesion: 0.09
Nodes (18): AUTOSAVE_DELAY, build_lists(), DETAIL_WIDTH, GAP, GroupKind, Bin, Group, Root (+10 more)

### Community 30 - "DriveClient"
Cohesion: 0.17
Nodes (4): check(), DriveClient, http_client(), Credentials

### Community 31 - "EntryData"
Cohesion: 0.18
Nodes (6): Loaded, CustomField, EntryData, EntrySummary, HistoryItem, is_reserved()

### Community 32 - "Nav"
Cohesion: 0.12
Nodes (16): error_message(), Nav, CancelDrive, ConnectDrive, Create, DisconnectDrive, DriveImported, ForgetVault (+8 more)

### Community 33 - "keepsave-sync/src/drive.rs"
Cohesion: 0.12
Nodes (10): API, ApiError, ApiErrorBody, FILE_FIELDS, FileList, FOLDER_MIME, multipart_body_has_metadata_then_file(), multipart_related() (+2 more)

### Community 34 - "keepsave-app/src/drive.rs"
Cohesion: 0.13
Nodes (7): DriveState, Connected, Connecting, Disconnected, Unavailable, error_message(), file_details()

### Community 35 - "drive_import.rs"
Cohesion: 0.20
Nodes (10): DriveImport, file_row(), Stage, Downloading, Failed, Idle, Listing, Picking (+2 more)

### Community 36 - "tests/vault.rs"
Cohesion: 0.28
Nodes (15): a_clone_saves_independently(), assert_same(), changing_the_password_applies_on_next_save(), deleting_goes_through_the_recycle_bin(), deleting_groups_and_emptying_the_bin(), entries_under_includes_nested_groups(), history_is_capped_at_ten_versions(), new_vault() (+7 more)

### Community 38 - "CoreError"
Cohesion: 0.19
Nodes (10): CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io, Kdf, Open (+2 more)

### Community 39 - "src/vault.rs"
Cohesion: 0.17
Nodes (6): DEFAULT_HISTORY_MAX_ITEMS, DEFAULT_HISTORY_MAX_SIZE, GENERATOR, RECYCLE_BIN_ICON, RECYCLE_BIN_NAME, trim_history()

### Community 40 - ".tick"
Cohesion: 0.30
Nodes (4): a_failed_save_is_retried_only_on_change_or_request(), Lock, Autosave, autosave_waits_then_saves_the_latest_generation()

### Community 41 - "SyncError"
Cohesion: 0.17
Nodes (10): SyncError, Denied, Http, Io, Keyring, Network, NotConfigured, Protocol (+2 more)

### Community 44 - "Dialog"
Cohesion: 0.25
Nodes (6): Dialog, CloseUnsaved, DeleteForever, Discard, EmptyBin, GroupName

### Community 46 - "ButtonStyle"
Cohesion: 0.50
Nodes (4): ButtonStyle, Danger, Glass, Primary

## Knowledge Gaps
- **117 isolated node(s):** `Welcome`, `Unlock`, `DriveImport`, `Unavailable`, `Disconnected` (+112 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 325 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Vault` connect `Vault` to `detail.rs`, `tests/vault.rs`, `GroupId`, `src/vault.rs`, `Core Errors`, `Background Worker`, `MainScreen`, `.to_data`, `main/mod.rs`, `EntryData`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **What connects `Welcome`, `Unlock`, `DriveImport` to the rest of the system?**
  _117 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `detail.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13170731707317074 - nodes in this community are weakly interconnected._
- **Why does `ScreenCtx` connect `ScreenCtx` to `Nav`, `detail.rs`, `keepsave-app/src/drive.rs`, `app.rs`, `drive_import.rs`, `.tick`, `Fluent i18n`, `MainScreen`, `dialogs.rs`, `Background Worker`, `create.rs`, `.ui`, `config.rs`, `screens/mod.rs`, `main/generator.rs`, `sidebar.rs`, `main/mod.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `Design Decisions and Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05658381808566896 - nodes in this community are weakly interconnected._
- **Why does `Action` connect `Action` to `detail.rs`, `.tick`, `MainScreen`, `dialogs.rs`, `Dialog`, `ScreenCtx`, `sidebar.rs`, `main/mod.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `app.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10052910052910052 - nodes in this community are weakly interconnected._