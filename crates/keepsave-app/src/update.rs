//! Update checks, the download, the restart into the new version, and the
//! banner that offers them (DESIGN.md Q24).

use std::{
  io,
  path::{Path, PathBuf},
  sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
  },
  time::{Duration, Instant},
};

use eframe::egui::{self, Align, Align2, CornerRadius, Frame, Id, Label, Layout, Margin, Order, RichText, Stroke, Ui, Vec2};
use egui_phosphor::regular as icon;
use fluent_bundle::FluentArgs;
use keepsave_sync::update::{Release, UpdateError};

use crate::{
  i18n::I18n,
  screens::Nav,
  theme,
  widgets::{ActionButton, icon_button, icon_link, notice},
  worker::Worker,
};

/// How often to look for a new version while the app runs.
const CHECK_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);
const BANNER_WIDTH: f32 = 340.0;

pub fn current_version() -> semver::Version {
  semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("the crate version is semver")
}

/// Bytes downloaded so far, shared with the worker.
#[derive(Default)]
pub struct Progress {
  done: AtomicU64,
  total: AtomicU64,
}

impl Progress {
  pub fn set(&self, done: u64, total: u64) {
    self.done.store(done, Ordering::Relaxed);
    self.total.store(total, Ordering::Relaxed);
  }

  /// 0..=1, or 0 before the size is known.
  fn fraction(&self) -> f32 {
    let total = self.total.load(Ordering::Relaxed);
    if total == 0 { 0.0 } else { (self.done.load(Ordering::Relaxed) as f64 / total as f64).min(1.0) as f32 }
  }
}

pub enum State {
  Idle,
  Checking,
  UpToDate,
  CheckFailed(String),
  Available(Release),
  Downloading { release: Release, progress: Arc<Progress> },
  Ready { release: Release, file: PathBuf },
  Failed { release: Release, error: String },
}

impl State {
  fn release(&self) -> Option<&Release> {
    match self {
      State::Available(release) | State::Downloading { release, .. } | State::Ready { release, .. } | State::Failed { release, .. } => Some(release),
      _ => None,
    }
  }
}

pub struct Updater {
  state: State,
  /// When the last check started.
  last_check: Option<Instant>,
  /// The banner was closed; it comes back for a newer version.
  dismissed: bool,
  /// The version the banner last offered.
  offered: Option<semver::Version>,
  /// Restart once the vault is locked.
  restart_pending: bool,
}

impl Updater {
  pub fn new() -> Self {
    cleanup();
    Self::delete_old_exe_later();
    Self { state: State::Idle, last_check: None, dismissed: false, offered: None, restart_pending: false }
  }

