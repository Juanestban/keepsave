//! "New PC" flow: connect Google Drive, pick a vault from the KeepSave
//! folder, choose where to save it, then unlock it.

use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Id, Pos2, RichText, ScrollArea, Sense, Ui, Vec2};
use egui_phosphor::regular as icon;
use keepsave_sync::RemoteFile;

use super::{Nav, ScreenCtx, emblem, title};
use crate::{
  drive::{self, DriveState},
  theme,
  widgets::{ActionButton, HOVER_TIME, centered_column, glass_card, link, notice, single_line},
  worker::Event,
};

enum Stage {
  /// Nothing requested yet; lists as soon as Drive is connected.
  Idle,
  Listing,
  Ready,
  Picking,
  Downloading(String),
  Failed(String),
}

pub struct DriveImport {
  stage: Stage,
  files: Vec<RemoteFile>,
}

impl DriveImport {
  pub fn new() -> Self {
    Self { stage: Stage::Idle, files: Vec::new() }
  }

  pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
    let tr = |key| cx.i18n.tr(key);
    let mut nav = None;

    let client = cx.drive.client().cloned();
    match (&client, &self.stage) {
      (Some(client), Stage::Idle) => {
        cx.worker.list_drive(client.clone());
        self.stage = Stage::Listing;
      }
      // Disconnected meanwhile, e.g. the token was revoked.
      (None, Stage::Listing | Stage::Ready | Stage::Downloading(_)) => self.stage = Stage::Idle,
      _ => {}
    }
    let busy = matches!(cx.drive, DriveState::Connecting) || matches!(self.stage, Stage::Listing | Stage::Picking | Stage::Downloading(_));

    centered_column(ui, Id::new("drive-import"), 460.0, Vec2::new(0.0, cx.lift), |ui| {
      ui.vertical_centered(|ui| emblem(ui, icon::CLOUD_ARROW_DOWN, busy));
      title(ui, &tr("drive-title"), Some(&tr("drive-subtitle")));
      ui.add_space(16.0);

      glass_card(ui, |ui| {
        ui.set_width(ui.available_width());
        match cx.drive {
          DriveState::NotSetUp => {
            ui.label(RichText::new(tr("drive-setup-hint")).color(theme::TEXT_MUTED));
            ui.add_space(4.0);
            if ui.add(ActionButton::new(tr("drive-setup")).icon(icon::GOOGLE_DRIVE_LOGO).primary().full_width()).clicked() {
              nav = Some(Nav::SetUpDrive);
            }
          }
          DriveState::Disconnected { error } => {
            ui.label(RichText::new(tr("drive-connect-hint")).color(theme::TEXT_MUTED));
            if let Some(error) = error {
              notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
            }
            ui.add_space(4.0);
            let connect = ActionButton::new(tr("drive-connect")).icon(icon::GOOGLE_DRIVE_LOGO).primary().full_width();
            if ui.add(connect).clicked() {
              nav = Some(Nav::ConnectDrive);
            }
          }
          DriveState::Connecting => {
            status_line(ui, &tr("drive-waiting"));
            if ui.add(ActionButton::new(tr("drive-cancel")).full_width()).clicked() {
              nav = Some(Nav::CancelDrive);
            }
          }
          DriveState::Connected(_) => self.connected_ui(ui, cx),
        }
      });

      ui.add_space(8.0);
      ui.vertical_centered(|ui| {
        if link(ui, &tr("common-back")).clicked() {
          nav = Some(Nav::Welcome);
        }
      });
    });
    nav
  }

  fn connected_ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) {
    let tr = |key| cx.i18n.tr(key);
    match &self.stage {
      Stage::Idle | Stage::Listing => status_line(ui, &tr("drive-loading")),
      Stage::Picking => status_line(ui, &tr("drive-picking")),
      Stage::Downloading(name) => status_line(ui, &cx.i18n.tr_arg("drive-downloading", "name", name.clone())),
      Stage::Failed(error) => {
        notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
        if ui.add(ActionButton::new(tr("drive-retry")).icon(icon::ARROWS_CLOCKWISE).full_width()).clicked() {
          self.stage = Stage::Idle;
        }
      }
      Stage::Ready if self.files.is_empty() => {
        ui.label(RichText::new(tr("drive-empty")).color(theme::TEXT_MUTED));
        if ui.add(ActionButton::new(tr("drive-retry")).icon(icon::ARROWS_CLOCKWISE).full_width()).clicked() {
          self.stage = Stage::Idle;
        }
      }
      Stage::Ready => {
        let mut picked = None;
        ScrollArea::vertical().max_height(300.0).auto_shrink([false, true]).show(ui, |ui| {
          ui.spacing_mut().item_spacing.y = 4.0;
          for file in &self.files {
            if file_row(ui, file).clicked() {
              picked = Some(file.clone());
            }
          }
        });
        if let Some(file) = picked {
          cx.worker.pick_import_path(tr("dialog-import-title"), tr("dialog-filter"), file);
          self.stage = Stage::Picking;
        }
      }
    }
  }

  pub fn on_event(&mut self, event: Event, cx: &ScreenCtx) -> Option<Nav> {
    let failed = |error| Stage::Failed(drive::error_message(cx.i18n, error));
    match event {
      Event::DriveFilesListed(Ok(files)) => {
        self.files = files;
        self.stage = Stage::Ready;
      }
      Event::DriveFilesListed(Err(error)) => self.stage = failed(&error),
      Event::DriveImportPathPicked { file, path } => match (path, cx.drive.client()) {
        (Some(path), Some(client)) => {
          self.stage = Stage::Downloading(file.name.clone());
          cx.worker.download_from_drive(client.clone(), file, path);
        }
        _ => self.stage = Stage::Ready,
      },
      Event::DriveDownloaded { path, result } => match result {
        Ok(file) => return Some(Nav::DriveImported { path, file }),
        Err(error) => self.stage = failed(&error),
      },
      _ => {}
    }
    None
  }
}

fn status_line(ui: &mut Ui, text: &str) {
  ui.horizontal(|ui| {
    ui.add(egui::Spinner::new().size(16.0).color(theme::ACCENT));
    ui.label(RichText::new(text).color(theme::TEXT_MUTED));
  });
}

/// A Drive file as a clickable row: icon, name, size and date.
fn file_row(ui: &mut Ui, file: &RemoteFile) -> egui::Response {
  let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 56.0), Sense::click());
  let hover = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
  let painter = ui.painter();
  painter.rect_filled(rect, CornerRadius::same(theme::RADIUS_CONTROL), theme::GLASS_FILL.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover));
  painter.text(
    Pos2::new(rect.left() + 26.0, rect.center().y),
    Align2::CENTER_CENTER,
    icon::DATABASE,
    theme::icon_font(22.0),
    theme::TEXT_MUTED.lerp_to_gamma(theme::ACCENT, hover),
  );

  let width = rect.width() - 64.0;
  let name = single_line(ui, &file.name, theme::heading(15.0), theme::TEXT, width);
  let details = single_line(ui, &drive::file_details(file), FontId::proportional(12.5), theme::TEXT_FAINT, width);
  let left = rect.left() + 52.0;
  ui.painter().galley(Pos2::new(left, rect.center().y - name.size().y), name, Color32::PLACEHOLDER);
  ui.painter().galley(Pos2::new(left, rect.center().y + 2.0), details, Color32::PLACEHOLDER);
  response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
