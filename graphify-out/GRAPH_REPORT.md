# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- 38 files · ~23,379 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 2, .ftl 2, .wgsl 1)

## Summary
- 792 nodes · 1881 edges · 30 communities (26 shown, 4 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 142 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5eea8483`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- detail.rs
- Entry Data Model
- Design Decisions and Docs
- app.rs
- Fonts and Entry Point
- Password Generator
- Shader Background
- Core Errors
- Fluent i18n
- Background Worker
- main/mod.rs
- dialogs.rs
- CI Workflow
- Create Vault Stages
- .ui
- Workspace Crates
- Workflow Rules
- Google Setup Guide
- session.rs
- Action
- config.rs
- create.rs
- unlock.rs
- sidebar.rs
- .to_data
- list.rs
- Field
- GroupKind

## God Nodes (most connected - your core abstractions)
1. `Vault` - 51 edges
2. `Action` - 49 edges
3. `ScreenCtx` - 48 edges
4. `MainScreen` - 45 edges
5. `Nav` - 27 edges
6. `CoreError` - 25 edges
7. `icon_button()` - 23 edges
8. `Event` - 22 edges
9. `I18n` - 21 edges
10. `EntryData` - 21 edges

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
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/detail.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/detail.rs`
- 3-file cycle: `crates/keepsave-app/src/screens/main/dialogs.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/main/mod.rs -> crates/keepsave-app/src/screens/main/dialogs.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Cargo workspace crates (core/sync/app)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, backups)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_keepass_style_merge, docs_design_q13_sync_behavior, docs_design_q17_new_pc_flow, docs_design_keepsave_sync [INFERRED 0.85]
- **Security controls (Argon2id, memory hygiene, keyring secrets, drive.file OAuth)** — readme_security_model, docs_design_q9_argon2id_kdf, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_q11_oauth [INFERRED 0.85]

## Communities (30 total, 4 thin omitted)

### Community 0 - "detail.rs"
Cohesion: 0.06
Nodes (48): a_new_draft_is_unchanged_until_typed_into(), an_edit_draft_tracks_changes_against_the_vault(), copy_button(), custom_field_names_are_validated(), custom_field_ui(), Detail, Edit, View (+40 more)

### Community 1 - "Entry Data Model"
Cohesion: 0.05
Nodes (30): Row, CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem, is_reserved(), DEFAULT_HISTORY_MAX_ITEMS (+22 more)

### Community 2 - "Design Decisions and Docs"
Cohesion: 0.06
Nodes (36): CLAUDE.md (Claude Code guidance), KeepSave password manager, config.toml, KeepSave Design (DESIGN.md), GOOGLE_SETUP.md, keepsave-app crate, keepsave-core crate, keepsave-sync crate (+28 more)

### Community 3 - "app.rs"
Cohesion: 0.12
Nodes (9): ENTRANCE_LIFT, ENTRANCE_TIME, KeepSaveApp, Screen, Create, Main, Unlock, Welcome (+1 more)

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
Cohesion: 0.07
Nodes (18): Event, NewVaultPathPicked, VaultCreated, VaultFilePicked, VaultSaved, VaultUnlocked, with_kdbx_extension(), Worker (+10 more)

### Community 10 - "main/mod.rs"
Cohesion: 0.06
Nodes (32): a_failed_save_is_retried_only_on_change_or_request(), Lock, Autosave, AUTOSAVE_DELAY, autosave_waits_then_saves_the_latest_generation(), browsable_url(), build_lists(), DETAIL_WIDTH (+24 more)

### Community 11 - "dialogs.rs"
Cohesion: 0.08
Nodes (22): buttons(), confirm_button(), danger_button(), Dialog, CloseUnsaved, DeleteForever, Discard, EmptyBin (+14 more)

### Community 12 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 13 - "Create Vault Stages"
Cohesion: 0.29
Nodes (5): CreateVault, Stage, Creating, Editing, Picking

### Community 14 - ".ui"
Cohesion: 0.19
Nodes (6): CardAction, Forget, None, Select, Unlock, vault_card()

### Community 15 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

### Community 20 - "session.rs"
Cohesion: 0.08
Nodes (6): CTX, LOCK_REQUESTED, run(), start(), wide(), window_proc()

### Community 21 - "Action"
Cohesion: 0.08
Nodes (25): Action, CancelEdit, CloseDetail, CloseWindow, CommitGroupName, Copy, DeleteEntry, DeleteEntryForever (+17 more)

### Community 22 - "config.rs"
Cohesion: 0.15
Nodes (5): Config, MAX_RECENT_VAULTS, missing_keys_take_their_defaults(), recent_vaults_are_unique_ordered_and_capped(), round_trip_and_missing_file()

### Community 23 - "create.rs"
Cohesion: 0.19
Nodes (4): language_switch(), Welcome, centered_column(), glass_card()

### Community 24 - "unlock.rs"
Cohesion: 0.23
Nodes (5): emblem(), link_pair(), title(), PASSWORD_ID, SHAKE_TIME

### Community 25 - "sidebar.rs"
Cohesion: 0.23
Nodes (4): group_row(), ROW_HEIGHT, show(), single_line()

### Community 26 - ".to_data"
Cohesion: 0.24
Nodes (5): FieldName, Duplicate, Empty, Reserved, same_data()

### Community 27 - "list.rs"
Cohesion: 0.33
Nodes (5): empty_state(), entry_row(), ROW_HEIGHT, show(), HOVER_TIME

### Community 28 - "Field"
Cohesion: 0.40
Nodes (5): Field, Custom, Password, Url, Username

### Community 29 - "GroupKind"
Cohesion: 0.50
Nodes (4): GroupKind, Bin, Group, Root

## Knowledge Gaps
- **84 isolated node(s):** `Welcome`, `Unlock`, `English`, `Spanish`, `Editing` (+79 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 257 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Vault` connect `Entry Data Model` to `detail.rs`, `Core Errors`, `Background Worker`, `main/mod.rs`, `.to_data`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **What connects `Welcome`, `Unlock`, `English` to the rest of the system?**
  _84 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `detail.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.056806227645697455 - nodes in this community are weakly interconnected._
- **Why does `Action` connect `Action` to `detail.rs`, `main/mod.rs`, `dialogs.rs`, `sidebar.rs`, `list.rs`, `Field`?**
  _High betweenness centrality (0.062) - this node is a cross-community bridge._
- **Should `Entry Data Model` be split into smaller, more focused modules?**
  _Cohesion score 0.05393258426966292 - nodes in this community are weakly interconnected._
- **Why does `ScreenCtx` connect `main/mod.rs` to `detail.rs`, `app.rs`, `Fluent i18n`, `Background Worker`, `dialogs.rs`, `.ui`, `config.rs`, `create.rs`, `unlock.rs`, `sidebar.rs`, `list.rs`?**
  _High betweenness centrality (0.060) - this node is a cross-community bridge._
- **Should `Design Decisions and Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05658381808566896 - nodes in this community are weakly interconnected._