  /// The `.exe` a portable update replaced. It belongs to the previous
  /// version, which may still be exiting, so wait a little.
  fn delete_old_exe_later() {
    std::thread::spawn(|| {
      std::thread::sleep(Duration::from_secs(10));
      if let Ok(exe) = std::env::current_exe() {
        let _ = std::fs::remove_file(sibling(&exe, ".old"));
      }
    });
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn restart_pending(&self) -> bool {
    self.restart_pending
  }

  /// Starts the automatic check when it is due. Call once per frame.
  pub fn tick(&mut self, ctx: &egui::Context, worker: &Worker, automatic: bool) {
    if !automatic || !matches!(self.state, State::Idle | State::UpToDate | State::CheckFailed(_)) {
      return;
    }
    match self.last_check.map(|at| at.elapsed()) {
      Some(elapsed) if elapsed < CHECK_INTERVAL => ctx.request_repaint_after(CHECK_INTERVAL - elapsed),
      _ => self.check(worker),
    }
  }

  /// Checks now, unless an update is already on its way.
  pub fn check(&mut self, worker: &Worker) {
    if matches!(self.state, State::Checking | State::Downloading { .. } | State::Ready { .. }) {
      return;
    }
    self.last_check = Some(Instant::now());
    self.state = State::Checking;
    worker.check_update();
  }

  pub fn checked(&mut self, result: Result<Option<Release>, UpdateError>, i18n: &I18n) {
    if !matches!(self.state, State::Checking) {
      return;
    }
    self.state = match result {
      Ok(Some(release)) => {
        if self.offered.as_ref() != Some(&release.version) {
          self.offered = Some(release.version.clone());
          self.dismissed = false;
        }
        State::Available(release)
      }
      Ok(None) => State::UpToDate,
      Err(error) => State::CheckFailed(error_message(i18n, &error)),
    };
  }

  /// Downloads the file for this kind of install. Off Windows, opens the
  /// release page instead.
  pub fn download(&mut self, worker: &Worker) {
    let release = match &self.state {
      State::Available(release) | State::Failed { release, .. } => release.clone(),
      _ => return,
    };
    let Some(install) = Install::current() else {
      let _ = webbrowser::open(&release.page_url);
      return;
    };
    let (name, dest) = install.download_target(&release);
    let progress = Arc::new(Progress::default());
    worker.download_update(release.clone(), name, dest, progress.clone());
    self.state = State::Downloading { release, progress };
  }

  pub fn downloaded(&mut self, result: Result<PathBuf, UpdateError>, i18n: &I18n) {
    let State::Downloading { release, .. } = std::mem::replace(&mut self.state, State::Idle) else {
      return;
    };
    self.state = match result {
      Ok(file) => State::Ready { release, file },
      Err(error) => State::Failed { release, error: error_message(i18n, &error) },
    };
  }

  /// Asks for a restart; the app calls [`Self::restart`] once no vault is
  /// open.
  pub fn request_restart(&mut self) {
    self.restart_pending = matches!(self.state, State::Ready { .. });
  }

  pub fn cancel_restart(&mut self) {
    self.restart_pending = false;
  }

  /// Hands over to the new version. On success the app must close now.
  pub fn restart(&mut self, i18n: &I18n) -> bool {
    self.restart_pending = false;
    let State::Ready { release, file } = &self.state else {
      return false;
    };
    let result = Install::current().ok_or_else(|| io::Error::other("unsupported platform")).and_then(|install| install.apply(file));
    match result {
      Ok(()) => true,
      Err(error) => {
        let error = error_message(i18n, &UpdateError::Io(error));
        self.state = State::Failed { release: release.clone(), error };
        false
      }
    }
  }

  /// The card at the bottom right while an update is on offer.
  pub fn banner(&mut self, ctx: &egui::Context, i18n: &I18n) -> Option<Nav> {
    let visible = !self.dismissed && self.state.release().is_some();
    let shown = ctx.animate_bool_with_time(Id::new("update-banner"), visible, 0.25);
    if !visible {
      return None;
    }
    let tr = |key| i18n.tr(key);
    let restart_pending = self.restart_pending;
    let release = self.state.release()?.clone();
    let mut nav = None;
    let mut dismiss = false;

    let frame = Frame::new()
      .fill(theme::GLASS_FILL_SOLID)
      .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
      .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
      .inner_margin(Margin::same(16))
      .shadow(egui::Shadow { offset: [0, 8], blur: 24, spread: 0, color: theme::SHADOW });
    egui::Area::new(Id::new("update-banner-area"))
      .order(Order::Foreground)
      .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-16.0, -16.0 + (1.0 - shown) * 16.0))
      .show(ctx, |ui| {
        ui.multiply_opacity(shown);
        frame.show(ui, |ui| {
          ui.set_width(BANNER_WIDTH);
          ui.spacing_mut().item_spacing.y = 10.0;
          let title = if matches!(self.state, State::Ready { .. }) { "update-ready" } else { "update-available" };
          let mut args = FluentArgs::new();
          args.set("version", release.version.to_string());
          args.set("current", env!("CARGO_PKG_VERSION"));
          header(ui, &tr(title), &i18n.tr_args("update-versions", &args), &tr("update-close"), &mut dismiss);

          match &self.state {
            State::Available(_) => {
              if actions(ui, ActionButton::new(tr("update-install")).icon(icon::DOWNLOAD_SIMPLE).primary(), &tr("update-notes"), &release.page_url) {
                nav = Some(Nav::DownloadUpdate);
              }
            }
            State::Downloading { progress, .. } => {
              let fraction = progress.fraction();
              progress_bar(ui, fraction);
              let percent = (fraction * 100.0).round() as u32;
              ui.label(RichText::new(i18n.tr_arg("update-downloading", "percent", percent)).small().color(theme::TEXT_MUTED));
            }
            State::Ready { .. } => {
              let button = ActionButton::new(tr("update-restart")).icon(icon::ARROW_CLOCKWISE).primary().loading(restart_pending);
              if ui.add(button).clicked() {
                nav = Some(Nav::RestartToUpdate);
              }
              ui.add(Label::new(RichText::new(tr("update-restart-hint")).small().color(theme::TEXT_FAINT)).wrap());
            }
            State::Failed { error, .. } => {
              notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
              if actions(ui, ActionButton::new(tr("update-retry")).icon(icon::ARROW_CLOCKWISE), &tr("update-open-page"), &release.page_url) {
                nav = Some(Nav::DownloadUpdate);
              }
            }
            _ => {}
          }
        });
      });
    if dismiss {
      self.dismissed = true;
    }
    nav
  }
}

