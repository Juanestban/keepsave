//! Settings: a modal over whatever screen is showing. Every change applies
//! and is saved at once.

use std::path::Path;

use eframe::egui::{
  self, Align, Color32, ComboBox, CornerRadius, Frame, Id, Label, Layout, Margin, Modal, RichText, ScrollArea, Sense, Stroke, Ui, Vec2,
};
use egui_phosphor::regular as icon;

use super::Nav;
use crate::{
  config::Config,
  drive::DriveState,
  i18n::{I18n, Language},
  theme,
  update::State as UpdateState,
  widgets::{ActionButton, HOVER_TIME, icon_button, icon_link, notice, single_line, toggle},
};

const WIDTH: f32 = 760.0;
const HEIGHT: f32 = 470.0;
const TABS_WIDTH: f32 = 180.0;
const CONTROL_WIDTH: f32 = 170.0;
const LOCK_MINUTES: [u32; 8] = [0, 1, 2, 5, 10, 15, 30, 60];
const CLIPBOARD_SECONDS: [u32; 6] = [0, 10, 15, 30, 60, 90];
const REPOSITORY: &str = "https://github.com/Juanestban/keepsave";
const EFF_WORDLIST: &str = "https://www.eff.org/dice";

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Tab {
  #[default]
  General,
  Security,
  Drive,
  Language,
  Appearance,
  About,
}

impl Tab {
  const ALL: [Self; 6] = [Self::General, Self::Security, Self::Drive, Self::Language, Self::Appearance, Self::About];

  fn label(self) -> (&'static str, &'static str) {
    match self {
      Self::General => (icon::SLIDERS_HORIZONTAL, "settings-tab-general"),
      Self::Security => (icon::SHIELD_CHECK, "settings-tab-security"),
      Self::Drive => (icon::GOOGLE_DRIVE_LOGO, "settings-tab-drive"),
      Self::Language => (icon::TRANSLATE, "settings-tab-language"),
      Self::Appearance => (icon::PAINT_BRUSH, "settings-tab-appearance"),
      Self::About => (icon::INFO, "settings-tab-about"),
    }
  }
}

/// What Settings can read while drawing.
pub struct SettingsCtx<'a> {
  pub i18n: &'a I18n,
  pub drive: &'a DriveState,
  /// Whether the user has set up their Google client.
  pub has_client: bool,
  /// Why `config.toml` couldn't be written, if it couldn't.
  pub config_error: Option<&'a str>,
  pub update: &'a UpdateState,
}

/// What the user asked of the app from Settings.
pub enum Request {
  Close,
  Nav(Nav),
}

#[derive(Default)]
pub struct Settings {
  tab: Tab,
}

impl Settings {
  /// Draws Settings. Plain settings are edited in `config` directly; the
  /// app saves it when it changes.
  pub fn show(&mut self, ctx: &egui::Context, cx: &SettingsCtx, config: &mut Config) -> Option<Request> {
    let tr = |key| cx.i18n.tr(key);
    let screen = ctx.content_rect();
    let width = WIDTH.min(screen.width() - 48.0);
    let height = HEIGHT.min(screen.height() - 150.0).max(240.0);
    let frame = Frame::new()
      .fill(theme::GLASS_FILL_SOLID)
      .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
      .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
      .inner_margin(Margin::same(24));

    let mut request = None;
    let modal = Modal::new(Id::new("settings")).frame(frame).backdrop_color(Color32::from_black_alpha(140)).show(ctx, |ui| {
      ui.set_width(width);
      let row = Vec2::new(ui.available_width(), 40.0);
      ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
        if icon_button(ui, icon::X, &tr("settings-close")).clicked() {
          request = Some(Request::Close);
        }
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          ui.label(RichText::new(icon::GEAR_SIX).font(theme::icon_font(22.0)).color(theme::ACCENT));
          ui.label(RichText::new(tr("settings-title")).font(theme::heading(20.0)).color(theme::TEXT));
        });
      });
      ui.add_space(12.0);

      ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(Vec2::new(TABS_WIDTH, height), Layout::top_down(Align::Min), |ui| {
          ui.spacing_mut().item_spacing.y = 4.0;
          for tab in Tab::ALL {
            let (glyph, key) = tab.label();
            if tab_button(ui, glyph, &tr(key), self.tab == tab).clicked() {
              self.tab = tab;
            }
          }
        });
        ui.add_space(16.0);
        ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), height), Layout::top_down(Align::Min), |ui| {
          ScrollArea::vertical().id_salt(("settings-body", self.tab as u8)).auto_shrink([false, false]).show(ui, |ui| {
            // Keep the controls clear of the scrollbar.
            ui.set_width(ui.available_width() - 12.0);
            ui.spacing_mut().item_spacing.y = 10.0;
            let nav = match self.tab {
              Tab::General => general(ui, cx, config),
              Tab::Security => {
                security(ui, cx, config);
                None
              }
              Tab::Drive => drive(ui, cx, config),
              Tab::Language => language(ui, cx, config),
              Tab::Appearance => {
                appearance(ui, cx, config);
                None
              }
              Tab::About => about(ui, cx, config),
            };
            if let Some(nav) = nav {
              request = Some(Request::Nav(nav));
            }
          });
        });
      });
    });
    if modal.should_close() {
      request = Some(Request::Close);
    }
    request
  }
}

