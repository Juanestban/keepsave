# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- 49 files · ~35,471 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 2, .ftl 2, .wgsl 1)

## Summary
- 1172 nodes · 2950 edges · 48 communities (40 shown, 8 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 179 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `81150782`
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
- .ui
- make_icon.py
- Workspace Crates
- Workflow Rules
- keepsave-sync/src/lib.rs
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
- Vault
- Nav
- keepsave-sync/src/drive.rs
- app.rs
- FakeStore
- entry_row
- Stage
- Unlock
- .tick
- SyncError
- ScreenCtx
- RemoteFile
- Screen
- Dialog
- ActionButton<'a>
- Field

## God Nodes (most connected - your core abstractions)
1. `Action` - 65 edges
2. `ScreenCtx` - 61 edges
3. `Vault` - 60 edges
4. `MainScreen` - 56 edges
5. `Nav` - 51 edges
6. `RemoteFile` - 42 edges
7. `Event` - 36 edges
8. `DriveClient` - 36 edges
9. `Config` - 30 edges
10. `icon_button()` - 30 edges

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
- 2-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/sync.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 2-file cycle: `crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/drive_import.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/drive_import.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/settings.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 3-file cycle: `crates/keepsave-sync/src/drive.rs -> crates/keepsave-sync/src/oauth.rs -> crates/keepsave-sync/src/lib.rs -> crates/keepsave-sync/src/drive.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/main/generator.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Cargo workspace crates (core/sync/app)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, backups)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_keepass_style_merge, docs_design_q13_sync_behavior, docs_design_q17_new_pc_flow, docs_design_keepsave_sync [INFERRED 0.85]
- **Security controls (Argon2id, memory hygiene, keyring secrets, drive.file OAuth)** — readme_security_model, docs_design_q9_argon2id_kdf, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_q11_oauth [INFERRED 0.85]

## Communities (48 total, 8 thin omitted)

### Community 0 - "detail.rs"
Cohesion: 0.12
Nodes (32): a_new_draft_is_unchanged_until_typed_into(), an_edit_draft_tracks_changes_against_the_vault(), COLUMN_GAP, copy_button(), custom_field_names_are_validated(), Detail, Edit, View (+24 more)

### Community 1 - "sync.rs"
Cohesion: 0.21
Nodes (20): a_deleted_remote_file_is_reported(), a_remote_copy_with_another_password_is_an_error(), add(), at(), backup_name(), backup_names_round_trip(), backup_stem(), backup_time() (+12 more)

### Community 2 - "Design Decisions and Docs"
Cohesion: 0.06
Nodes (36): CLAUDE.md (Claude Code guidance), KeepSave password manager, config.toml, KeepSave Design (DESIGN.md), GOOGLE_SETUP.md, keepsave-app crate, keepsave-core crate, keepsave-sync crate (+28 more)

### Community 3 - "app.rs"
Cohesion: 0.22
Nodes (3): ENTRANCE_LIFT, ENTRANCE_TIME, Background

### Community 4 - "Fonts and Entry Point"
Cohesion: 0.07
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
Cohesion: 0.09
Nodes (16): arguments_are_interpolated_without_isolation_marks(), both_languages_have_the_same_keys(), bundle_for(), bundles_parse(), ENGLISH_FTL, I18n, keys(), Language (+8 more)

### Community 9 - "Background Worker"
Cohesion: 0.08
Nodes (16): Event, DriveConnected, DriveDisconnected, DriveDownloaded, DriveFilesListed, DriveImportPathPicked, DriveUploaded, NewVaultPathPicked (+8 more)

### Community 10 - "MainScreen"
Cohesion: 0.13
Nodes (8): Discard, UnlinkDrive, browsable_url(), DriveSync, MainScreen, Toast, error_message(), OpenVault

### Community 11 - "dialogs.rs"
Cohesion: 0.19
Nodes (16): buttons(), confirm_button(), danger_button(), Outcome, Cancel, Confirm, Open, show() (+8 more)

### Community 12 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 13 - ".ui"
Cohesion: 0.43
Nodes (3): DriveImport, file_row(), status_line()

### Community 14 - "make_icon.py"
Cohesion: 0.17
Nodes (8): bmp(), ico(), keyhole(), main(), png(), render(), rounded_box(), shade()

### Community 15 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

### Community 17 - "keepsave-sync/src/lib.rs"
Cohesion: 0.18
Nodes (6): BACKUPS_FOLDER_NAME, FOLDER_NAME, http_client(), Credentials, MAX_BACKUPS, SyncOutcome

### Community 20 - "session.rs"
Cohesion: 0.08
Nodes (6): CTX, LOCK_REQUESTED, run(), start(), wide(), window_proc()

### Community 21 - "Action"
Cohesion: 0.06
Nodes (33): Action, CancelDrive, CancelEdit, CloseDetail, CloseGenerator, CloseWindow, CommitGroupName, ConnectDrive (+25 more)

### Community 22 - "settings.rs"
Cohesion: 0.05
Nodes (47): Config, drive_links_round_trip_and_replace(), DriveLink, MAX_RECENT_VAULTS, missing_keys_take_their_defaults(), recent_vaults_are_unique_ordered_and_capped(), round_trip_and_missing_file(), icon() (+39 more)

