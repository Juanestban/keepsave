//! The vault name, its groups, Google Drive and the recycle bin.

use eframe::egui::{self, Align, Align2, Color32, CornerRadius, FontId, Label, Layout, Pos2, Rect, Response, RichText, ScrollArea, Sense, Ui, Vec2};
use egui_phosphor::regular as icon;

use super::{Action, GroupKind, GroupRow, MainScreen};
use crate::{
  drive::DriveState,
  screens::ScreenCtx,
  theme,
  widgets::{HOVER_TIME, glass_panel, icon_button, single_line},
};

const ROW_HEIGHT: f32 = 38.0;

pub fn show(ui: &mut Ui, rect: Rect, screen: &MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let Some(lists) = &screen.lists else {
    return;
  };
  let tr = |key| cx.i18n.tr(key);
  let searching = !screen.search.trim().is_empty();

  glass_panel(ui, rect, theme::GLASS_FILL, "main-sidebar", |ui| {
    ui.horizontal(|ui| {
      ui.label(RichText::new(icon::DATABASE).font(theme::icon_font(20.0)).color(theme::ACCENT));
      ui.add(Label::new(RichText::new(screen.vault().name()).font(theme::heading(17.0)).color(theme::TEXT)).truncate());
    });
    let file = screen.open.path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    ui.add(Label::new(RichText::new(file).small().color(theme::TEXT_FAINT)).truncate());
    ui.add_space(14.0);

    let header = Vec2::new(ui.available_width(), 26.0);
    ui.allocate_ui_with_layout(header, Layout::left_to_right(Align::Center), |ui| {
      ui.label(RichText::new(tr("main-groups").to_uppercase()).small().color(theme::TEXT_FAINT));
      ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        if icon_button(ui, icon::FOLDER_PLUS, &tr("main-new-group")).clicked() {
          actions.push(Action::NewGroup);
        }
      });
    });
    ui.add_space(4.0);

    let bottom_rows = usize::from(lists.bin.is_some()) + 1;
    let bin_space = if bottom_rows > 0 { bottom_rows as f32 * (ROW_HEIGHT + 2.0) + 12.0 } else { 0.0 };
    ScrollArea::vertical().id_salt("main-groups").max_height((ui.available_height() - bin_space).max(0.0)).auto_shrink([false, true]).show(
      ui,
      |ui| {
        ui.spacing_mut().item_spacing.y = 2.0;
        for row in &lists.groups {
          let selected = !searching && row.id == screen.group;
          let response = group_row(ui, row, &row.name, selected);
          if response.clicked() {
            actions.push(Action::SelectGroup(row.id));
          }
          if row.kind == GroupKind::Group {
            response.context_menu(|ui| {
              if ui.button(tr("group-rename")).clicked() {
                actions.push(Action::RenameGroup(row.id));
                ui.close();
              }
              if ui.button(tr("group-delete")).clicked() {
                actions.push(Action::DeleteGroup(row.id));
                ui.close();
              }
            });
          }
        }
      },
    );

    // Bottom up: the bin sits at the very bottom, Drive above it.
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
      ui.spacing_mut().item_spacing.y = 2.0;
      if let Some(bin) = &lists.bin {
        let selected = !searching && bin.id == screen.group;
        let response = group_row(ui, bin, &tr("main-recycle-bin"), selected);
        if response.clicked() {
          actions.push(Action::SelectGroup(bin.id));
        }
        response.context_menu(|ui| {
          let button = egui::Button::new(tr("bin-empty"));
          if ui.add_enabled(bin.count > 0, button).clicked() {
            actions.push(Action::EmptyBin);
            ui.close();
          }
        });
      }
      drive_section(ui, screen, cx, actions);
    });
  });
}