fn general(ui: &mut Ui, cx: &SettingsCtx, config: &Config) -> Option<Nav> {
  let tr = |key| cx.i18n.tr(key);
  let mut nav = None;
  if let Some(error) = cx.config_error {
    notice(ui, icon::WARNING_CIRCLE, &cx.i18n.tr_arg("settings-config-error", "details", error.to_owned()), theme::DANGER);
  }
  section(ui, &tr("settings-recent"), Some(&tr("settings-recent-hint")));
  if config.recent_vaults.is_empty() {
    ui.label(RichText::new(tr("settings-recent-empty")).color(theme::TEXT_FAINT));
    return None;
  }
  for path in &config.recent_vaults {
    let removed = item_row(ui, icon::VAULT, &file_name(path), &path.display().to_string(), None, |ui| {
      icon_button(ui, icon::X, &tr("settings-recent-remove")).clicked()
    });
    if removed {
      nav = Some(Nav::ForgetVault(path.clone()));
    }
  }
  ui.add_space(4.0);
  if icon_link(ui, icon::TRASH, &tr("settings-recent-clear")).clicked() {
    nav = Some(Nav::ForgetAllVaults);
  }
  nav
}

fn security(ui: &mut Ui, cx: &SettingsCtx, config: &mut Config) {
  ui.spacing_mut().item_spacing.y = 16.0;
  let tr = |key| cx.i18n.tr(key);
  section(ui, &tr("settings-tab-security"), None);
  let minutes = |value: u32| if value == 0 { tr("settings-never") } else { cx.i18n.tr_arg("settings-minutes", "count", value) };
  setting(ui, &tr("settings-lock-after"), Some(&tr("settings-lock-after-hint")), |ui| {
    choice(ui, "settings-lock-after", &mut config.lock_after_minutes, &LOCK_MINUTES, minutes);
  });
  setting(ui, &tr("settings-lock-minimize"), None, |ui| {
    toggle(ui, &mut config.lock_on_minimize, true, &tr("settings-lock-minimize"));
  });
  let sleep_hint = if cfg!(windows) { None } else { Some(tr("settings-lock-sleep-windows")) };
  setting(ui, &tr("settings-lock-sleep"), sleep_hint.as_deref(), |ui| {
    toggle(ui, &mut config.lock_on_os_sleep, cfg!(windows), &tr("settings-lock-sleep"));
  });
  let seconds = |value: u32| if value == 0 { tr("settings-clipboard-keep") } else { cx.i18n.tr_arg("settings-seconds", "count", value) };
  setting(ui, &tr("settings-clipboard-clear"), Some(&tr("settings-clipboard-clear-hint")), |ui| {
    choice(ui, "settings-clipboard-clear", &mut config.clipboard_clear_seconds, &CLIPBOARD_SECONDS, seconds);
  });
}

