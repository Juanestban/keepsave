//! The vault name, its groups and the recycle bin.

use eframe::egui::{self, Align, Align2, Color32, CornerRadius, FontId, Label, Layout, Pos2, Rect, Response, RichText, ScrollArea, Sense, Ui, Vec2};
use egui_phosphor::regular as icon;

use super::{Action, GroupKind, GroupRow, MainScreen};
use crate::{
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

    let bin_space = if lists.bin.is_some() { ROW_HEIGHT + 12.0 } else { 0.0 };
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

    if let Some(bin) = &lists.bin {
      ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
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
      });
    }
  });
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
