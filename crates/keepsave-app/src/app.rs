//! App state, navigation and the per-frame loop.

use std::{path::PathBuf, sync::Arc};

use eframe::egui::{self, Align2, Id, Order, Ui, Vec2};
use egui_phosphor::regular as icon;
use tokio::task::AbortHandle;

use crate::{
  background::Background,
  config::Config,
  drive::{self, DriveState},
  fonts,
  hello::{HelloError, Sealed},
  i18n::{I18n, Language},
  screens::{
    CreateVault, DriveImport, DriveSetup, DriveSetupRequest, MainScreen, Nav, ScreenCtx, Settings, SettingsCtx, SettingsRequest, Unlock, Welcome,
    hello_error_message,
  },
  session, theme,
  update::Updater,
  widgets::icon_button,
  worker::{Event, Worker},
};
use keepsave_sync::{Credentials, SyncError};

/// How long a new screen takes to fade and slide in.
const ENTRANCE_TIME: f64 = 0.28;
const ENTRANCE_LIFT: f32 = 16.0;

enum Screen {
  Welcome(Welcome),
  Create(CreateVault),
  Unlock(Unlock),
  DriveImport(DriveImport),
  Main(Box<MainScreen>),
}

pub struct KeepSaveApp {
  config: Config,
  config_path: Option<PathBuf>,
  i18n: I18n,
  worker: Worker,
  background: Background,
  drive: DriveState,
  /// The OAuth client Drive uses, if there is one.
  client: Option<Credentials>,
  /// The Drive setup guide, over everything else.
  drive_setup: Option<DriveSetup>,
  /// The browser sign-in in progress, to cancel it.
  connecting: Option<AbortHandle>,
  screen: Screen,
  /// When the current screen appeared; `None` until its first frame.
  screen_since: Option<f64>,
  settings: Option<Settings>,
  /// Why `config.toml` couldn't be written last time.
  config_error: Option<String>,
  updater: Updater,
  /// Windows Hello can be used on this device.
  hello: bool,
  /// The master password Windows Hello can open, for one vault, until exit.
  sealed: Option<Sealed>,
  /// The vault a Hello prompt was already shown for since it was opened,
  /// so a cancelled prompt isn't asked again at once.
  seal_tried: Option<PathBuf>,
  /// Why sealing failed last time.
  hello_error: Option<String>,
}

impl KeepSaveApp {
  pub fn new(cc: &eframe::CreationContext<'_>, config: Config, config_path: Option<PathBuf>) -> Self {
    fonts::install(&cc.egui_ctx);
    theme::apply(&cc.egui_ctx);
    session::start(cc.egui_ctx.clone());

    let language = config.language.as_deref().and_then(Language::from_code).unwrap_or_else(Language::detect);
    let screen = if config.recent_vaults.is_empty() { Screen::Welcome(Welcome) } else { Screen::Unlock(Unlock::new(None, &config)) };

    let i18n = I18n::new(language);

    let client = drive::load_client();
    let worker = Worker::new(cc.egui_ctx.clone()).expect("start the background runtime");
    worker.check_hello();
    Self {
      drive: DriveState::startup(client.as_ref(), &i18n),
      client,
      drive_setup: None,
      connecting: None,
      i18n,
      worker,
      background: Background::new(cc),
      config,
      config_path,
      screen,
      screen_since: None,
      settings: None,
      config_error: None,
      updater: Updater::new(),
      hello: false,
      sealed: None,
      seal_tried: None,
      hello_error: None,
    }
  }