fn drive(ui: &mut Ui, cx: &SettingsCtx, config: &Config) -> Option<Nav> {
  let tr = |key| cx.i18n.tr(key);
  let mut nav = None;
  section(ui, &tr("settings-drive-connection"), Some(&tr("drive-connect-hint")));
  match cx.drive {
    DriveState::NotSetUp => setting(ui, &tr("drive-unavailable"), Some(&tr("settings-drive-setup-hint")), |ui| {
      if ui.add(ActionButton::new(tr("drive-setup")).icon(icon::GOOGLE_DRIVE_LOGO).primary()).clicked() {
        nav = Some(Nav::SetUpDrive);
      }
    }),
    DriveState::Disconnected { error } => {
      setting(ui, &tr("settings-drive-disconnected"), None, |ui| {
        if ui.add(ActionButton::new(tr("drive-connect-short")).icon(icon::GOOGLE_DRIVE_LOGO).primary()).clicked() {
          nav = Some(Nav::ConnectDrive);
        }
      });
      if let Some(error) = error {
        notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
      }
    }
    DriveState::Connecting => setting(ui, &tr("drive-waiting"), None, |ui| {
      if ui.add(ActionButton::new(tr("drive-cancel"))).clicked() {
        nav = Some(Nav::CancelDrive);
      }
      ui.add(egui::Spinner::new().size(16.0).color(theme::ACCENT));
    }),
    DriveState::Connected(_) => setting(ui, &tr("settings-drive-connected"), None, |ui| {
      if ui.add(ActionButton::new(tr("settings-drive-disconnect")).icon(icon::SIGN_OUT)).clicked() {
        nav = Some(Nav::DisconnectDrive);
      }
    }),
  }

  if cx.has_client {
    ui.add_space(8.0);
    section(ui, &tr("settings-drive-client"), None);
    setting(ui, &tr("settings-drive-client-user"), Some(&tr("settings-drive-client-user-hint")), |ui| {
      if icon_button(ui, icon::TRASH, &tr("settings-drive-client-remove")).clicked() {
        nav = Some(Nav::RemoveDriveClient);
      }
      if ui.add(ActionButton::new(tr("settings-drive-client-change")).icon(icon::PENCIL_SIMPLE)).clicked() {
        nav = Some(Nav::ChangeDriveClient);
      }
    });
  }

  ui.add_space(8.0);
  section(ui, &tr("settings-drive-links"), None);
  if config.drive_links.is_empty() {
    ui.add(Label::new(RichText::new(tr("settings-drive-links-empty")).color(theme::TEXT_FAINT)).wrap());
  }
  for link in &config.drive_links {
    let pending = link.pending.then(|| tr("sync-pending"));
    let unlinked = item_row(ui, icon::CLOUD, &link.name, &link.path.display().to_string(), pending.as_deref(), |ui| {
      icon_button(ui, icon::LINK_BREAK, &tr("settings-drive-unlink")).clicked()
    });
    if unlinked {
      nav = Some(Nav::UnlinkDrive(link.path.clone()));
    }
  }
  nav
}

fn language(ui: &mut Ui, cx: &SettingsCtx, config: &Config) -> Option<Nav> {
  let tr = |key| cx.i18n.tr(key);
  let mut nav = None;
  section(ui, &tr("settings-tab-language"), None);
  let current = config.language.as_deref().and_then(Language::from_code);
  let system = cx.i18n.tr_arg("settings-language-system", "language", tr(Language::detect().name_key()));
  let options = std::iter::once((None, system)).chain(Language::ALL.map(|language| (Some(language), tr(language.name_key()))));
  for (language, label) in options {
    if choice_row(ui, &label, current == language).clicked() && current != language {
      nav = Some(Nav::SetLanguage(language));
    }
  }
  nav
}

fn appearance(ui: &mut Ui, cx: &SettingsCtx, config: &mut Config) {
  ui.spacing_mut().item_spacing.y = 16.0;
  let tr = |key| cx.i18n.tr(key);
  section(ui, &tr("settings-tab-appearance"), None);
  setting(ui, &tr("settings-animated-background"), Some(&tr("settings-animated-background-hint")), |ui| {
    toggle(ui, &mut config.animated_background, true, &tr("settings-animated-background"));
  });
}