/// Icon, title, versions and the close button.
fn header(ui: &mut Ui, title: &str, versions: &str, close: &str, dismiss: &mut bool) {
  ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), 40.0), Layout::right_to_left(Align::Center), |ui| {
    if icon_button(ui, icon::X, close).clicked() {
      *dismiss = true;
    }
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
      ui.label(RichText::new(icon::ARROW_CIRCLE_UP).font(theme::icon_font(26.0)).color(theme::ACCENT));
      ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.label(RichText::new(title).font(theme::heading(15.0)).color(theme::TEXT));
        ui.label(RichText::new(versions).small().color(theme::TEXT_MUTED));
      });
    });
  });
}

/// A button and, beside it, a link that opens the release page. Returns
/// whether the button was clicked.
fn actions(ui: &mut Ui, button: ActionButton, link: &str, page_url: &str) -> bool {
  ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 14.0;
    let clicked = ui.add(button).clicked();
    if icon_link(ui, icon::ARROW_SQUARE_OUT, link).clicked() {
      let _ = webbrowser::open(page_url);
    }
    clicked
  })
  .inner
}

fn progress_bar(ui: &mut Ui, fraction: f32) {
  let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 6.0), egui::Sense::hover());
  let radius = CornerRadius::same(3);
  ui.painter().rect_filled(rect, radius, theme::INPUT_FILL);
  let mut filled = rect;
  filled.set_width(rect.width() * fraction);
  ui.painter().rect_filled(filled, radius, theme::ACCENT);
}

/// How this copy of KeepSave was installed, which decides what to download.
enum Install {
  /// From the MSI, in `%LOCALAPPDATA%\Programs\KeepSave`.
  Msi,
  /// Anywhere else: the `.exe` replaces itself.
  Portable(PathBuf),
}

impl Install {
  #[cfg(windows)]
  fn current() -> Option<Self> {
    let exe = std::env::current_exe().ok()?;
    let installed = directories::BaseDirs::new().map(|dirs| dirs.data_local_dir().join("Programs").join("KeepSave"));
    let in_install_dir = installed.is_some_and(|dir| exe.parent().is_some_and(|parent| same_path(parent, &dir)));
    Some(if in_install_dir { Install::Msi } else { Install::Portable(exe) })
  }

  #[cfg(not(windows))]
  fn current() -> Option<Self> {
    None
  }

  /// The asset to download and where to put it.
  fn download_target(&self, release: &Release) -> (String, PathBuf) {
    match self {
      Install::Msi => {
        let name = release.msi_name();
        let dest = update_dir().join(&name);
        (name, dest)
      }
      // Next to the `.exe`, so the swap is a rename on the same volume and
      // an unwritable folder fails before anything is downloaded.
      Install::Portable(exe) => (release.portable_name(), sibling(exe, ".new")),
    }
  }

