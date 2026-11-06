//! Background work: a tokio runtime that runs Argon2 and file dialogs off the
//! UI thread and reports back over a channel.

use std::{
  future::Future,
  io,
  path::PathBuf,
  sync::mpsc::{self, Receiver, Sender},
};

use std::sync::Arc;

use eframe::egui;
use keepsave_core::{CoreError, KdfParams, SecretString, Vault};
use keepsave_sync::{
  BrowserPage, Credentials, DriveClient, RemoteFile, RemoteStore, SyncError, SyncLink, SyncOutcome,
  update::{self, Release, UpdateError},
};
use tokio::{runtime::Runtime, task::AbortHandle};

use crate::{
  hello::{self, HelloError, Sealed},
  update::Progress,
};

/// An unlocked vault and the file it lives in.
pub struct OpenVault {
  pub vault: Vault,
  pub path: PathBuf,
}

/// Why a Windows Hello unlock didn't open the vault.
pub enum QuickUnlockError {
  Hello(HelloError),
  Vault(CoreError),
}

pub enum Event {
  VaultFilePicked(Option<PathBuf>),
  NewVaultPathPicked(Option<PathBuf>),
  VaultCreated(Result<OpenVault, CoreError>),
  VaultUnlocked(Result<OpenVault, CoreError>),
  QuickUnlocked(Result<OpenVault, QuickUnlockError>),
  HelloChecked(bool),
  QuickUnlockSealed(Result<Sealed, HelloError>),
  /// `generation` identifies the snapshot that was written.
  VaultSaved {
    generation: u64,
    result: Result<(), CoreError>,
  },
  DriveConnected(Result<DriveClient, SyncError>),
  DriveFilesListed(Result<Vec<RemoteFile>, SyncError>),
  DriveImportPathPicked {
    file: RemoteFile,
    path: Option<PathBuf>,
  },
  DriveDownloaded {
    path: PathBuf,
    result: Result<RemoteFile, SyncError>,
  },
  DriveUploaded {
    path: PathBuf,
    result: Result<RemoteFile, SyncError>,
  },
  DriveDisconnected,
  VaultSynced {
    path: PathBuf,
    /// The autosave generation of the snapshot that was synced.
    generation: u64,
    local_changed: bool,
    result: Result<SyncOutcome, SyncError>,
  },
  /// `Some` when a newer release is out.
  UpdateChecked(Result<Option<Release>, UpdateError>),
  UpdateDownloaded(Result<PathBuf, UpdateError>),
}

impl Event {
  /// The Drive error this event carries, if any.
  pub fn drive_error(&self) -> Option<&SyncError> {
    match self {
      Event::DriveConnected(Err(error))
      | Event::DriveFilesListed(Err(error))
      | Event::DriveDownloaded { result: Err(error), .. }
      | Event::DriveUploaded { result: Err(error), .. }
      | Event::VaultSynced { result: Err(error), .. } => Some(error),
      _ => None,
    }
  }
}

pub struct Worker {
  runtime: Runtime,
  sender: Sender<Event>,
  receiver: Receiver<Event>,
  ctx: egui::Context,
}