  fn navigate(&mut self, nav: Nav, ctx: &egui::Context) {
    let screen = match nav {
      Nav::Welcome => Screen::Welcome(Welcome),
      Nav::Create => Screen::Create(CreateVault::new(&self.i18n)),
      Nav::Unlock { select } => {
        if select.is_none() && self.config.recent_vaults.is_empty() {
          Screen::Welcome(Welcome)
        } else {
          Screen::Unlock(Unlock::new(select, &self.config))
        }
      }
      Nav::Opened(open) => {
        self.config.remember_vault(&open.path);
        self.save_config();
        let pending = self.config.drive_link(&open.path).is_some_and(|link| link.pending);
        // One sealed vault at a time (Q25).
        if self.sealed.as_ref().is_some_and(|sealed| sealed.path != open.path) {
          self.sealed = None;
        }
        self.seal_tried = None;
        Screen::Main(Box::new(MainScreen::new(open, pending)))
      }
      // Replacing the screen drops the vault, which wipes its secrets,
      // and the clipboard, which clears what we copied. egui's widget
      // memory may hold edited text, so it goes too.
      Nav::Lock(path) => {
        ctx.memory_mut(|memory| memory.data = Default::default());
        Screen::Unlock(Unlock::new(Some(path), &self.config))
      }
      Nav::SetLanguage(language) => {
        self.i18n = I18n::new(language.unwrap_or_else(Language::detect));
        self.config.language = language.map(|language| language.code().into());
        self.save_config();
        return;
      }
      Nav::OpenSettings => {
        self.settings = Some(Settings::default());
        return;
      }
      Nav::ForgetAllVaults => {
        self.config.forget_all_vaults();
        self.save_config();
        self.sealed = None;
        match self.screen {
          Screen::Unlock(_) => Screen::Welcome(Welcome),
          _ => return,
        }
      }
      Nav::ForgetVault(path) => {
        self.config.forget_vault(&path);
        self.save_config();
        self.refresh_quick_unlock();

        match &mut self.screen {
          Screen::Unlock(_) if self.config.recent_vaults.is_empty() => Screen::Welcome(Welcome),
          Screen::Unlock(unlock) => {
            unlock.forgotten(&path, &self.config);
            return;
          }
          _ => return,
        }
      }
      Nav::SetQuickUnlock { path, on } => {
        self.config.set_quick_unlock(&path, on);
        self.save_config();
        // Asked for explicitly: prompt even if a prompt was cancelled before.
        self.seal_tried = None;
        self.refresh_quick_unlock();
        return;
      }
      Nav::DropQuickUnlock => {
        self.sealed = None;
        return;
      }
      Nav::ConnectDrive => {
        match &self.client {
          Some(credentials) => {
            self.connecting = Some(self.worker.connect_drive(credentials.clone(), drive::browser_pages(&self.i18n)));
            self.drive = DriveState::Connecting;
          }
          None => self.drive_setup = Some(DriveSetup::new()),
        }
        return;
      }
      Nav::SetUpDrive => {
        self.drive_setup = Some(DriveSetup::new());
        return;
      }
      Nav::ChangeDriveClient => {
        self.drive_setup = Some(DriveSetup::credentials_only());
        return;
      }
      Nav::RemoveDriveClient => {
        if let Err(error) = keepsave_sync::delete_client() {
          // Rare (the credential store failed); Settings shows it on the
          // connection row.
          if let DriveState::Disconnected { error: slot } = &mut self.drive {
            *slot = Some(drive::error_message(&self.i18n, &error));
          }
          return;
        }
        self.disconnect_drive();
        self.client = None;
        self.drive = DriveState::NotSetUp;
        return;
      }
      Nav::CancelDrive => {
        if let Some(task) = self.connecting.take() {
          task.abort();
        }
        self.drive = DriveState::Disconnected { error: None };
        return;
      }
      Nav::DisconnectDrive => {
        self.disconnect_drive();
        return;
      }
      Nav::ImportFromDrive => Screen::DriveImport(DriveImport::new()),
      Nav::DriveImported { path, file } => {
        self.config.link_drive(&path, &file);
        self.config.remember_vault(&path);
        self.save_config();
        Screen::Unlock(Unlock::new(Some(path), &self.config))
      }
      Nav::LinkDrive { path, file } => {
        self.config.link_drive(&path, &file);
        self.save_config();
        return;
      }
      Nav::UnlinkDrive(path) => {
        self.config.unlink_drive(&path);
        self.save_config();
        return;
      }
      Nav::DriveSynced { path, file, pending } => {
        self.config.drive_synced(&path, &file, pending);
        self.save_config();
        return;
      }
      Nav::DrivePending(path) => {
        self.config.set_drive_pending(&path, true);
        self.save_config();
        return;
      }
      Nav::CheckForUpdates => {
        self.updater.check(&self.worker);
        return;
      }
      Nav::DownloadUpdate => {
        self.updater.download(&self.worker);
        return;
      }
      Nav::RestartToUpdate => {
        self.updater.request_restart();
        self.settings = None;
        if let Screen::Main(main) = &mut self.screen {
          let cx = ScreenCtx {
            i18n: &self.i18n,
            worker: &self.worker,
            config: &self.config,
            lift: 0.0,
            os_lock: false,
            drive: &self.drive,
            overlay: false,
            hello: self.hello,
            sealed: self.sealed.as_ref(),
          };
          if let Some(nav) = main.lock(&cx) {
            self.navigate(nav, ctx);
          }
        }
        return;
      }
    };
    self.screen = screen;
    self.screen_since = None;
    self.refresh_quick_unlock();
  }

