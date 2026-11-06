# KeepSave — Design

KeepSave is a personal, desktop password manager written in native Rust with
`egui`/`eframe`. It reproduces KeePass behavior on top of the standard KDBX4
file format, adds a minimalist dark "liquid glass" UI, and syncs vaults
through Google Drive so they can be restored on any PC.

This document records every design decision agreed before implementation
(Q1–Q23, plus Q24–Q25 added later) and the reasoning behind each. Change a decision here first, then in
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
- Windows Hello quick unlock: see Q25. Key files come later.

### Q25 — Windows Hello quick unlock

- **Session only.** After a vault is unlocked with its master password,
  Windows Hello (face, fingerprint or PIN) can unlock it again after a lock
  (inactivity, minimize, OS sleep). The master password is still required
  after the app restarts, and for any other vault.
- **Opt-in per vault.** A checkbox under the password on the Unlock screen
  and a toggle per recent vault under Settings → Security. The list of
  enabled vaults is kept in `config.toml` (`quick_unlock`, paths only, never
  a secret). Forgetting a vault also turns it off.
- **Crypto: `KeyCredentialManager`, not `UserConsentVerifier`.** A
  KeepSave key credential (TPM-backed RSA, created on first use) signs a
  random 32-byte challenge. RSA PKCS#1 v1.5 signatures are deterministic, so
  the same challenge always yields the same signature. The AES-256-GCM key
  that encrypts the master password is SHA-256 of a domain label plus that
  signature. Only the challenge, nonce and ciphertext stay in RAM; the
  signature and derived key are wiped right after use. Without Hello
  verification nothing can decrypt the password, unlike DPAPI, which any
  process of the same user can call.
- **When it seals.** Sealing needs a Hello prompt, so it happens right after
  a password unlock of an enabled vault, or when the open vault is enabled
  in Settings. Cancelling the prompt just means the next unlock needs the
  password. One sealed vault at a time: opening another vault drops it.
- **When it is dropped.** On exit, when the vault is turned off, when another
  vault is opened, and when the sealed password no longer opens the file
  (e.g. its password was changed in another client). Each of these falls
  back to the master password.
- **Unlock screen.** For the sealed vault, a primary "Unlock with Windows
  Hello" button appears above the password field; Enter on an empty password
  field does the same. The prompt is not shown on its own: after an OS sleep
  the user may be away.
- Hello runs on the worker (its WinRT calls block). The prompt window can
  open behind ours, so a helper brings the "Credential Dialog Xaml Host"
  window to the front while it waits.
- Other platforms: hidden, as Hello is Windows only.

### Q10 — Memory hygiene and locking

| Behavior | Default | Configurable |
|---|---|---|
| Secrets in RAM wrapped with `secrecy` / `zeroize` | always | no |
| Auto-lock on inactivity | 5 min | yes |
| Lock on minimize | on | yes |
| Lock on OS sleep / session lock | on | yes |
| Clipboard auto-clear (only if it still holds our value) | 15 s | yes |
| Passwords masked by default, reveal toggle | on | — |

Implementation notes:

- The timers and toggles above are set under Settings → Security and kept
  in `config.toml` (`lock_after_minutes`, `lock_on_minimize`,
  `lock_on_os_sleep`, `clipboard_clear_seconds`).
- Copied secrets are excluded from the Windows clipboard history and cloud
  clipboard. The clipboard is also cleared on lock and on exit, again only if
  it still holds our value.
- OS sleep and session lock are detected on Windows with a hidden window
  that listens for `WM_POWERBROADCAST` and `WM_WTSSESSION_CHANGE`. Other
  platforms rely on the inactivity timer for now.
- Locking with unsaved changes saves first. If that save fails, the vault
  stays unlocked and shows the error, because losing data is worse. An entry
  still being edited is discarded on lock: the lock must not wait for the
  user.
- Closing the window with unsaved changes also saves first; if the save
  fails, the user chooses between staying and closing without saving.
- Labels are not selectable, so secrets only reach the clipboard through the
  copy actions, which auto-clear.

### Q12 — Secrets vs. settings

- The Google refresh token goes in the OS credential store (crate `keyring`:
  Windows Credential Manager, macOS Keychain, Linux Secret Service), under
  service `KeepSave`, account `google-drive`.
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
  → PKCE (S256). The flow is three HTTP requests, so it is written by hand on
  `reqwest` instead of pulling in the `oauth2` crate.
- The authorization request asks for `access_type=offline` and
  `prompt=consent`, so Google always returns a refresh token. The loopback
  listener gives up after 5 minutes and the user can cancel it.
