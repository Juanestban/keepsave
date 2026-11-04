# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- 19 files · ~13,866 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 529 nodes · 1077 edges · 20 communities (15 shown, 5 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 87 edges (avg confidence: 0.89)
- Token cost: 69,942 input · 0 output

## Community Hubs (Navigation)
- Screens and Glass Widgets
- Entry Data Model
- Design Decisions and Docs
- App Shell and Navigation
- Fonts and Entry Point
- Password Generator
- Shader Background
- Core Errors
- Fluent i18n
- Background Worker
- Vault Integration Tests
- Action Button
- CI Workflow
- Create Vault Stages
- Vault Card Actions
- Workspace Crates
- Workflow Rules
- Google Setup Guide

## God Nodes (most connected - your core abstractions)
1. `Vault` - 36 edges
2. `Nav` - 24 edges
3. `ScreenCtx` - 19 edges
4. `Event` - 19 edges
5. `Worker` - 18 edges
6. `CoreError` - 17 edges
7. `Language` - 17 edges
8. `KdfParams` - 16 edges
9. `I18n` - 16 edges
10. `EntryData` - 15 edges

## Surprising Connections (you probably didn't know these)
- `Never block the UI thread` --semantically_similar_to--> `Q14 - Cargo workspace architecture (core/sync/app)`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Secrets handling convention (secrecy/zeroize, keyring)` --semantically_similar_to--> `Q10 - Memory hygiene and locking (secrecy/zeroize, auto-lock, clipboard clear)`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Q20 - Packaging (portable .exe, LTO; later MSI/auto-update)` --semantically_similar_to--> `Post-v1 backlog`  [INFERRED] [semantically similar]
  docs/DESIGN.md → ROADMAP.md
- `i18n via Fluent keys (es, en)` --conceptually_related_to--> `Q15 - i18n (Fluent es/en, sys-locale) and assets (Inter, Phosphor)`  [INFERRED]
  CLAUDE.md → docs/DESIGN.md
- `Build-time credential embedding (KEEPSAVE_GOOGLE_CLIENT_ID/SECRET via .env)` --references--> `M4 - Google Drive connection (todo)`  [INFERRED]
  docs/GOOGLE_SETUP.md → ROADMAP.md

## Import Cycles
- 1-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/vault.rs -> crates/keepsave-app/src/screens/mod.rs`
- 2-file cycle: `crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs`
- 3-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 3-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 3-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 3-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/vault.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 3-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/vault.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/vault.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 4-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/welcome.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`
- 5-file cycle: `crates/keepsave-app/src/app.rs -> crates/keepsave-app/src/screens/create.rs -> crates/keepsave-app/src/screens/mod.rs -> crates/keepsave-app/src/screens/unlock.rs -> crates/keepsave-app/src/worker.rs -> crates/keepsave-app/src/app.rs`

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **Google Drive sync flow (OAuth, RemoteStore, merge, backups)** — docs_design_q11_oauth, docs_design_remotestore_trait, docs_design_keepass_style_merge, docs_design_q13_sync_behavior, docs_design_q17_new_pc_flow, docs_design_keepsave_sync [INFERRED 0.85]
- **Cargo workspace crates (core/sync/app)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture [EXTRACTED 1.00]
- **Security controls (Argon2id, memory hygiene, keyring secrets, drive.file OAuth)** — readme_security_model, docs_design_q9_argon2id_kdf, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_q11_oauth [INFERRED 0.85]

## Communities (20 total, 5 thin omitted)

### Community 0 - "Screens and Glass Widgets"
Cohesion: 0.05
Nodes (35): CreateVault, emblem(), error_message(), link_pair(), Nav, Create, ForgetVault, Lock (+27 more)

### Community 1 - "Entry Data Model"
Cohesion: 0.07
Nodes (14): CustomField, EntryData, EntrySummary, GroupSummary, HistoryItem, is_reserved(), DEFAULT_HISTORY_MAX_ITEMS, DEFAULT_HISTORY_MAX_SIZE (+6 more)