fn about(ui: &mut Ui, cx: &SettingsCtx, config: &mut Config) -> Option<Nav> {
  let tr = |key| cx.i18n.tr(key);
  let mut nav = None;
  ui.horizontal(|ui| {
    ui.label(RichText::new(icon::VAULT).font(theme::icon_font(34.0)).color(theme::ACCENT));
    ui.vertical(|ui| {
      ui.spacing_mut().item_spacing.y = 2.0;
      ui.label(RichText::new(tr("app-name")).font(theme::heading(20.0)).color(theme::TEXT));
      ui.label(RichText::new(cx.i18n.tr_arg("about-version", "version", env!("CARGO_PKG_VERSION"))).color(theme::TEXT_MUTED));
    });
  });
  ui.label(RichText::new(tr("app-tagline")).color(theme::TEXT));

  ui.add_space(8.0);
  section(ui, &tr("about-updates"), None);
  let (status, color) = match cx.update {
    UpdateState::Idle => (tr("update-status-idle"), theme::TEXT_MUTED),
    UpdateState::Checking => (tr("update-status-checking"), theme::TEXT_MUTED),
    UpdateState::UpToDate => (tr("update-status-up-to-date"), theme::SUCCESS),
    UpdateState::CheckFailed(error) => (error.clone(), theme::DANGER),
    UpdateState::Available(release) | UpdateState::Failed { release, .. } => {
      (cx.i18n.tr_arg("update-status-available", "version", release.version.to_string()), theme::ACCENT)
    }
    UpdateState::Downloading { .. } => (tr("update-status-downloading"), theme::ACCENT),
    UpdateState::Ready { .. } => (tr("update-ready"), theme::ACCENT),
  };
  setting_text(ui, RichText::new(status).color(color), None, |ui| match cx.update {
    UpdateState::Available(_) | UpdateState::Failed { .. } => {
      if ui.add(ActionButton::new(tr("update-install")).icon(icon::DOWNLOAD_SIMPLE).primary()).clicked() {
        nav = Some(Nav::DownloadUpdate);
      }
    }
    UpdateState::Ready { .. } => {
      if ui.add(ActionButton::new(tr("update-restart")).icon(icon::ARROW_CLOCKWISE).primary()).clicked() {
        nav = Some(Nav::RestartToUpdate);
      }
    }
    state => {
      let busy = matches!(state, UpdateState::Checking | UpdateState::Downloading { .. });
      if ui.add(ActionButton::new(tr("update-check-now")).icon(icon::ARROWS_CLOCKWISE).loading(busy)).clicked() {
        nav = Some(Nav::CheckForUpdates);
      }
    }
  });
  if let UpdateState::Failed { error, .. } = cx.update {
    notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
  }
  setting(ui, &tr("update-automatic"), Some(&tr("update-automatic-hint")), |ui| {
    toggle(ui, &mut config.check_for_updates, true, &tr("update-automatic"));
  });

  ui.add_space(8.0);
  section(ui, &tr("about-license-title"), None);
  ui.add(Label::new(RichText::new(tr("about-license")).color(theme::TEXT_MUTED)).wrap());
  if icon_link(ui, icon::GITHUB_LOGO, &tr("about-repository")).clicked() {
    let _ = webbrowser::open(REPOSITORY);
  }

  ui.add_space(8.0);
  section(ui, &tr("about-credits"), None);
  for key in ["about-credit-inter", "about-credit-phosphor", "about-credit-eff"] {
    ui.add(Label::new(RichText::new(tr(key)).color(theme::TEXT_MUTED)).wrap());
  }
  if icon_link(ui, icon::ARROW_SQUARE_OUT, &tr("about-eff-link")).clicked() {
    let _ = webbrowser::open(EFF_WORDLIST);
  }
  nav
}

// ---- building blocks ----

fn section(ui: &mut Ui, title: &str, hint: Option<&str>) {
  ui.label(RichText::new(title).font(theme::heading(15.0)).color(theme::TEXT));
  if let Some(hint) = hint {
    ui.add(Label::new(RichText::new(hint).small().color(theme::TEXT_FAINT)).wrap());
  }
}

/// A setting's name and hint on the left, its control on the right.
fn setting(ui: &mut Ui, label: &str, hint: Option<&str>, control: impl FnOnce(&mut Ui)) {
  setting_text(ui, RichText::new(label).color(theme::TEXT), hint, control);
}

/// [`setting`] with a styled label.
fn setting_text(ui: &mut Ui, label: RichText, hint: Option<&str>, control: impl FnOnce(&mut Ui)) {
  let width = ui.available_width();
  ui.allocate_ui_with_layout(Vec2::new(width, 0.0), Layout::left_to_right(Align::Center), |ui| {
    let text_width = (width - CONTROL_WIDTH - 16.0).max(120.0);
    ui.allocate_ui_with_layout(Vec2::new(text_width, 0.0), Layout::top_down(Align::Min), |ui| {
      ui.set_width(text_width);
      ui.spacing_mut().item_spacing.y = 2.0;
      ui.add(Label::new(label).wrap());
      if let Some(hint) = hint {
        ui.add(Label::new(RichText::new(hint).small().color(theme::TEXT_FAINT)).wrap());
      }
    });
    ui.with_layout(Layout::right_to_left(Align::Center), control);
  });
}

