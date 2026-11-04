# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- Corpus is ~6,013 words - fits in a single context window. You may not need a graph.

## Summary
- 248 nodes · 425 edges · 16 communities (10 shown, 6 thin omitted)
- Extraction: 92% EXTRACTED · 8% INFERRED · 0% AMBIGUOUS · INFERRED: 33 edges (avg confidence: 0.86)
- Token cost: 52,304 input · 0 output

## Community Hubs (Navigation)
- CI & Roadmap
- Password Generator
- Docs & Conventions
- Vault API
- Entry Model
- KDF Calibration
- Vault Internals
- Core Errors
- Vault Tests
- Design: Secrets & Sync
- Workspace Crates

## God Nodes (most connected - your core abstractions)
1. `Vault` - 36 edges
2. `CoreError` - 17 edges
3. `KdfParams` - 16 edges
4. `EntryData` - 15 edges
5. `new_vault()` - 10 edges
6. `EntrySummary` - 9 edges
7. `KeepSave Roadmap` - 9 edges
8. `M1 - Workspace & core` - 9 edges
9. `sample_entry()` - 8 edges
10. `CLAUDE.md (Claude Code guidance)` - 8 edges

## Surprising Connections (you probably didn't know these)
- `Never block the UI thread` --semantically_similar_to--> `Q14 - Cargo workspace architecture`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Secrets handling convention (secrecy/zeroize, keyring)` --semantically_similar_to--> `Q10 - Memory hygiene and locking`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Build-time credential embedding (KEEPSAVE_GOOGLE_CLIENT_ID/SECRET via .env)` --references--> `M4 - Google Drive connection`  [INFERRED]
  docs/GOOGLE_SETUP.md → ROADMAP.md
- `M1 - Workspace & core` --references--> `CI Workflow (ci.yml)`  [INFERRED]
  ROADMAP.md → .github/workflows/ci.yml
- `cargo test --workspace step` --references--> `keepsave-core crate`  [INFERRED]
  .github/workflows/ci.yml → ROADMAP.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **CI quality gate (fmt, clippy, test)** — _github_workflows_ci_cargo_fmt_check, _github_workflows_ci_cargo_clippy, _github_workflows_ci_cargo_test [EXTRACTED 1.00]
- **KeepSave Cargo workspace crates (app -> sync -> core)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture, claude_dependency_direction [EXTRACTED 1.00]
- **KeepSave Cargo workspace crates** — roadmap_keepsave_core, roadmap_keepsave_sync, roadmap_keepsave_app [EXTRACTED 1.00]
- **RemoteStore trait and its implementations** — roadmap_remotestore, roadmap_drive_rest_impl, roadmap_fake_remotestore [EXTRACTED 1.00]
- **Google Drive OAuth + sync flow** — docs_design_q11_oauth, docs_design_q13_sync_behavior, docs_design_keepass_style_merge, docs_design_remotestore_trait, docs_google_setup_desktop_oauth_client, docs_design_q17_new_pc_flow [INFERRED 0.85]
- **Secret storage and memory hygiene** — claude_secrets_handling, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_keyring, docs_design_config_toml [INFERRED 0.85]

## Communities (16 total, 6 thin omitted)

### Community 0 - "CI & Roadmap"
Cohesion: 0.07
Nodes (37): CI Workflow (ci.yml), cargo clippy -D warnings step, cargo fmt --check step, cargo test --workspace step, CI check job (windows-latest), Swatinem/rust-cache, docs/DESIGN.md, docs/GOOGLE_SETUP.md (+29 more)

### Community 1 - "Password Generator"
Cohesion: 0.09
Nodes (22): AMBIGUOUS, CharsetOptions, DIGITS, entropy_matches_pool_size(), exclude_ambiguous_removes_them(), generate_passphrase(), generate_password(), GeneratorError (+14 more)

### Community 2 - "Docs & Conventions"
Cohesion: 0.09
Nodes (16): CLAUDE.md (Claude Code guidance), graphify knowledge-graph refresh workflow, KeepSave password manager, KeepSave Design document, drive.file OAuth scope, keepass crate, KeePassXC, keepsave-app crate (+8 more)

### Community 4 - "Entry Model"
Cohesion: 0.16
Nodes (5): CustomField, EntryData, EntrySummary, HistoryItem, is_reserved()

### Community 5 - "KDF Calibration"
Cohesion: 0.12
Nodes (9): calibrate_respects_bounds(), config_round_trip(), default_parallelism(), KdfParams, .DEFAULT_MEMORY_BYTES, .DEFAULT_UNLOCK_TIME, .MAX_ITERATIONS, .MIN_ITERATIONS (+1 more)

### Community 6 - "Vault Internals"
Cohesion: 0.12
Nodes (7): DEFAULT_HISTORY_MAX_ITEMS, DEFAULT_HISTORY_MAX_SIZE, GENERATOR, key_for(), RECYCLE_BIN_ICON, RECYCLE_BIN_NAME, trim_history()

### Community 7 - "Core Errors"
Cohesion: 0.19
Nodes (10): CoreError, EntryNotFound, GroupNotFound, HistoryNotFound, InvalidKey, Io, Kdf, Open (+2 more)

### Community 8 - "Vault Tests"
Cohesion: 0.32
Nodes (12): assert_same(), changing_the_password_applies_on_next_save(), deleting_goes_through_the_recycle_bin(), deleting_groups_and_emptying_the_bin(), history_is_capped_at_ten_versions(), new_vault(), PASSWORD, sample_entry() (+4 more)

### Community 9 - "Design: Secrets & Sync"
Cohesion: 0.19
Nodes (6): config.toml settings file, keyring OS credential store, Q16 - Screens (Welcome, Unlock, Main, Settings), Q17 - New PC import-from-Drive flow, Q19 - KeePass-compatible keyboard shortcuts, Settings -> Google Drive Connect/Disconnect

### Community 10 - "Workspace Crates"
Cohesion: 1.00
Nodes (3): keepsave-app, keepsave-core, keepsave-sync

## Knowledge Gaps
- **29 isolated node(s):** `InvalidKey`, `Save`, `Io`, `EntryNotFound`, `GroupNotFound` (+24 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 82 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Vault` connect `Vault API` to `Vault Tests`, `Entry Model`, `KDF Calibration`, `Vault Internals`?**
  _High betweenness centrality (0.099) - this node is a cross-community bridge._
- **What connects `InvalidKey`, `Save`, `Io` to the rest of the system?**
  _29 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `CI & Roadmap` be split into smaller, more focused modules?**
  _Cohesion score 0.06747638326585695 - nodes in this community are weakly interconnected._
- **Why does `CoreError` connect `Core Errors` to `Vault Tests`, `Entry Model`, `KDF Calibration`, `Vault Internals`?**
  _High betweenness centrality (0.071) - this node is a cross-community bridge._
- **Should `Password Generator` be split into smaller, more focused modules?**
  _Cohesion score 0.09411764705882353 - nodes in this community are weakly interconnected._
- **Why does `KdfParams` connect `KDF Calibration` to `Vault Tests`, `Entry Model`, `Vault Internals`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **Should `Docs & Conventions` be split into smaller, more focused modules?**
  _Cohesion score 0.09359605911330049 - nodes in this community are weakly interconnected._