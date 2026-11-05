//! App state, navigation and the per-frame loop.

use std::path::PathBuf;

use eframe::egui::{self, Ui};

use crate::{
    background::Background,
    config::Config,
    fonts,
    i18n::{I18n, Language},
    screens::{CreateVault, MainScreen, Nav, ScreenCtx, Unlock, Welcome},
    session, theme,
    worker::{Event, Worker},
};

/// How long a new screen takes to fade and slide in.
const ENTRANCE_TIME: f64 = 0.28;
const ENTRANCE_LIFT: f32 = 16.0;

enum Screen {
    Welcome(Welcome),
    Create(CreateVault),
    Unlock(Unlock),
    Main(Box<MainScreen>),
}

pub struct KeepSaveApp {
    config: Config,
    config_path: Option<PathBuf>,
    i18n: I18n,
    worker: Worker,
    background: Background,
    screen: Screen,
    /// When the current screen appeared; `None` until its first frame.
    screen_since: Option<f64>,
}

impl KeepSaveApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::install(&cc.egui_ctx);
        theme::apply(&cc.egui_ctx);
        session::start(cc.egui_ctx.clone());

        let config_path = Config::default_path();
        let config = config_path
            .as_deref()
            .map(Config::load_from)
            .unwrap_or_default();
        let language = config
            .language
            .as_deref()
            .and_then(Language::from_code)
            .unwrap_or_else(Language::detect);
        let screen = if config.recent_vaults.is_empty() {
            Screen::Welcome(Welcome)
        } else {
            Screen::Unlock(Unlock::new(None, &config))
        };

        Self {
            i18n: I18n::new(language),
            worker: Worker::new(cc.egui_ctx.clone()).expect("start the background runtime"),
            background: Background::new(cc),
            config,
            config_path,
            screen,
            screen_since: None,
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
                Screen::Main(Box::new(MainScreen::new(open)))
            }
            // Replacing the screen drops the vault, which wipes its secrets,
            // and the clipboard, which clears what we copied. egui's widget
            // memory may hold edited text, so it goes too.
            Nav::Lock(path) => {
                ctx.memory_mut(|memory| memory.data = Default::default());
                Screen::Unlock(Unlock::new(Some(path), &self.config))
            }
            Nav::SetLanguage(language) => {
                self.i18n = I18n::new(language);
                self.config.language = Some(language.code().into());
                self.save_config();
                return;
            }
            Nav::ForgetVault(path) => {
                self.config.forget_vault(&path);
                self.save_config();
                match &mut self.screen {
                    Screen::Unlock(_) if self.config.recent_vaults.is_empty() => {
                        Screen::Welcome(Welcome)
                    }
                    Screen::Unlock(unlock) => {
                        unlock.forgotten(&path, &self.config);
                        return;
                    }
                    _ => return,
                }
            }
        };
        self.screen = screen;
        self.screen_since = None;
    }

    fn save_config(&self) {
        if let Some(path) = &self.config_path {
            // Settings are a convenience: failing to write them must not
            // interrupt what the user is doing. Settings (M6) will surface it.
            let _ = self.config.save_to(path);
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Nav> {
        let cx = ScreenCtx {
            i18n: &self.i18n,
            worker: &self.worker,
            config: &self.config,
            lift: 0.0,
            os_lock: false,
        };
        match &mut self.screen {
            Screen::Welcome(welcome) => welcome.on_event(event),
            Screen::Create(create) => create.on_event(event, &cx),
            Screen::Unlock(unlock) => unlock.on_event(event, &cx),
            Screen::Main(main) => {
                main.on_event(event, &cx);
                None
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

        self.background.paint(ui);
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
                };
                match &mut self.screen {
                    Screen::Welcome(welcome) => welcome.ui(ui, &cx),
                    Screen::Create(create) => create.ui(ui, &cx),
                    Screen::Unlock(unlock) => unlock.ui(ui, &cx),
                    Screen::Main(main) => main.ui(ui, &cx),
                }
            })
            .inner;
        if let Some(nav) = nav {
            self.navigate(nav, &ctx);
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        theme::BASE.to_normalized_gamma_f32()
    }
}
