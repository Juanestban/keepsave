# KeepSave — Design

KeepSave is a personal, desktop password manager written in native Rust with
`egui`/`eframe`. It reproduces KeePass behavior on top of the standard KDBX4
file format, adds a minimalist dark "liquid glass" UI, and syncs vaults
through Google Drive so they can be restored on any PC.

This document records every design decision agreed before implementation
(Q1–Q23) and the reasoning behind each. Change a decision here first, then in
code.

## 1. Scope

### Q1 — v1 feature set

**In v1:**

- Unlock with a master password
- Groups (one nesting level shown in the UI; the KDBX model supports more)
- Entries: title, username, password, URL, notes, custom fields
- Search
- Password generator (see Q18)
- Copy to clipboard with auto-clear after 15 s
- Auto-lock on inactivity
- Recycle bin and per-entry history
- Google Drive backup and bidirectional sync

**Deferred (post-v1):** auto-type, attachments, key files, entry templates,
CSV/KDBX import from other managers, Windows Hello quick unlock, MSI
installer, auto-update.

**Why:** "everything KeePass does" is too large for a first release. v1 covers
daily use; the deferred list is additive and does not affect the file format.

### Q3 — Users

Single person, multiple vaults, each with its own master password. No
accounts, roles or shared vaults.

### Q4 — Platforms

Desktop only: Windows first, macOS and Linux supported (nearly free with
eframe). Mobile access is covered by opening the same `.kdbx` with existing
apps (KeePassDX on Android, Strongbox/KeePassium on iOS).

**Why:** egui on mobile is experimental and lacks autofill and keyboard
integration.

## 2. Storage and security

### Q2 — File format: KDBX4

Vaults are standard KDBX4 files, read and written with the `keepass` crate.

**Why:** they stay interoperable with KeePassXC and mobile clients, the format
is audited, and a vault is a single encrypted file that is easy to sync. We
considered SQLCipher and field-level encrypted SQLite and rejected them: both
are proprietary formats, SQLCipher complicates Windows builds, and field-level
encryption leaks metadata.

### Q9 — Key derivation and unlock

- KDF: Argon2id. Parameters are calibrated when a vault is created so that
  unlock takes about 1 s on the current machine, with about 64 MiB of memory.
- v1 unlocks with the master password only.
- Key files and Windows Hello quick unlock come later.

### Q10 — Memory hygiene and locking

| Behavior | Default | Configurable |
|---|---|---|
| Secrets in RAM wrapped with `secrecy` / `zeroize` | always | no |
| Auto-lock on inactivity | 5 min | yes |
| Lock on minimize | on | yes |
| Lock on OS sleep / session lock | on | yes |
| Clipboard auto-clear (only if it still holds our value) | 15 s | yes |
| Passwords masked by default, reveal toggle | on | — |

### Q12 — Secrets vs. settings

- The Google refresh token goes in the OS credential store (crate `keyring`:
  Windows Credential Manager, macOS Keychain, Linux Secret Service).
- Settings go in `config.toml` under the platform config dir (crate
  `directories`), e.g. `%APPDATA%\KeepSave\config.toml`. That covers language,
  recent vaults, lock timers, appearance, and the Drive `fileId` linked to
  each vault.

## 3. Google Drive

### Q5 — Model: bidirectional sync + versioned backups

The vault syncs both ways, with a KeePass-style merge on conflict. A versioned
backup copy is kept in a separate folder on every upload.

### Q11 — OAuth

- The user's own Google Cloud project with an OAuth client of type
  **Desktop app**.
- Flow: system browser → loopback redirect (`http://127.0.0.1:<random port>`)
  → PKCE (`oauth2` crate).
- Scope: `https://www.googleapis.com/auth/drive.file`. The app sees only files
  it created, in a visible `KeepSave/` folder, so mobile KeePass apps can open
  the same file.
- Publish the consent screen as **In production**, unverified. While a project
  is in *Testing*, refresh tokens expire after 7 days. `drive.file` is a
  non-sensitive scope, so it needs no heavy verification; the user accepts the
  "unverified app" warning once.
- Setup guide: [GOOGLE_SETUP.md](GOOGLE_SETUP.md).

### Q13 — Sync behavior

- **Triggers:** unlocking a linked vault, after each save (3 s debounce), and
  a manual "Sync now" button.
- **Change detection:** compare Drive's `version` / `modifiedTime` with the
  last synced values stored in `config.toml`.
- **Offline:** everything works locally and the vault is marked
  *pending upload*.
- **Before overwriting remote:** copy the remote file to `KeepSave/backups/`
  and keep the latest 10.
- **Merge:** by entry/group UUID and last-modification time. KDBX
  `DeletedObjects` is honored so deletions propagate, and the losing version
  of an entry is pushed into that entry's history, so nothing is lost.

