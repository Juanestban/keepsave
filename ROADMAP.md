# Roadmap

Status legend: ⬜ todo · 🟨 in progress · ✅ done

Every milestone ends with something runnable and a suggested commit. Design
rationale lives in [docs/DESIGN.md](docs/DESIGN.md).

## M0 — Design & repo bootstrap ✅

- [x] Design interview (Q1–Q23) → `docs/DESIGN.md`
- [x] Google setup guide → `docs/GOOGLE_SETUP.md`
- [x] `CLAUDE.md`, `ROADMAP.md`, `LICENSE-MIT`, `LICENSE-APACHE`

## M1 — Workspace & core ✅

- [x] Convert to Cargo workspace: `crates/keepsave-core`, `crates/keepsave-sync`, `crates/keepsave-app`
- [x] Workspace metadata: `license = "MIT OR Apache-2.0"`, repository URL, edition 2024
- [x] Core domain model (vault, group, entry, custom fields) over the `keepass` crate
- [x] Create / open / save KDBX4 with Argon2id, plus KDF calibration (~1 s, 64 MiB)
- [x] Recycle bin and entry history
- [x] Password generator (character + passphrase modes, entropy)
- [x] Unit tests: KDBX round-trip and generator
- [x] CI workflow (fmt, clippy, test on Windows)

## M2 — UI foundation ✅

- [x] eframe app with the wgpu backend
- [x] Glass theme: tokens, panels, borders, highlights, shadows
- [x] Animated blurred background shader
- [x] Inter font and Phosphor icons
- [x] Fluent i18n (es, en) with system locale detection
- [x] Welcome and Unlock screens; Argon2 runs on a worker thread
- [x] Create-vault form with strength meter; `config.toml` with recent vaults

## M3 — Main screen ✅

- [x] Sidebar with groups, entry list, sliding detail panel
- [x] Entry CRUD, custom fields, reveal toggle
- [x] Search (`Ctrl+F`)
- [x] Clipboard copy with auto-clear
- [x] Keyboard shortcuts (KeePass set)
- [x] Autosave
- [x] Auto-lock (inactivity, minimize, OS sleep / session lock)
- [x] Inline password generator, entry history with restore, recycle bin restore

## M4 — Google Drive connection ⬜

- [ ] OAuth (loopback + PKCE); refresh token stored in `keyring`
- [ ] `RemoteStore` trait and Drive REST implementation (folder, list, upload, download)
- [ ] Import-from-Drive flow (new PC)
- [ ] Link a vault to a Drive `fileId` in `config.toml`

## M5 — Sync & merge ⬜

- [ ] KeePass-style merge by UUID / mtime, honoring `DeletedObjects`; losers go to history
- [ ] Sync triggers: unlock, debounced save, manual button
- [ ] Offline "pending upload" state
- [ ] Versioned backups in `KeepSave/backups/` (keep 10)
- [ ] Merge tests and sync tests with a fake `RemoteStore`

## M6 — Settings, polish, release ⬜

- [ ] Settings screen (General, Security, Google Drive, Language, Appearance)
- [ ] Optional Windows 11 Acrylic/Mica (`window-vibrancy`)
- [ ] Animation polish and performance pass
- [ ] Release profile (LTO) and portable `.exe`
- [ ] Manual check: vault opens in KeePassXC
- [ ] About screen with EFF wordlist attribution (CC BY 3.0)

## Post-v1 backlog

- Auto-type
- Attachments
- Key files
- Windows Hello quick unlock
- Entry templates
- Import from CSV and other managers
- MSI installer (`cargo-wix`) and auto-update
- Real backdrop blur
- Code signing