  #[cfg(windows)]
  fn apply(&self, file: &Path) -> io::Result<()> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    match self {
      Install::Msi => {
        let exe = std::env::current_exe()?;
        let script = format!(
          "Wait-Process -Id {pid} -ErrorAction SilentlyContinue; \
           Start-Process msiexec.exe -Wait -ArgumentList '/i \"{msi}\" /passive /norestart'; \
           Start-Process -FilePath {exe}",
          pid = std::process::id(),
          msi = file.display().to_string().replace('\'', "''"),
          exe = ps_quote(&exe),
        );
        Command::new("powershell.exe")
          .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-EncodedCommand"])
          .arg(encode_command(&script))
          .creation_flags(CREATE_NO_WINDOW)
          .spawn()?;
      }
      Install::Portable(exe) => {
        let old = sibling(exe, ".old");
        let _ = std::fs::remove_file(&old);
        // Windows lets a running `.exe` be renamed, not overwritten.
        std::fs::rename(exe, &old)?;
        if let Err(error) = std::fs::rename(file, exe) {
          let _ = std::fs::rename(&old, exe);
          return Err(error);
        }
        Command::new(exe).spawn()?;
      }
    }
    Ok(())
  }

  #[cfg(not(windows))]
  fn apply(&self, _file: &Path) -> io::Result<()> {
    Err(io::Error::other("unsupported platform"))
  }
}

/// `path` with `suffix` added to its file name: `keepsave.exe.old`.
fn sibling(path: &Path, suffix: &str) -> PathBuf {
  let mut name = path.as_os_str().to_owned();
  name.push(suffix);
  name.into()
}

/// Where MSI downloads go.
fn update_dir() -> PathBuf {
  std::env::temp_dir().join("KeepSave-update")
}

/// Deletes unfinished downloads and installed MSIs. Runs before any new
/// download can start.
fn cleanup() {
  if let Ok(exe) = std::env::current_exe() {
    for suffix in [".new", ".new.part"] {
      let _ = std::fs::remove_file(sibling(&exe, suffix));
    }
  }
  let _ = std::fs::remove_dir_all(update_dir());
}

#[cfg(windows)]
fn same_path(a: &Path, b: &Path) -> bool {
  a.as_os_str().to_string_lossy().trim_end_matches('\\').eq_ignore_ascii_case(b.as_os_str().to_string_lossy().trim_end_matches('\\'))
}

/// A PowerShell single-quoted string.
#[cfg(windows)]
fn ps_quote(path: &Path) -> String {
  format!("'{}'", path.display().to_string().replace('\'', "''"))
}

/// What `powershell -EncodedCommand` takes: base64 of UTF-16LE. Sidesteps
/// quoting the script on the command line.
#[cfg(windows)]
fn encode_command(script: &str) -> String {
  use base64::Engine;
  let bytes: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
  base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn error_message(i18n: &I18n, error: &UpdateError) -> String {
  match error {
    UpdateError::Network(_) => i18n.tr("update-error-network"),
    UpdateError::Http(403 | 429) => i18n.tr("update-error-rate-limit"),
    UpdateError::Checksum => i18n.tr("update-error-checksum"),
    UpdateError::MissingAsset(_) => i18n.tr("update-error-missing"),
    UpdateError::Io(error) if error.kind() == io::ErrorKind::PermissionDenied => i18n.tr("update-error-folder"),
    other => i18n.tr_arg("error-generic", "details", other.to_string()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn siblings_extend_the_file_name() {
    assert_eq!(sibling(Path::new("C:/a/keepsave.exe"), ".old"), PathBuf::from("C:/a/keepsave.exe.old"));
  }

  #[cfg(windows)]
  #[test]
  fn paths_compare_like_windows() {
    assert!(same_path(Path::new(r"C:\Users\A\AppData\Local\Programs\KeepSave\"), Path::new(r"c:\users\a\appdata\local\programs\keepsave")));
    assert!(!same_path(Path::new(r"C:\Tools"), Path::new(r"C:\Tools\KeepSave")));
    assert_eq!(ps_quote(Path::new(r"C:\O'Brien\keepsave.exe")), r"'C:\O''Brien\keepsave.exe'");
    // "a" in UTF-16LE is 61 00.
    assert_eq!(encode_command("a"), "YQA=");
  }
}
