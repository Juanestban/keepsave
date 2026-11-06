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

## M4 — Google Drive connection 🟨

Code complete; waiting for an end-to-end check against a real Google account.

- [x] OAuth (loopback + PKCE); refresh token stored in `keyring`
- [x] `RemoteStore` trait and Drive REST implementation (folder, list, upload, download)
- [x] Import-from-Drive flow (new PC)
- [x] Link a vault to a Drive `fileId` in `config.toml`
- [ ] Manual check with real credentials: connect, upload, import on another PC, disconnect

## M5 — Sync & merge 🟨

Code complete; waiting for a check with two PCs (or two vault copies) on a real account.

- [x] KeePass-style merge by UUID / mtime, honoring `DeletedObjects`; losers go to history
- [x] Sync triggers: unlock, debounced save, manual button
- [x] Offline "pending upload" state
- [x] Versioned backups in `KeepSave/backups/` (keep 10)
- [x] Merge tests and sync tests with a fake `RemoteStore`
- [ ] Manual check: edit on two PCs, sync both, confirm nothing is lost

## M6 — Settings, polish, release 🟨

- [x] Settings screen (General, Security, Google Drive, Language, Appearance)
- [x] ~~Optional Windows 11 Mica~~ dropped: it rendered wrongly on screen (see DESIGN.md Q8)
- [x] Animation polish and performance pass (still background option, no repaint while minimized)
- [x] Release profile (LTO) and portable `.exe` (about 20 MB, assets embedded)
- [ ] Manual check: vault opens in KeePassXC
- [x] About screen with EFF wordlist attribution (CC BY 3.0)
- [x] Standalone password generator from the main top bar
- [x] App icon (in the `.exe` and the window) and version 1.0.0

## M7 — Installer & updates ✅

- [x] Release workflow: tag `v*` → checks, build, GitHub Release
- [x] Per-user MSI (WiX v5) and `scripts/package.ps1` (MSI, portable `.exe`, `SHA256SUMS.txt`)
- [x] Update check at startup and every 6 h (Settings → About: status, Check now, automatic toggle)
- [x] Update banner: download with progress, SHA-256 check, restart into the new version (MSI or portable)
- [x] Fix: the worker runtime had no IO/timer driver, so every network task panicked
- [x] Version 1.1.0
- [ ] Manual check: install the MSI, then update from one release to the next

## M8 — Bring your own Google client ✅

- [x] Users set up their own OAuth client in the app; it is kept in the OS credential store
- [x] In-app setup guide: stepper with deep links, copyable values and tips, ending in Save and connect
- [x] Settings → Google Drive: the client in use, Change and Remove
- [x] Drive actions show "Set up Google Drive" instead of hiding when there is no client
- [x] No build embeds a client: the `.env` build script is gone
- [x] Version 1.2.0
- [ ] Manual check: go through the guide with a real Google account and sync a vault

## Post-v1 backlog

- Auto-type
- Attachments
- Key files
- Windows Hello quick unlock
- Entry templates
- Import from CSV and other managers
- Real backdrop blur
- Code signing
