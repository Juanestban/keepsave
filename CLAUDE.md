# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project

KeepSave is a personal desktop password manager in native Rust
(`egui`/`eframe`, wgpu backend). It stores vaults as standard KDBX4 files
(KeePass-compatible), has a minimalist dark "liquid glass" UI, and syncs vaults
bidirectionally through Google Drive.

- Repo: <https://github.com/Juanestban/keepsave>
- License: MIT OR Apache-2.0
- **Read first:** [docs/DESIGN.md](docs/DESIGN.md) for every design decision
  (Q1–Q24) and [ROADMAP.md](ROADMAP.md) for current progress.

## Layout (target, from M1)

```
crates/keepsave-core   # model, KDBX I/O, merge, generator — no UI, no network
crates/keepsave-sync   # OAuth (loopback + PKCE) + Drive REST, behind a RemoteStore trait
crates/keepsave-app    # eframe binary: glass theme, animations, Fluent i18n, screens
docs/                  # DESIGN.md, GOOGLE_SETUP.md
```

## Commands

```sh
cargo build
cargo run -p keepsave-app
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release -p keepsave-app   # portable .exe
```

## Conventions

- **Language:** code, identifiers, comments, commit messages and every `.md`
  file are in **English**. Chat with the user is in Spanish.
- **Dependency direction:** `app → sync → core` and `app → core`. Core has no
  dependency on UI, network or async.
- **Never block the UI thread:** Argon2 and network work run on the background
  tokio runtime and talk to the UI over channels.
- **Secrets:** wrap them in `secrecy::SecretString` / `zeroize`. Never log
  them, never `Debug`-print them, never write them to `config.toml`. OAuth
  tokens live in `keyring`.
- **i18n:** all user-visible strings go through Fluent keys, in both
  `es` and `en` `.ftl` files.
- **File format:** stay strictly KDBX4-compatible. A vault must still open in
  KeePassXC.

## Workflow with the user

- **Commits:** the user commits manually. After each change, **suggest** a
  Conventional Commit message (`feat(core): …`, `fix(app): …`, `docs: …`,
  `chore: …`) and the files to stage. Do not run `git commit` or `git push`.
- **Roadmap:** after finishing a task, tick it in `ROADMAP.md` and update the
  milestone status.
- **graphify:** after each increment (milestone or meaningful feature), refresh
  the knowledge graph in `graphify-out/` with the graphify skill. Before
  exploring the codebase, query the graph first to save tokens.
- **Design changes:** update `docs/DESIGN.md` first, then code.