  /// Drops the Drive connection and its refresh token, and cancels a
  /// sign-in in progress.
  fn disconnect_drive(&mut self) {
    if let Some(task) = self.connecting.take() {
      task.abort();
    }
    match std::mem::replace(&mut self.drive, DriveState::Disconnected { error: None }) {
      DriveState::Connected(client) => self.worker.disconnect_drive(client),
      // Not connected now, but a token from an earlier session may remain.
      _ => {
        let _ = keepsave_sync::delete_refresh_token();
      }
    }
  }

  /// Stores the client from the guide and connects with it. A different
  /// client can't use the old refresh token, so that goes first.
  fn save_drive_client(&mut self, credentials: Credentials, ctx: &egui::Context) {
    if let Err(error) = keepsave_sync::save_client(&credentials) {
      if let Some(setup) = &mut self.drive_setup {
        setup.error = Some(drive::error_message(&self.i18n, &error));
      }
      return;
    }
    let same = self.client.as_ref().is_some_and(|current| current.client_id == credentials.client_id);
    if !same {
      self.disconnect_drive();
    }
    self.client = Some(credentials);
    self.drive_setup = None;
    if !matches!(self.drive, DriveState::Connected(_)) {
      self.navigate(Nav::ConnectDrive, ctx);
    }
  }

  /// The Drive setup guide, if it is open.
  fn drive_setup(&mut self, ctx: &egui::Context) {
    let Some(setup) = &mut self.drive_setup else {
      return;
    };
    match setup.show(ctx, &self.i18n) {
      Some(DriveSetupRequest::Close) => self.drive_setup = None,
      Some(DriveSetupRequest::Save(credentials)) => self.save_drive_client(credentials, ctx),
      None => {}
    }
  }

  /// Keeps the sealed password in line with the config and the open vault
  /// (Q25): drops it when its vault is turned off, and seals the open vault
  /// when it is turned on and not sealed yet.
  fn refresh_quick_unlock(&mut self) {
    if self.sealed.as_ref().is_some_and(|sealed| !self.config.quick_unlock(&sealed.path)) {
      self.sealed = None;
    }
    let Screen::Main(main) = &self.screen else {
      return;
    };
    let open = main.open_vault();
    let sealed = self.sealed.as_ref().is_some_and(|sealed| sealed.path == open.path);
    if self.hello && !sealed && self.seal_tried.as_ref() != Some(&open.path) && self.config.quick_unlock(&open.path) {
      self.seal_tried = Some(open.path.clone());
      self.worker.seal_quick_unlock(open.path.clone(), open.vault.master_password());
    }
  }

  fn save_config(&mut self) {
    if let Some(path) = &self.config_path {
      // Settings are a convenience: failing to write them must not
      // interrupt what the user is doing. Settings shows the error.
      self.config_error = self.config.save_to(path).err().map(|error| error.to_string());
    }
  }

  /// Settings over the current screen, or the gear that opens it on the
  /// screens without a top bar.
  fn settings(&mut self, ctx: &egui::Context) -> Option<Nav> {
    let Some(settings) = &mut self.settings else {
      if !matches!(self.screen, Screen::Main(_)) {
        let tooltip = self.i18n.tr("settings-open");
        let open = egui::Area::new(Id::new("settings-gear"))
          .order(Order::Foreground)
          .anchor(Align2::RIGHT_TOP, Vec2::new(-16.0, 16.0))
          .show(ctx, |ui| icon_button(ui, icon::GEAR_SIX, &tooltip).clicked())
          .inner;
        if open {
          return Some(Nav::OpenSettings);
        }
      }
      return None;
    };
    let before = self.config.clone();
    let cx = SettingsCtx {
      i18n: &self.i18n,
      drive: &self.drive,
      has_client: self.client.is_some(),
      config_error: self.config_error.as_deref(),
      hello: self.hello,
      hello_error: self.hello_error.as_deref(),
      update: self.updater.state(),
    };
    let request = settings.show(ctx, &cx, &mut self.config);
    if self.config != before {
      self.save_config();
      if self.config.quick_unlock != before.quick_unlock {
        self.seal_tried = None;
        self.refresh_quick_unlock();
      }
    }
    match request? {
      SettingsRequest::Close => {
        self.settings = None;
        None
      }
      SettingsRequest::Nav(nav) => Some(nav),
    }
  }

