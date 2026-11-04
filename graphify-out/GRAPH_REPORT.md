# Graph Report - keepsave  (2026-11-04)

## Corpus Check
- Corpus is ~2,669 words - fits in a single context window. You may not need a graph.

## Summary
- 58 nodes · 87 edges · 12 communities (5 shown, 7 thin omitted)
- Extraction: 55% EXTRACTED · 45% INFERRED · 0% AMBIGUOUS · INFERRED: 39 edges (avg confidence: 0.9)
- Token cost: 66,809 input · 0 output

## Community Hubs (Navigation)
- Secrets & Google OAuth
- Docs & Workflow
- KDBX Storage & Core
- Workspace Crates
- UI Screens & i18n
- Glass Look & Release
- Sync & Merge
- Scope & Generator
- Cargo Package

## God Nodes (most connected - your core abstractions)
1. `CLAUDE.md (Claude Code guidance)` - 10 edges
2. `Q11 - OAuth (Desktop client, loopback + PKCE, drive.file)` - 7 edges
3. `M4 - Google Drive connection` - 6 edges
4. `Q14 - Cargo workspace architecture` - 6 edges
5. `keepsave-core crate` - 6 edges
6. `keepsave-sync crate` - 6 edges
7. `M1 - Workspace & core` - 5 edges
8. `KeepSave Design document` - 5 edges
9. `config.toml settings file` - 5 edges
10. `Q13 - Sync behavior (triggers, change detection, offline, backups)` - 5 edges

## Surprising Connections (you probably didn't know these)
- `Never block the UI thread` --semantically_similar_to--> `Q14 - Cargo workspace architecture`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `Secrets handling convention (secrecy/zeroize, keyring)` --semantically_similar_to--> `Q10 - Memory hygiene and locking`  [INFERRED] [semantically similar]
  CLAUDE.md → docs/DESIGN.md
- `CLAUDE.md (Claude Code guidance)` --references--> `Q2 - KDBX4 file format`  [INFERRED]
  CLAUDE.md → docs/DESIGN.md
- `Dependency direction app -> sync -> core` --conceptually_related_to--> `Q14 - Cargo workspace architecture`  [INFERRED]
  CLAUDE.md → docs/DESIGN.md
- `M1 - Workspace & core` --implements--> `Q14 - Cargo workspace architecture`  [INFERRED]
  ROADMAP.md → docs/DESIGN.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **KeepSave Cargo workspace crates (app -> sync -> core)** — docs_design_keepsave_core, docs_design_keepsave_sync, docs_design_keepsave_app, docs_design_q14_architecture, claude_dependency_direction [EXTRACTED 1.00]
- **Google Drive OAuth + sync flow** — docs_design_q11_oauth, docs_design_q13_sync_behavior, docs_design_keepass_style_merge, docs_design_remotestore_trait, docs_google_setup_desktop_oauth_client, docs_design_q17_new_pc_flow [INFERRED 0.85]
- **Secret storage and memory hygiene** — claude_secrets_handling, docs_design_q10_memory_hygiene_locking, docs_design_q12_secrets_vs_settings, docs_design_keyring, docs_design_config_toml [INFERRED 0.85]

## Communities (12 total, 7 thin omitted)

### Community 0 - "Secrets & Google OAuth"
Cohesion: 0.23
Nodes (9): config.toml settings file, drive.file OAuth scope, keyring OS credential store, Q17 - New PC import-from-Drive flow, Build-time credential embedding (KEEPSAVE_GOOGLE_CLIENT_ID/SECRET via .env), Settings -> Google Drive Connect/Disconnect, Desktop app OAuth client, OAuth consent screen (External, In production) (+1 more)

### Community 1 - "Docs & Workflow"
Cohesion: 0.38
Nodes (5): graphify knowledge-graph refresh workflow, KeepSave Design document, Google Drive setup guide, M0 - Design & repo bootstrap (done), KeepSave Roadmap

### Community 2 - "KDBX Storage & Core"
Cohesion: 0.33
Nodes (3): keepass crate, KeePassXC, M1 - Workspace & core

### Community 3 - "Workspace Crates"
Cohesion: 0.53
Nodes (6): CLAUDE.md (Claude Code guidance), KeepSave password manager, keepsave-app crate, keepsave-core crate, keepsave-sync crate, RemoteStore trait

### Community 4 - "UI Screens & i18n"
Cohesion: 0.40
Nodes (4): Q16 - Screens (Welcome, Unlock, Main, Settings), Q19 - KeePass-compatible keyboard shortcuts, M2 - UI foundation, M3 - Main screen

## Knowledge Gaps
- **4 isolated node(s):** `keepsave`, `KeepSave password manager`, `graphify knowledge-graph refresh workflow`, `Post-v1 backlog`
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 14 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **7 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `CLAUDE.md (Claude Code guidance)` connect `Workspace Crates` to `Docs & Workflow`, `KDBX Storage & Core`?**
  _High betweenness centrality (0.248) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `Q11 - OAuth (Desktop client, loopback + PKCE, drive.file)` (e.g. with `keepsave-sync crate` and `Desktop app OAuth client`) actually correct?**
  _`Q11 - OAuth (Desktop client, loopback + PKCE, drive.file)` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `keepsave`, `KeepSave password manager`, `graphify knowledge-graph refresh workflow` to the rest of the system?**
  _4 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Why does `Q11 - OAuth (Desktop client, loopback + PKCE, drive.file)` connect `Secrets & Google OAuth` to `Docs & Workflow`, `Workspace Crates`?**
  _High betweenness centrality (0.172) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `M4 - Google Drive connection` (e.g. with `Build-time credential embedding (KEEPSAVE_GOOGLE_CLIENT_ID/SECRET via .env)` and `Q11 - OAuth (Desktop client, loopback + PKCE, drive.file)`) actually correct?**
  _`M4 - Google Drive connection` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Why does `Q16 - Screens (Welcome, Unlock, Main, Settings)` connect `UI Screens & i18n` to `Secrets & Google OAuth`, `KDBX Storage & Core`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Q14 - Cargo workspace architecture` (e.g. with `Dependency direction app -> sync -> core` and `Never block the UI thread`) actually correct?**
  _`Q14 - Cargo workspace architecture` has 3 INFERRED edges - model-reasoned connections that need verification._