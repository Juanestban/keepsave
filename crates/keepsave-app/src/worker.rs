//! Background work: a tokio runtime that runs Argon2 and file dialogs off the
//! UI thread and reports back over a channel.

use std::{
  future::Future,
  io,
  path::PathBuf,
  sync::mpsc::{self, Receiver, Sender},
};

use eframe::egui;
use keepsave_core::{CoreError, KdfParams, SecretString, Vault};
use tokio::runtime::Runtime;

/// An unlocked vault and the file it lives in.
pub struct OpenVault {
  pub vault: Vault,
  pub path: PathBuf,
}

pub enum Event {
  VaultFilePicked(Option<PathBuf>),
  NewVaultPathPicked(Option<PathBuf>),
  VaultCreated(Result<OpenVault, CoreError>),
  VaultUnlocked(Result<OpenVault, CoreError>),
  /// `generation` identifies the snapshot that was written.
  VaultSaved {
    generation: u64,
    result: Result<(), CoreError>,
  },
}

pub struct Worker {
  runtime: Runtime,
  sender: Sender<Event>,
  receiver: Receiver<Event>,
  ctx: egui::Context,
}

impl Worker {
  pub fn new(ctx: egui::Context) -> io::Result<Self> {
    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(2).thread_name("keepsave-worker").build()?;
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

  /// Writes a snapshot of the vault. Saving re-derives the key, so it takes
  /// as long as an unlock.
  pub fn save_vault(&self, vault: Vault, path: PathBuf, generation: u64) {
    self.spawn_blocking(move || Event::VaultSaved { generation, result: vault.save(&path) });
  }

  fn spawn(&self, task: impl Future<Output = Event> + Send + 'static) {
    let sender = self.sender.clone();
    let ctx = self.ctx.clone();
    self.runtime.spawn(async move {
      // The receiver only goes away when the app is closing.
      let _ = sender.send(task.await);
      ctx.request_repaint();
    });
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