/// A drop-down of `options`, plus the current value if it is not one of
/// them (set by hand in `config.toml`).
fn choice(ui: &mut Ui, id: &str, value: &mut u32, options: &[u32], label: impl Fn(u32) -> String) {
  let mut values = options.to_vec();
  if !values.contains(value) {
    values.push(*value);
    values.sort_unstable();
  }
  ui.spacing_mut().interact_size.y = theme::CONTROL_HEIGHT - 8.0;
  ComboBox::from_id_salt(id).selected_text(label(*value)).width(CONTROL_WIDTH).show_ui(ui, |ui| {
    for option in values {
      ui.selectable_value(value, option, label(option));
    }
  });
}

/// A file or link on its own row: icon, title, a faint path and an optional
/// badge, with `action` on the right. Returns what `action` returns.
fn item_row(ui: &mut Ui, glyph: &str, title: &str, path: &str, badge: Option<&str>, action: impl FnOnce(&mut Ui) -> bool) -> bool {
  let frame = Frame::new().fill(theme::GLASS_FILL).corner_radius(CornerRadius::same(theme::RADIUS_CONTROL)).inner_margin(Margin::symmetric(12, 8));
  frame
    .show(ui, |ui| {
      let width = ui.available_width();
      ui.allocate_ui_with_layout(Vec2::new(width, 36.0), Layout::right_to_left(Align::Center), |ui| {
        let clicked = action(ui);
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          ui.label(RichText::new(glyph).font(theme::icon_font(18.0)).color(theme::TEXT_MUTED));
          let text_width = ui.available_width();
          ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.horizontal(|ui| {
              let title = single_line(ui, title, egui::FontId::proportional(14.5), theme::TEXT, text_width - 120.0);
              ui.label(title);
              if let Some(badge) = badge {
                ui.label(RichText::new(badge).small().color(theme::WARNING));
              }
            });
            ui.label(single_line(ui, path, egui::FontId::proportional(12.0), theme::TEXT_FAINT, text_width));
          });
        });
        clicked
      })
      .inner
    })
    .inner
}

/// A tab in the left column.
fn tab_button(ui: &mut Ui, glyph: &str, label: &str, selected: bool) -> egui::Response {
  let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 36.0), Sense::click());
  let hover = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
  let active = ui.ctx().animate_bool_with_time(response.id.with("selected"), selected, HOVER_TIME);
  let fill = Color32::TRANSPARENT.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover).lerp_to_gamma(theme::GLASS_FILL_ACTIVE, active);
  ui.painter().rect_filled(rect, CornerRadius::same(theme::RADIUS_CONTROL), fill);
  let color = theme::TEXT_MUTED.lerp_to_gamma(theme::TEXT, hover.max(active));
  let icon_color = theme::TEXT_MUTED.lerp_to_gamma(theme::ACCENT, active);
  let icon_pos = egui::pos2(rect.left() + 22.0, rect.center().y);
  ui.painter().text(icon_pos, egui::Align2::CENTER_CENTER, glyph, theme::icon_font(17.0), icon_color);
  let text = single_line(ui, label, egui::FontId::proportional(14.5), color, rect.width() - 52.0);
  ui.painter().galley(egui::pos2(rect.left() + 42.0, rect.center().y - text.size().y / 2.0), text, color);
  response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// An option in a single-choice list, with a check when chosen.
fn choice_row(ui: &mut Ui, label: &str, selected: bool) -> egui::Response {
  let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 40.0), Sense::click());
  let hover = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
  let fill = theme::GLASS_FILL.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover);
  let stroke = if selected { theme::ACCENT } else { theme::GLASS_STROKE };
  ui.painter().rect(rect, CornerRadius::same(theme::RADIUS_CONTROL), fill, Stroke::new(1.0, stroke), egui::StrokeKind::Inside);
  let text = single_line(ui, label, egui::FontId::proportional(14.5), theme::TEXT, rect.width() - 60.0);
  ui.painter().galley(egui::pos2(rect.left() + 14.0, rect.center().y - text.size().y / 2.0), text, theme::TEXT);
  if selected {
    let pos = egui::pos2(rect.right() - 22.0, rect.center().y);
    ui.painter().text(pos, egui::Align2::CENTER_CENTER, icon::CHECK, theme::icon_font(17.0), theme::ACCENT);
  }
  response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The file name of a vault path, for labels.
fn file_name(path: &Path) -> String {
  path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
}