- The access token lives only in memory. A refresh token that Google rejects
  (`invalid_grant`) disconnects Drive and asks the user to connect again.
- **Credentials (revised after v1.1):** each user brings their own OAuth
  client. A desktop app can't keep a client secret: anything in the `.exe`
  can be read back with a hex editor. Google says as much for installed
  apps, and security rests on PKCE, the loopback redirect and the user's
  consent. But a client shipped in a public release could still be reused
  to impersonate "KeepSave" on a consent screen or to burn the project's
  quota, and Google could suspend it for everyone. So:
  - KeepSave opens an in-app **setup guide** (see Q16) that walks the user
    through creating the client, then takes the client ID and secret. They
    are stored together in the OS credential store (account
    `google-client`), never in `config.toml`.
  - No build embeds a client, not even a developer's: there is no build-time
    setting for it, so a published `.exe` can't leak one by mistake. The
    developer goes through the same guide as everyone else.
  - Without a client, Drive shows "Set up Google Drive" wherever it would
    offer to connect.
  - Saving a different client forgets the refresh token, which belongs to
    the old one. Removing the client also disconnects.
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

Implementation (M5):

- **Merge engine:** the `keepass` crate's own merge (feature `_merge`),
  which follows KeePassXC. It is marked unstable, so the crate version stays
  pinned by `Cargo.lock` and our merge tests guard upgrades. After a merge
  KeepSave trims every entry's history back to the vault's limit (10).
  Attachments are not merged yet (they are post-v1 anyway).
- **One sync:**
  1. Read the Drive file's metadata. A trashed or missing file is an error;
     the user can unlink the vault.
  2. If the remote `version` changed since the last sync, download it,
     decrypt it with the vault's master password and merge it into a
     snapshot of the local vault. A different master password on Drive is
     an error, never a silent overwrite.
  3. If the local vault has changes Drive doesn't have yet, copy the Drive
     file to `KeepSave/backups/` (server-side copy named
     `<vault name> <UTC timestamp>.kdbx`), upload the merged snapshot over
     it, and delete the oldest backups of that vault beyond 10.
  4. The open vault then merges the remote copy too, so edits made while the
     sync ran are kept; autosave writes the result locally.
- **Pending upload:** each link in `config.toml` has a `pending` flag, set
  on the first local change after a sync and cleared once Drive has it. It
  survives restarts, so changes made offline are uploaded at the next
  unlock.
- **Timing:** a sync runs when a linked vault opens (or Drive connects), 3 s
  after a save that left changes pending, and on demand. After a network
  error it retries every 60 s while the vault is open; other errors wait for
  the next change or a manual retry.
- **Results that arrive after locking are dropped,** and the link keeps its
  old `version`. The next sync then downloads and merges our own upload
  again, which is harmless; updating the version without merging could lose
  remote changes.
- **Known gap:** Drive v3 has no conditional update, so two PCs uploading in
  the same second can race. The loser's version is still in
  `KeepSave/backups/`.

### Drive in the UI

Settings → Google Drive manages the connection and lists every linked
vault. Day-to-day, Drive also lives in the main screen's sidebar, above the
recycle bin:

- **Not connected:** "Connect Google Drive". While the browser is open, a
  spinner and a Cancel action.
- **Connected, vault not linked:** "Upload to Drive" creates the file in
  `KeepSave/` and links the vault.
- **Linked:** "On Google Drive". Its menu unlinks the vault or disconnects
  Drive.

In M4 KeepSave only created files on Drive and downloaded them. From M5 a
linked vault syncs (Q13), and the sidebar row shows the sync state: synced
(with the time), pending upload, syncing, waiting for the network, or the
error. Clicking it syncs now; its menu also unlinks or disconnects.

Links are kept in `config.toml` as `[[drive_links]]` entries with the local
path, the Drive `file_id`, the `version` and `modified_time` last seen
(to detect remote changes) and the `pending` upload flag. Disconnecting keeps the links, so
reconnecting resumes them.

### Q17 — "New PC" flow