/// One row whose look and click depend on the Drive connection and on
/// whether this vault is linked.
fn drive_section(ui: &mut Ui, screen: &MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let tr = |key| cx.i18n.tr(key);
  let link = cx.config.drive_link(&screen.open.path);
  match cx.drive {
    DriveState::NotSetUp => {
      if drive_row(ui, icon::GOOGLE_DRIVE_LOGO, &tr("drive-setup"), theme::ACCENT, false).clicked() {
        actions.push(Action::ConnectDrive);
      }
    }
    DriveState::Disconnected { error } => {
      let mut response = drive_row(ui, icon::CLOUD_SLASH, &tr("drive-connect-short"), theme::ACCENT, false);
      if let Some(error) = error {
        response = response.on_hover_text(error);
      }
      if response.clicked() {
        actions.push(Action::ConnectDrive);
      }
    }
    DriveState::Connecting => {
      let response = drive_row(ui, icon::CLOUD, &tr("drive-waiting"), theme::TEXT_MUTED, true).on_hover_text(tr("drive-cancel-tooltip"));
      if response.clicked() {
        actions.push(Action::CancelDrive);
      }
    }
    DriveState::Connected(_) if screen.uploading => {
      drive_row(ui, icon::CLOUD_ARROW_UP, &tr("drive-uploading"), theme::TEXT_MUTED, true);
    }
    DriveState::Connected(_) => {
      let response = match link {
        Some(link) => {
          let sync = &screen.drive;
          let linked = cx.i18n.tr_arg("drive-linked-tooltip", "name", link.name.clone());
          let response = if sync.in_flight.is_some() {
            drive_row(ui, icon::CLOUD_ARROW_UP, &tr("sync-running"), theme::TEXT_MUTED, true)
          } else if let Some(error) = &sync.error {
            drive_row(ui, icon::WARNING_CIRCLE, &tr("sync-failed"), theme::DANGER, false).on_hover_text(error)
          } else if sync.offline {
            drive_row(ui, icon::CLOUD_SLASH, &tr("sync-offline"), theme::WARNING, false).on_hover_text(tr("sync-offline-tooltip"))
          } else if sync.pending {
            drive_row(ui, icon::CLOUD_ARROW_UP, &tr("sync-pending"), theme::TEXT_MUTED, false).on_hover_text(linked)
          } else {
            let label = match &sync.last_synced {
              Some(time) => cx.i18n.tr_arg("sync-done", "time", time.clone()),
              None => tr("drive-linked"),
            };
            drive_row(ui, icon::CLOUD_CHECK, &label, theme::SUCCESS, false).on_hover_text(linked)
          };
          if response.clicked() && sync.in_flight.is_none() {
            actions.push(Action::SyncNow);
          }
          response
        }
        None => {
          let response = drive_row(ui, icon::CLOUD_ARROW_UP, &tr("drive-upload"), theme::ACCENT, false);
          if response.clicked() {
            actions.push(Action::UploadToDrive);
          }
          response
        }
      };
      response.context_menu(|ui| {
        if link.is_some() && ui.add_enabled(screen.drive.in_flight.is_none(), egui::Button::new(tr("sync-now"))).clicked() {
          actions.push(Action::SyncNow);
          ui.close();
        }
        if link.is_some() && ui.button(tr("drive-unlink")).clicked() {
          actions.push(Action::UnlinkDrive);
          ui.close();
        }
        if ui.button(tr("drive-disconnect")).clicked() {
          actions.push(Action::DisconnectDrive);
          ui.close();
        }
      });
    }
  }
}

fn drive_row(ui: &mut Ui, glyph: &str, label: &str, color: Color32, busy: bool) -> Response {
  let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), ROW_HEIGHT), Sense::click());
  let hover = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
  let painter = ui.painter();
  painter.rect_filled(rect, CornerRadius::same(10), Color32::TRANSPARENT.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover));
  painter.text(Pos2::new(rect.left() + 20.0, rect.center().y), Align2::CENTER_CENTER, glyph, theme::icon_font(17.0), color);
  let reserve = if busy { 32.0 } else { 8.0 };
  let text = single_line(ui, label, FontId::proportional(14.5), color, rect.right() - reserve - (rect.left() + 40.0));
  ui.painter().galley(Pos2::new(rect.left() + 40.0, rect.center().y - text.size().y / 2.0), text, color);
  if busy {
    let spinner = Rect::from_center_size(Pos2::new(rect.right() - 18.0, rect.center().y), Vec2::splat(14.0));
    egui::Spinner::new().size(14.0).color(theme::TEXT_MUTED).paint_at(ui, spinner);
  }
  response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A selectable group: icon, name and entry count.
fn group_row(ui: &mut Ui, row: &GroupRow, name: &str, selected: bool) -> Response {
  let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), ROW_HEIGHT), Sense::click());
  let ctx = ui.ctx();
  let hover = ctx.animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
  let active = ctx.animate_bool_with_time(response.id.with("selected"), selected, HOVER_TIME);

  let fill = Color32::TRANSPARENT.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover).lerp_to_gamma(theme::GLASS_FILL_ACTIVE, active);
  let painter = ui.painter();
  painter.rect_filled(rect, CornerRadius::same(10), fill);
  if active > 0.0 {
    let bar = Rect::from_center_size(Pos2::new(rect.left() + 1.5, rect.center().y), Vec2::new(3.0, 16.0 * active));
    painter.rect_filled(bar, CornerRadius::same(2), theme::ACCENT.gamma_multiply(active));
  }

  let glyph = match row.kind {
    GroupKind::Root => icon::HOUSE,
    GroupKind::Group => icon::FOLDER_SIMPLE,
    GroupKind::Bin => icon::TRASH,
  };
  let icon_color = theme::TEXT_MUTED.lerp_to_gamma(theme::ACCENT, active);
  painter.text(Pos2::new(rect.left() + 20.0, rect.center().y), Align2::CENTER_CENTER, glyph, theme::icon_font(17.0), icon_color);

  let count = row.count.to_string();
  let count_galley = single_line(ui, &count, FontId::proportional(12.5), theme::TEXT_FAINT, 60.0);
  let count_pos = Pos2::new(rect.right() - 12.0 - count_galley.size().x, rect.center().y - count_galley.size().y / 2.0);
  let name_width = count_pos.x - (rect.left() + 40.0) - 8.0;
  let text_color = theme::TEXT_MUTED.lerp_to_gamma(theme::TEXT, hover.max(active));
  let name_galley = single_line(ui, name, FontId::proportional(14.5), text_color, name_width);
  let name_pos = Pos2::new(rect.left() + 40.0, rect.center().y - name_galley.size().y / 2.0);
  let painter = ui.painter();
  painter.galley(name_pos, name_galley, text_color);
  painter.galley(count_pos, count_galley, theme::TEXT_FAINT);

  response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