### Q17 — "New PC" flow

1. Welcome → **Import from Google Drive**
2. OAuth
3. List the `.kdbx` files in `KeepSave/`
4. Pick one and choose a local folder (default `Documents\KeepSave\`)
5. Download and unlock
6. The vault is now linked: its `fileId` is saved in `config.toml`

## 4. User interface

### Q6 — Visual style

A minimalist, elegant dark theme with **liquid glass** surfaces. It takes no
branding or assets from any existing product.

### Q8 — Glass implementation

- **Base: simulated glass.**
  - A slowly animated background of large blurred color blobs, rendered with a
    custom `wgpu` shader through an eframe paint callback.
  - Panels on top: translucent fill, thin luminous border, specular highlight
    on the top edge, soft shadow.
  - Because the background is already blurred, the panels read as glass with
    no real backdrop blur.
- **Optional:** Windows 11 Acrylic/Mica through `window-vibrancy`, as the
  "System transparency" toggle in Settings.
- **Future:** real in-app backdrop blur (framebuffer copy and blur pass), only
  if the simulated look falls short.
- Renderer: eframe **wgpu** backend.

### Q15 — i18n and assets

- Spanish and English from v1. Fluent `.ftl` files loaded with
  `fluent-bundle`.
- Default language from the OS (`sys-locale`), with a selector in Settings.
  Until Settings exists, the Welcome screen has an Español | English switch.
- Font: Inter (Regular and SemiBold, SIL OFL 1.1), embedded.
- Icons: Phosphor (`egui-phosphor`), drawn from a font family that holds only
  Phosphor, because Inter has its own glyphs in part of the same private-use
  range.
- `egui-phosphor` 0.14 targets egui 0.36, so eframe stays on 0.36 until it
  catches up.

### Q16 — Screens

1. **Welcome** (first run): Create vault, Open .kdbx, Import from Google Drive.
   - **Create vault** is a form: name, master password with a strength meter,
     and confirmation. A save dialog then picks the file. Argon2 calibration
     and the first save run on the worker.
2. **Unlock**: recent vaults as glass cards, master password field, subtle
   animation while Argon2 runs. A wrong password shakes the card. The app
   opens here when there are recent vaults, otherwise on Welcome.
3. **Main**: glass sidebar with groups, entry list in the center, detail panel
   that slides in from the right, and a persistent search bar (`Ctrl+F`).
4. **Settings**: General, Security, Google Drive, Language, Appearance.

### Q18 — Password generator

- **Character mode:**
  - length 8–128, default 20;
  - upper case, lower case, digits and symbols;
  - optional exclusion of ambiguous characters (`0O1lI`);
  - guarantees at least one character from each enabled class.
- **Passphrase mode:** EFF large wordlist, default 5 words, configurable
  separator.
- Entropy meter (bits). Randomness from `OsRng`.
- Available inline while editing an entry and as a standalone tool.

### Q19 — Keyboard shortcuts (KeePass-compatible)

| Shortcut | Action |
|---|---|
| `Ctrl+B` | Copy username |
| `Ctrl+C` | Copy password |
| `Ctrl+U` | Open URL |
| `Ctrl+N` | New entry |
| `Ctrl+L` | Lock |
| `Ctrl+F` | Search |
| `Ctrl+S` | Save (autosave also runs after every change) |

## 5. Engineering

### Q14 — Architecture

The project is a Cargo workspace:

```
crates/
  keepsave-core/   # model, KDBX I/O, merge, generator — no UI, no network
  keepsave-sync/   # OAuth + minimal Drive REST client (reqwest + oauth2)
  keepsave-app/    # eframe/egui binary: theme, animations, i18n, screens
```

- Network and Argon2 work run on a `tokio` runtime in a background thread and
  talk to the UI over channels, so the UI thread never blocks.
- `keepsave-sync` puts Drive behind a `RemoteStore` trait so it can be tested
  without the network.
- `google-drive3` is not used; the few REST endpoints we need are written by
  hand.

### Q20 — Packaging

- v1: a single portable `.exe`, assets embedded, release profile with LTO.
- Later: MSI installer (`cargo-wix`) and auto-update.
- No code signing yet, so SmartScreen warns on first run.

### Q21 — Tests and CI

- **core:** unit tests for merge (conflicts, deletions, history) and KDBX
  read/write round-trips. A manual check confirms the files open in KeePassXC.
- **sync:** tests against a fake `RemoteStore`.
- **CI:** GitHub Actions on Windows running `cargo fmt --check`,
  `cargo clippy -D warnings` and `cargo test`.

### Q22 — Milestones

See [../ROADMAP.md](../ROADMAP.md).

### Q23 — Documentation

This file and `GOOGLE_SETUP.md` are the source of truth for design decisions.

## License

Dual-licensed under MIT or Apache-2.0, at your option.