### Community 23 - "CoreError"
Cohesion: 0.11
Nodes (26): CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io, Kdf, Merge (+18 more)

### Community 24 - "screens/mod.rs"
Cohesion: 0.15
Nodes (9): ROW_HEIGHT, drive_row(), drive_section(), group_row(), ROW_HEIGHT, show(), avatar(), HOVER_TIME (+1 more)

### Community 25 - "main/generator.rs"
Cohesion: 0.14
Nodes (8): Generator, Mode, Characters, Passphrase, Pick, Copy, Use, show_tool()

### Community 26 - "unlock.rs"
Cohesion: 0.11
Nodes (14): emblem(), link_pair(), title(), CardAction, Forget, None, Select, PASSWORD_ID (+6 more)

### Community 27 - "widgets.rs"
Cohesion: 0.13
Nodes (13): custom_field_ui(), focus_stroke(), glass_panel(), icon_button(), icon_link(), input_frame(), multiline_field(), paint_highlight() (+5 more)

### Community 28 - "oauth.rs"
Cohesion: 0.07
Nodes (36): browser_pages(), Url, access_from(), AccessToken, AUTH_URL, authorization_url(), authorization_url_asks_for_offline_drive_file_access(), authorize() (+28 more)

### Community 29 - "main/mod.rs"
Cohesion: 0.09
Nodes (19): AUTOSAVE_DELAY, build_lists(), DETAIL_WIDTH, GAP, GroupKind, Bin, Group, Root (+11 more)

### Community 31 - "Vault"
Cohesion: 0.06
Nodes (29): Loaded, same_data(), CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem, is_reserved() (+21 more)

### Community 32 - "Nav"
Cohesion: 0.11
Nodes (19): Nav, CancelDrive, ConnectDrive, Create, DisconnectDrive, DriveImported, DrivePending, DriveSynced (+11 more)

### Community 33 - "keepsave-sync/src/drive.rs"
Cohesion: 0.12
Nodes (11): API, ApiError, ApiErrorBody, FILE_FIELDS, FileList, FilePage, FOLDER_MIME, multipart_body_has_metadata_then_file() (+3 more)

### Community 34 - "app.rs"
Cohesion: 0.21
Nodes (3): KeepSaveApp, Create, Main

### Community 35 - "FakeStore"
Cohesion: 0.21
Nodes (3): FakeStore, prune_backups(), State

### Community 36 - "entry_row"
Cohesion: 0.47
Nodes (4): empty_state(), entry_row(), show(), Row

### Community 37 - "Stage"
Cohesion: 0.29
Nodes (7): Stage, Downloading, Failed, Idle, Listing, Picking, Ready

### Community 40 - ".tick"
Cohesion: 0.28
Nodes (4): a_failed_save_is_retried_only_on_change_or_request(), Lock, Autosave, autosave_waits_then_saves_the_latest_generation()

### Community 41 - "SyncError"
Cohesion: 0.06
Nodes (28): DriveState, Connected, Connecting, Disconnected, Unavailable, error_message(), file_details(), is_transient() (+20 more)

### Community 42 - "ScreenCtx"
Cohesion: 0.32
Nodes (3): show(), Regions, ScreenCtx

### Community 46 - "Screen"
Cohesion: 0.17
Nodes (9): Screen, DriveImport, Unlock, Welcome, CreateVault, Stage, Creating, Editing (+1 more)

### Community 47 - "Dialog"
Cohesion: 0.25
Nodes (6): Dialog, CloseUnsaved, DeleteForever, Discard, EmptyBin, GroupName

### Community 49 - "Field"
Cohesion: 0.50
Nodes (4): Field, Custom, Password, Username

## Knowledge Gaps
- **140 isolated node(s):** `Welcome`, `Unlock`, `DriveImport`, `Unavailable`, `Disconnected` (+135 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 377 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Vault` connect `Vault` to `detail.rs`, `sync.rs`, `Core Errors`, `Background Worker`, `MainScreen`, `keepsave-sync/src/lib.rs`, `CoreError`, `main/mod.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **What connects `Welcome`, `Unlock`, `DriveImport` to the rest of the system?**
  _140 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `detail.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1173054587688734 - nodes in this community are weakly interconnected._
- **Why does `RemoteFile` connect `RemoteFile` to `Nav`, `keepsave-sync/src/drive.rs`, `sync.rs`, `FakeStore`, `SyncError`, `Background Worker`, `.ui`, `keepsave-sync/src/lib.rs`, `settings.rs`, `screens/mod.rs`, `DriveClient`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._
- **Should `Design Decisions and Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05658381808566896 - nodes in this community are weakly interconnected._
- **Why does `Action` connect `Action` to `detail.rs`, `entry_row`, `.tick`, `ScreenCtx`, `dialogs.rs`, `MainScreen`, `Dialog`, `Field`, `screens/mod.rs`, `main/generator.rs`, `main/mod.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `Fonts and Entry Point` be split into smaller, more focused modules?**
  _Cohesion score 0.07419712070874862 - nodes in this community are weakly interconnected._