1. Welcome → **Import from Google Drive**
2. OAuth
3. List the `.kdbx` files in `KeepSave/`
4. Pick one and choose where to save it (a save dialog that starts in
   `Documents\KeepSave\`, created if missing)
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
- **Dropped in M6:** the optional Windows 11 Mica backdrop ("System
  transparency"). It needed a transparent window on DX12 with a
  DirectComposition swapchain, applied only at startup, and it rendered
  wrongly on the real screen, so it was removed rather than shipped broken.
  The simulated glass is the only look.
- **Future:** real in-app backdrop blur (framebuffer copy and blur pass), only
  if the simulated look falls short.
- Renderer: eframe **wgpu** backend.

### Q15 — i18n and assets

- Spanish and English from v1. Fluent `.ftl` files loaded with
  `fluent-bundle`.
- Default language from the OS (`sys-locale`), with a selector in
  Settings → Language: System, Español, English. "System" stores no
  language in `config.toml`, so the OS choice applies again.
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
   - **Sidebar:** vault name, the root group, its child groups (one level),
     and the recycle bin pinned at the bottom. A right-click menu renames or
     deletes a group, or empties the bin.
   - **Top bar** (over the list): search, New entry, save status, Lock.
   - **List:** title and username per row, with copy buttons on hover. While
     searching, results span all groups (except the bin) and show their group.
     The root lists its own entries; any other group, and the bin, also list
     the entries of deeper groups (which the sidebar doesn't show), labeled
     with their group.
   - **Detail:** the side panel shows an entry, with copy, reveal and
     open-URL actions, tags, custom fields and history with restore.
   - **Editor:** creating or editing an entry opens a wide modal (up to
     780 px) so the fields have room: Title | Username, Password | URL,
     the inline generator full width, Group | Tags, Notes, then custom
     fields one per row (name, value, protect, remove). Cancel and Save sit
     at the bottom right. Clicking outside, Esc or Cancel asks before
     discarding changes.
   - On narrow windows the detail panel overlaps the list instead of
     squeezing it.
   - **Autosave:** every change is saved on the worker about 1 s after the
     last edit (`Ctrl+S` saves at once). The top bar shows Saved, Saving or
     the error.
   - Deleting moves to the recycle bin; deleting from the bin and emptying it
     ask for confirmation. Entries in the bin can be restored to the root.
4. **Settings** (M6): a modal over whatever screen is showing, so an open
   vault stays open beneath it. A gear button opens it: top right on the
   screens before unlocking, and in the main top bar. Esc or a click
   outside closes it. Every change applies and is saved at once; there is
   no Save button. Tabs on the left:
   - **General:** recent vaults (remove one or clear the list). If
     `config.toml` can't be written, the error shows here.
   - **Security:** auto-lock after (off, 1, 2, 5, 10, 15, 30 or 60 min),
     lock on minimize, lock on OS sleep or session lock, clear the
     clipboard after (off, 10, 15, 30, 60 or 90 s).
   - **Google Drive:** connection state with Connect, Cancel or
     Disconnect; which OAuth client is in use (yours, or the one built in)
     with *Change* (reopens the guide on its last step) and *Remove*; and
     the linked vaults (name on Drive, local path, pending upload) with
     Unlink.
   - **Language:** System, Español, English.
   - **Appearance:** animated background (off draws a still frame and stops
     the constant repaint).
   - **About:** version, update status with *Check now* and the automatic
     check toggle (Q24), licenses, repository link and third-party credits:
     Inter (SIL OFL 1.1), Phosphor icons (MIT) and the EFF large wordlist
     (CC BY 3.0).
5. **Password generator** (M6): a button in the main top bar opens the
   generator (Q18) in a modal. Its action copies the password, with the
   same auto-clear as any other copied secret.
6. **Google Drive setup guide** (after v1.1): a modal stepper, opened by
   any "Set up Google Drive" action. A row of numbered dots shows progress;
   Back and Next move between steps. Steps follow `GOOGLE_SETUP.md`:
   intro (what it is and why it is private), create a project, enable the
   Drive API, branding (consent screen), data access (the `drive.file`
   scope), publish the app (In production), create a Desktop client, and
   paste the client ID and secret. Each step has short numbered
   instructions, an *Open in Google Cloud* button with a deep link, chips
   that copy the exact values to type (`KeepSave`, the scope URL, the
   client name), and a tip where Google's console is confusing. The last
   step checks the ID's format (`….apps.googleusercontent.com`) and offers
   *Save and connect*.

### Q18 — Password generator

- **Character mode:**
  - length 8–128, default 20;
  - upper case, lower case, digits and symbols;
  - optional exclusion of ambiguous characters (`0O1lI`);
  - guarantees at least one character from each enabled class.
- **Passphrase mode:** EFF large wordlist, default 5 words, configurable
  separator.
- Entropy meter (bits). Randomness from `OsRng`.
- Available inline while editing an entry (a panel under the password
  field, M3) and as a standalone tool from the main top bar (M6).

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
| `↑` / `↓` | Move through the entry list |
| `Enter` | Edit the selected entry |
| `Delete` | Delete the selected entry |
| `Esc` | Cancel editing, clear the search, or close the detail panel |

The copy and open shortcuts act on the selected entry and only when no text
field has focus, so `Ctrl+C` still copies text inside fields. URLs open only
with `http` or `https`; a URL without a scheme gets `https://`.

## 5. Engineering

### Q14 — Architecture

The project is a Cargo workspace:

```
crates/
  keepsave-core/   # model, KDBX I/O, merge, generator — no UI, no network
  keepsave-sync/   # OAuth + minimal Drive REST client (reqwest, hand-written OAuth)
  keepsave-app/    # eframe/egui binary: theme, animations, i18n, screens
```

- Network and Argon2 work run on a `tokio` runtime in a background thread and
  talk to the UI over channels, so the UI thread never blocks.
- `keepsave-sync` puts Drive behind a `RemoteStore` trait so it can be tested
  without the network.
- `keepsave-sync` also holds the update check and download (Q24): it is the
  crate that owns the HTTP client.
- `google-drive3` is not used; the few REST endpoints we need are written by
  hand.

### Q20 — Packaging

- v1 (1.0.0): a single portable `.exe`, assets embedded, release profile
  with LTO.
- **Icon:** an original design: a rounded square with the accent-to-violet
  gradient of the theme, a glass highlight, and a light keyhole. It is drawn
  by `scripts/make_icon.py` (Python standard library only), which writes
  `assets/icon/keepsave.ico` (16–256 px) and a raw 128 px RGBA copy. The
  `.ico` goes into the `.exe` resources with `winresource` (with the
  version info); the RGBA copy is the window and taskbar icon.
- **MSI installer:** a per-user package written for WiX v5
  (`crates/keepsave-app/installer/keepsave.wxs`, built by
  `scripts/package.ps1`). It installs `keepsave.exe` to
  `%LOCALAPPDATA%\Programs\KeepSave` with a Start menu shortcut, needs no
  administrator rights, and upgrades in place (fixed `UpgradeCode`, major
  upgrade). No wizard: it installs with a progress bar, which also suits
  silent updates. `cargo-wix` was dropped because it targets WiX v3 and
  machine-wide installs.
- **Releases:** `.github/workflows/release.yml` runs on a `v*` tag. It checks
  that the tag matches the workspace version, builds the release `.exe`
  without a Google client (see Q11), builds the MSI, and
  publishes a GitHub Release with `keepsave-<version>-x64.msi`,
  `keepsave-<version>-x64-portable.exe` and `SHA256SUMS.txt`. A tag with a
  pre-release suffix (`v1.1.0-rc.1`) makes a pre-release, which the updater
  ignores.
- No code signing yet, so SmartScreen warns on first run.

### Q24 — Updates

- **Check:** the app asks GitHub for the latest release
  (`/repos/Juanestban/keepsave/releases/latest`, no token; drafts and
  pre-releases are skipped) at startup and every 6 hours while running. It
  compares the tag with its own version as semver. Settings → About shows
  the result, has *Check now*, and a toggle *Check for updates
  automatically* (on by default; off means only *Check now* contacts
  GitHub). The request carries only the `KeepSave/<version>` user agent.
- **Banner:** a newer version shows a glass card at the bottom right, over
  every screen: the new version, *Update*, *What's new* (the release page)
  and a close button that hides it until the next start. *Update*
  downloads in place with a progress bar; when it is ready the card offers
  *Restart to update*. Errors show in the card with *Retry* and a link to
  the release page.
- **Integrity:** the download is hashed while it streams and must match its
  line in the release's `SHA256SUMS.txt`; a release without it can't be
  installed from the app. Without code signing this guards against broken
  downloads, not against a compromised GitHub account.
- **Install:** an MSI install (the app runs from
  `%LOCALAPPDATA%\Programs\KeepSave`) downloads the MSI and hands it to a
  hidden PowerShell that waits for KeepSave to exit, runs
  `msiexec /i … /passive` and starts the new version. A portable copy
  downloads the portable `.exe`, renames itself to `keepsave.exe.old`, puts
  the new file in its place and starts it; the next start deletes the old
  file. If the folder isn't writable, the card says so and links to the
  release page.
- **Restart with a vault open:** the app locks first, which saves pending
  changes; it restarts only once the vault is locked. A failed save cancels
  the restart and shows the usual error.
- Off Windows, *Update* opens the release page instead.

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