### Community 2 - "Design Decisions and Docs"
Cohesion: 0.06
Nodes (36): CLAUDE.md (Claude Code guidance), KeepSave password manager, config.toml, KeepSave Design (DESIGN.md), GOOGLE_SETUP.md, keepsave-app crate, keepsave-core crate, keepsave-sync crate (+28 more)

### Community 3 - "App Shell and Navigation"
Cohesion: 0.06
Nodes (17): ENTRANCE_LIFT, ENTRANCE_TIME, KeepSaveApp, Screen, Create, Unlock, Vault, Welcome (+9 more)

### Community 4 - "Fonts and Entry Point"
Cohesion: 0.07
Nodes (31): ICONS, install(), INTER_REGULAR, INTER_SEMIBOLD, SEMIBOLD, ACCENT, ACCENT_HOVER, apply() (+23 more)

### Community 5 - "Password Generator"
Cohesion: 0.09
Nodes (24): AMBIGUOUS, CharsetOptions, DIGITS, entropy_matches_pool_size(), estimate_entropy_bits(), estimate_grows_with_length_and_classes(), exclude_ambiguous_removes_them(), generate_passphrase() (+16 more)

### Community 6 - "Shader Background"
Cohesion: 0.07
Nodes (6): BackgroundCallback, FRAME_INTERVAL, paint_fallback(), Resources, UNFOCUSED_FRAME_INTERVAL, UNIFORMS_SIZE

### Community 7 - "Core Errors"
Cohesion: 0.08
Nodes (19): CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io, Kdf, Open (+11 more)

### Community 8 - "Fluent i18n"
Cohesion: 0.12
Nodes (12): arguments_are_interpolated_without_isolation_marks(), both_languages_have_the_same_keys(), bundle_for(), bundles_parse(), ENGLISH_FTL, I18n, keys(), Language (+4 more)

### Community 9 - "Background Worker"
Cohesion: 0.12
Nodes (7): Event, NewVaultPathPicked, VaultCreated, VaultFilePicked, VaultUnlocked, with_kdbx_extension(), Worker

### Community 10 - "Vault Integration Tests"
Cohesion: 0.32
Nodes (12): assert_same(), changing_the_password_applies_on_next_save(), deleting_goes_through_the_recycle_bin(), deleting_groups_and_emptying_the_bin(), history_is_capped_at_ten_versions(), new_vault(), PASSWORD, sample_entry() (+4 more)

### Community 12 - "CI Workflow"
Cohesion: 0.33
Nodes (6): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache

### Community 13 - "Create Vault Stages"
Cohesion: 0.50
Nodes (4): Stage, Creating, Editing, Picking

### Community 14 - "Vault Card Actions"
Cohesion: 0.50
Nodes (4): CardAction, Forget, None, Select

### Community 15 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

## Knowledge Gaps
- **43 isolated node(s):** `EntryNotFound`, `GroupNotFound`, `HistoryNotFound`, `InvalidKey`, `Io` (+38 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 174 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Vault` connect `Entry Data Model` to `Vault Integration Tests`, `Core Errors`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **What connects `EntryNotFound`, `GroupNotFound`, `HistoryNotFound` to the rest of the system?**
  _43 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Screens and Glass Widgets` be split into smaller, more focused modules?**
  _Cohesion score 0.05359719645433931 - nodes in this community are weakly interconnected._
- **Why does `CoreError` connect `Core Errors` to `Entry Data Model`, `Vault Integration Tests`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Should `Entry Data Model` be split into smaller, more focused modules?**
  _Cohesion score 0.06599597585513078 - nodes in this community are weakly interconnected._
- **Why does `Nav` connect `Screens and Glass Widgets` to `Fluent i18n`, `App Shell and Navigation`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Should `Design Decisions and Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.05605499735589635 - nodes in this community are weakly interconnected._