impl Worker {
  pub fn new(ctx: egui::Context) -> io::Result<Self> {
    // IO and timers: reqwest (Drive, updates) needs both.
    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).thread_name("keepsave-worker").enable_all().build()?;
    let (sender, receiver) = mpsc::channel();
    Ok(Self { runtime, sender, receiver, ctx })
  }

  /// Events that finished since the last call. Call once per frame.
  pub fn poll(&self) -> impl Iterator<Item = Event> + '_ {
    self.receiver.try_iter()
  }

  pub fn pick_vault_file(&self, title: String, filter: String) {
    self.spawn(async move {
      let file = rfd::AsyncFileDialog::new().set_title(title).add_filter(filter, &["kdbx"]).pick_file().await;
      Event::VaultFilePicked(file.map(|file| file.path().to_owned()))
    });
  }

  pub fn pick_new_vault_path(&self, title: String, filter: String, file_name: String) {
    self.spawn(async move {
      let file = rfd::AsyncFileDialog::new().set_title(title).add_filter(filter, &["kdbx"]).set_file_name(file_name).save_file().await;
      Event::NewVaultPathPicked(file.map(|file| with_kdbx_extension(file.path().to_owned())))
    });
  }

  /// Calibrates Argon2 for this machine, then creates and saves the vault.
  pub fn create_vault(&self, path: PathBuf, name: String, password: SecretString) {
    self.spawn_blocking(move || {
      let result = KdfParams::calibrate(KdfParams::DEFAULT_UNLOCK_TIME).and_then(|kdf| {
        let vault = Vault::create(&name, password, kdf);
        vault.save(&path)?;
        Ok(OpenVault { vault, path })
      });
      Event::VaultCreated(result)
    });
  }

  pub fn unlock_vault(&self, path: PathBuf, password: SecretString) {
    self.spawn_blocking(move || {
      let result = Vault::open(&path, password).map(|vault| OpenVault { vault, path });
      Event::VaultUnlocked(result)
    });
  }

  pub fn check_hello(&self) {
    self.spawn_blocking(|| Event::HelloChecked(hello::available()));
  }

  /// Asks for Windows Hello and keeps `password` sealed for `path`.
  pub fn seal_quick_unlock(&self, path: PathBuf, password: SecretString) {
    self.spawn_blocking(move || Event::QuickUnlockSealed(hello::seal(path, &password)));
  }

  /// Asks for Windows Hello, then unlocks with the sealed password.
  pub fn quick_unlock(&self, sealed: Sealed) {
    self.spawn_blocking(move || {
      let result = hello::unseal(&sealed).map_err(QuickUnlockError::Hello).and_then(|password| {
        let path = sealed.path.clone();
        Vault::open(&path, password).map(|vault| OpenVault { vault, path }).map_err(QuickUnlockError::Vault)
      });
      Event::QuickUnlocked(result)
    });
  }

  /// Writes a snapshot of the vault. Saving re-derives the key, so it takes
  /// as long as an unlock.
  pub fn save_vault(&self, vault: Vault, path: PathBuf, generation: u64) {
    self.spawn_blocking(move || Event::VaultSaved { generation, result: vault.save(&path) });
  }

  /// Opens the browser for Google sign-in and waits for the redirect.
  /// Abort the returned handle to cancel.
  pub fn connect_drive(&self, credentials: Credentials, pages: (BrowserPage, BrowserPage)) -> AbortHandle {
    self.spawn(async move { Event::DriveConnected(DriveClient::connect(credentials, pages, webbrowser::open).await) })
  }

  pub fn disconnect_drive(&self, client: Arc<DriveClient>) {
    self.spawn(async move {
      // The token is gone locally either way; nothing to report.
      let _ = client.disconnect().await;
      Event::DriveDisconnected
    });
  }

  pub fn list_drive(&self, client: Arc<DriveClient>) {
    self.spawn(async move { Event::DriveFilesListed(client.list_vaults().await) });
  }

  /// Asks where to save `file`, starting in `Documents\KeepSave`.
  pub fn pick_import_path(&self, title: String, filter: String, file: RemoteFile) {
    self.spawn(async move {
      let mut dialog = rfd::AsyncFileDialog::new().set_title(title).add_filter(filter, &["kdbx"]).set_file_name(file.name.clone());
      if let Some(dir) = directories::UserDirs::new().and_then(|dirs| dirs.document_dir().map(|docs| docs.join("KeepSave"))) {
        // If it can't be created, the dialog starts in its default folder.
        if std::fs::create_dir_all(&dir).is_ok() {
          dialog = dialog.set_directory(dir);
        }
      }
      let path = dialog.save_file().await.map(|handle| with_kdbx_extension(handle.path().to_owned()));
      Event::DriveImportPathPicked { file, path }
    });
  }

  pub fn download_from_drive(&self, client: Arc<DriveClient>, file: RemoteFile, path: PathBuf) {
    self.spawn(async move {
      let result = async {
        let bytes = client.download(&file.id).await?;
        write_atomically(&path, &bytes)?;
        Ok(file)
      }
      .await;
      Event::DriveDownloaded { path, result }
    });
  }

  /// Uploads the vault as a new file in the Drive folder, named like the
  /// local file. Encrypting re-derives the key, so it runs off the runtime.
  pub fn upload_to_drive(&self, client: Arc<DriveClient>, vault: Vault, path: PathBuf) {
    self.spawn(async move {
      let result = async {
        let bytes = tokio::task::spawn_blocking(move || vault.to_bytes())
          .await
          .map_err(io::Error::other)?
          .map_err(|error| io::Error::other(error.to_string()))?;
        let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| "KeepSave.kdbx".into());
        client.create(&name, bytes).await
      }
      .await;
      Event::DriveUploaded { path, result }
    });
  }

  /// Merges the Drive copy into `snapshot` and uploads what Drive lacks.
  pub fn sync_vault(&self, client: Arc<DriveClient>, snapshot: Vault, link: SyncLink, local_changed: bool, path: PathBuf, generation: u64) {
    self.spawn(async move {
      let result = keepsave_sync::sync_vault(&*client, snapshot, &link, local_changed, chrono::Utc::now()).await;
      Event::VaultSynced { path, generation, local_changed, result }
    });
  }

  pub fn check_update(&self) {
    self.spawn(async { Event::UpdateChecked(update::check(&crate::update::current_version()).await) });
  }

  /// Downloads `name` from `release` to `dest`, reporting into `progress`.
  pub fn download_update(&self, release: Release, name: String, dest: PathBuf, progress: Arc<Progress>) {
    let ctx = self.ctx.clone();
    self.spawn(async move {
      let result = update::download(&release, &name, &dest, |done, total| {
        progress.set(done, total);
        ctx.request_repaint();
      })
      .await;
      Event::UpdateDownloaded(result)
    });
  }

  fn spawn(&self, task: impl Future<Output = Event> + Send + 'static) -> AbortHandle {
    let sender = self.sender.clone();
    let ctx = self.ctx.clone();
    self
      .runtime
      .spawn(async move {
        // The receiver only goes away when the app is closing.
        let _ = sender.send(task.await);
        ctx.request_repaint();
      })
      .abort_handle()
  }

  fn spawn_blocking(&self, task: impl FnOnce() -> Event + Send + 'static) {
    let sender = self.sender.clone();
    let ctx = self.ctx.clone();
    self.runtime.spawn_blocking(move || {
      let _ = sender.send(task());
      ctx.request_repaint();
    });
  }
}

/// Writes next to `path` first, so a failed download never leaves half a
/// vault behind.
fn write_atomically(path: &std::path::Path, bytes: &[u8]) -> io::Result<()> {
  let mut tmp = path.as_os_str().to_owned();
  tmp.push(".tmp");
  std::fs::write(&tmp, bytes)?;
  std::fs::rename(&tmp, path)
}

fn with_kdbx_extension(path: PathBuf) -> PathBuf {
  let has_extension = path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("kdbx"));
  if has_extension {
    path
  } else {
    let mut name = path.into_os_string();
    name.push(".kdbx");
    name.into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn adds_missing_kdbx_extension() {
    assert_eq!(with_kdbx_extension("C:/a/vault".into()), PathBuf::from("C:/a/vault.kdbx"));
    assert_eq!(with_kdbx_extension("C:/a/vault.KDBX".into()), PathBuf::from("C:/a/vault.KDBX"));
    assert_eq!(with_kdbx_extension("C:/a/my.vault".into()), PathBuf::from("C:/a/my.vault.kdbx"));
  }
}
