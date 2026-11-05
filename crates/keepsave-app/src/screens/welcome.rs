//! First-run screen: create, open or import a vault.

use eframe::egui::{Id, RichText, Ui, Vec2};
use egui_phosphor::regular as icon;

use super::{Nav, ScreenCtx, emblem, title};
use crate::{
  drive::DriveState,
  theme,
  widgets::{ActionButton, centered_column, glass_card, link},
  worker::Event,
};

#[derive(Default)]
pub struct Welcome;

impl Welcome {
  pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
    let tr = |key| cx.i18n.tr(key);
    let mut nav = None;

    centered_column(ui, Id::new("welcome"), 420.0, Vec2::new(0.0, cx.lift), |ui| {
      ui.vertical_centered(|ui| emblem(ui, icon::VAULT, false));
      title(ui, &tr("app-name"), Some(&tr("app-tagline")));
      ui.add_space(20.0);

      glass_card(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 12.0;
        let create = ActionButton::new(tr("welcome-create")).icon(icon::PLUS).primary().full_width();
        if ui.add(create).clicked() {
          nav = Some(Nav::Create);
        }
        let open = ActionButton::new(tr("welcome-open")).icon(icon::FOLDER_OPEN).full_width();
        if ui.add(open).clicked() {
          cx.worker.pick_vault_file(tr("dialog-open-title"), tr("dialog-filter"));
        }
        let available = !matches!(cx.drive, DriveState::Unavailable);
        let drive = ActionButton::new(tr("welcome-import-drive")).icon(icon::GOOGLE_DRIVE_LOGO).full_width().enabled(available);
        if ui.add(drive).clicked() {
          nav = Some(Nav::ImportFromDrive);
        }
        if !available {
          ui.vertical_centered(|ui| {
            ui.label(RichText::new(tr("drive-unavailable")).small().color(theme::TEXT_FAINT));
          });
        }
      });

      ui.add_space(8.0);
      if !cx.config.recent_vaults.is_empty() {
        ui.vertical_centered(|ui| {
          if link(ui, &tr("welcome-recent")).clicked() {
            nav = Some(Nav::Unlock { select: None });
          }
        });
      }
    });
    nav
  }

  pub fn on_event(&mut self, event: Event) -> Option<Nav> {
    match event {
      Event::VaultFilePicked(Some(path)) => Some(Nav::Unlock { select: Some(path) }),
      _ => None,
    }
  }
}