  fn handle_event(&mut self, event: Event) -> Option<Nav> {
    if let Event::DriveConnected(result) = event {
      self.connecting = None;
      self.drive = match result {
        Ok(client) => DriveState::Connected(Arc::new(client)),
        Err(error) => DriveState::Disconnected { error: Some(drive::error_message(&self.i18n, &error)) },
      };
      return None;
    }
    match event {
      Event::HelloChecked(available) => {
        self.hello = available;
        self.refresh_quick_unlock();
        return None;
      }
      Event::QuickUnlockSealed(result) => {
        match result {
          // The vault may have been turned off while the prompt was open.
          Ok(sealed) if self.config.quick_unlock(&sealed.path) => {
            self.sealed = Some(sealed);
            self.hello_error = None;
          }
          Ok(_) | Err(HelloError::Cancelled) => {}
          Err(error) => self.hello_error = Some(hello_error_message(&self.i18n, &error)),
        }
        return None;
      }
      Event::UpdateChecked(result) => {
        self.updater.checked(result, &self.i18n);
        return None;
      }
      Event::UpdateDownloaded(result) => {
        self.updater.downloaded(result, &self.i18n);
        return None;
      }
      _ => {}
    }
    if let Some(SyncError::Reauthorize) = event.drive_error() {
      // Google revoked the grant: forget the token and ask to connect again.
      let _ = keepsave_sync::delete_refresh_token();
      self.drive = DriveState::Disconnected { error: Some(drive::error_message(&self.i18n, &SyncError::Reauthorize)) };
    }

    let cx = ScreenCtx {
      i18n: &self.i18n,
      worker: &self.worker,
      config: &self.config,
      lift: 0.0,
      os_lock: false,
      drive: &self.drive,
      overlay: self.settings.is_some() || self.drive_setup.is_some(),
      hello: self.hello,
      sealed: self.sealed.as_ref(),
    };
    match &mut self.screen {
      Screen::Welcome(welcome) => welcome.on_event(event),
      Screen::Create(create) => create.on_event(event, &cx),
      Screen::Unlock(unlock) => unlock.on_event(event, &cx),
      Screen::DriveImport(import) => import.on_event(event, &cx),
      Screen::Main(main) => main.on_event(event, &cx),
    }
  }

  /// Restarts into the downloaded update once no vault is open. A failed
  /// save before locking cancels it.
  fn restart_when_locked(&mut self, ctx: &egui::Context) {
    if !self.updater.restart_pending() {
      return;
    }
    match &self.screen {
      Screen::Main(main) if !main.locking() => self.updater.cancel_restart(),
      Screen::Main(_) => {}
      _ => {
        if self.updater.restart(&self.i18n) {
          ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
      }
    }
  }

  /// Eased 0→1 progress of the current screen's entrance.
  fn entrance(&mut self, ui: &Ui) -> f32 {
    let now = ui.input(|input| input.time);
    let since = *self.screen_since.get_or_insert(now);
    let t = ((now - since) / ENTRANCE_TIME).clamp(0.0, 1.0) as f32;
    if t < 1.0 {
      ui.ctx().request_repaint();
    }
    1.0 - (1.0 - t).powi(3)
  }
}

impl eframe::App for KeepSaveApp {
  fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
    let ctx = ui.ctx().clone();
    let events: Vec<Event> = self.worker.poll().collect();
    for event in events {
      if let Some(nav) = self.handle_event(event) {
        self.navigate(nav, &ctx);
      }
    }
    // Taken on every screen, so a lock before unlocking can't carry over.
    let os_lock = session::take_lock_request();

    self.background.paint(ui, self.config.animated_background);
    let entrance = self.entrance(ui);

    let nav = egui::CentralPanel::no_frame()
      .show(ui, |ui| {
        ui.multiply_opacity(entrance);
        let cx = ScreenCtx {
          i18n: &self.i18n,
          worker: &self.worker,
          config: &self.config,
          lift: (1.0 - entrance) * ENTRANCE_LIFT,
          os_lock,
          drive: &self.drive,
          overlay: self.settings.is_some() || self.drive_setup.is_some(),
          hello: self.hello,
          sealed: self.sealed.as_ref(),
        };
        match &mut self.screen {
          Screen::Welcome(welcome) => welcome.ui(ui, &cx),
          Screen::Create(create) => create.ui(ui, &cx),
          Screen::Unlock(unlock) => unlock.ui(ui, &cx),
          Screen::DriveImport(import) => import.ui(ui, &cx),
          Screen::Main(main) => main.ui(ui, &cx),
        }
      })
      .inner;
    if let Some(nav) = nav {
      self.navigate(nav, &ctx);
    }
    if let Some(nav) = self.settings(&ctx) {
      self.navigate(nav, &ctx);
    }
    self.drive_setup(&ctx);
    if self.settings.is_none()
      && self.drive_setup.is_none()
      && let Some(nav) = self.updater.banner(&ctx, &self.i18n)
    {
      self.navigate(nav, &ctx);
    }
    self.updater.tick(&ctx, &self.worker, self.config.check_for_updates);
    self.restart_when_locked(&ctx);
  }

  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    theme::BASE.to_normalized_gamma_f32()
  }
}
