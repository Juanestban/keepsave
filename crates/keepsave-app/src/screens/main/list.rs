//! The entries of the selected group, or the search results.

use eframe::egui::{
    self, Align, Color32, CornerRadius, FontId, Layout, Pos2, Rect, RichText, ScrollArea, Sense,
    Ui, UiBuilder, Vec2,
};
use egui_phosphor::regular as icon;

use super::{Action, Field, GroupKind, MainScreen, Row};
use crate::{
    screens::ScreenCtx,
    theme,
    widgets::{ActionButton, HOVER_TIME, avatar, glass_panel, icon_button, single_line},
};

const ROW_HEIGHT: f32 = 58.0;

pub fn show(
    ui: &mut Ui,
    rect: Rect,
    screen: &mut MainScreen,
    cx: &ScreenCtx,
    actions: &mut Vec<Action>,
) {
    let scroll_to_selected = std::mem::take(&mut screen.scroll_to_selected);
    let Some(lists) = &screen.lists else {
        return;
    };
    let tr = |key| cx.i18n.tr(key);
    let query = screen.search.trim();
    let kind = lists
        .groups
        .iter()
        .chain(&lists.bin)
        .find(|group| group.id == screen.group)
        .map(|group| group.kind);

    glass_panel(ui, rect, theme::GLASS_FILL, "main-list", |ui| {
        let title = if !query.is_empty() {
            cx.i18n.tr_arg("main-results", "query", query.to_owned())
        } else if kind == Some(GroupKind::Bin) {
            tr("main-recycle-bin")
        } else {
            lists
                .groups
                .iter()
                .find(|group| group.id == screen.group)
                .map(|group| group.name.clone())
                .unwrap_or_default()
        };
        ui.horizontal(|ui| {
            ui.add(
                egui::Label::new(
                    RichText::new(title)
                        .font(theme::heading(18.0))
                        .color(theme::TEXT),
                )
                .truncate(),
            );
            ui.label(
                RichText::new(cx.i18n.tr_arg("entries-count", "count", lists.rows.len()))
                    .small()
                    .color(theme::TEXT_FAINT),
            );
        });
        ui.add_space(8.0);

        if lists.rows.is_empty() {
            let (glyph, message) = if !query.is_empty() {
                (icon::MAGNIFYING_GLASS, tr("main-no-results"))
            } else if kind == Some(GroupKind::Bin) {
                (icon::TRASH, tr("main-empty-bin"))
            } else {
                (icon::KEY, tr("main-empty-group"))
            };
            empty_state(
                ui,
                glyph,
                &message,
                query.is_empty() && kind != Some(GroupKind::Bin),
                cx,
                actions,
            );
            return;
        }

        ScrollArea::vertical()
            .id_salt("main-entries")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(ui.available_width() - 8.0);
                ui.spacing_mut().item_spacing.y = 2.0;
                for row in &lists.rows {
                    let selected = screen.selected == Some(row.entry.id);
                    entry_row(
                        ui,
                        row,
                        selected,
                        scroll_to_selected && selected,
                        cx,
                        actions,
                    );
                }
            });
    });
}

fn entry_row(
    ui: &mut Ui,
    row: &Row,
    selected: bool,
    scroll_to: bool,
    cx: &ScreenCtx,
    actions: &mut Vec<Action>,
) {
    let tr = |key| cx.i18n.tr(key);
    let id = row.entry.id;
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), ROW_HEIGHT), Sense::click());
    if scroll_to {
        response.scroll_to_me(None);
    }
    if response.clicked() {
        actions.push(Action::SelectEntry(id));
    }
    if response.double_clicked() {
        actions.push(Action::EditEntry);
    }
    if !ui.is_rect_visible(rect) {
        return;
    }

    let pointer_inside = ui.rect_contains_pointer(rect);
    let ctx = ui.ctx();
    let hover = ctx.animate_bool_with_time(response.id.with("hover"), pointer_inside, HOVER_TIME);
    let active = ctx.animate_bool_with_time(response.id.with("selected"), selected, HOVER_TIME);
    let fill = Color32::TRANSPARENT
        .lerp_to_gamma(theme::GLASS_FILL_HOVER, hover)
        .lerp_to_gamma(theme::ACCENT.gamma_multiply(0.16), active);
    ui.painter()
        .rect_filled(rect, CornerRadius::same(theme::RADIUS_CONTROL), fill);

    let title = if row.entry.title.is_empty() {
        tr("detail-untitled")
    } else {
        row.entry.title.clone()
    };
    avatar(
        ui,
        Pos2::new(rect.left() + 30.0, rect.center().y),
        18.0,
        &title,
    );

    // Room on the right for the group label, or the copy buttons on hover.
    let buttons_width = 76.0;
    let group = row.group.as_deref().map(|name| {
        single_line(
            ui,
            name,
            FontId::proportional(12.0),
            theme::TEXT_FAINT,
            (rect.width() * 0.3).min(140.0),
        )
    });
    let right_reserve = group
        .as_ref()
        .map_or(0.0, |galley| galley.size().x + 16.0)
        .max(if pointer_inside { buttons_width } else { 0.0 });
    let text_left = rect.left() + 60.0;
    let text_width = rect.right() - right_reserve - text_left - 8.0;

    let title_galley = single_line(ui, &title, theme::heading(14.5), theme::TEXT, text_width);
    let has_username = !row.entry.username.is_empty();
    let title_y = if has_username {
        rect.center().y - title_galley.size().y + 1.0
    } else {
        rect.center().y - title_galley.size().y / 2.0
    };
    ui.painter()
        .galley(Pos2::new(text_left, title_y), title_galley, theme::TEXT);
    if has_username {
        let user = single_line(
            ui,
            &row.entry.username,
            FontId::proportional(12.5),
            theme::TEXT_MUTED,
            text_width,
        );
        ui.painter().galley(
            Pos2::new(text_left, rect.center().y + 2.0),
            user,
            theme::TEXT_MUTED,
        );
    }

    if pointer_inside {
        let buttons = Rect::from_min_max(
            Pos2::new(rect.right() - buttons_width, rect.top()),
            Pos2::new(rect.right() - 8.0, rect.bottom()),
        );
        let mut bar = ui.new_child(
            UiBuilder::new()
                .max_rect(buttons)
                .layout(Layout::right_to_left(Align::Center)),
        );
        if icon_button(&mut bar, icon::PASSWORD, &tr("main-copy-password")).clicked() {
            actions.push(Action::Copy(id, Field::Password));
        }
        if has_username && icon_button(&mut bar, icon::USER, &tr("main-copy-username")).clicked() {
            actions.push(Action::Copy(id, Field::Username));
        }
    } else if let Some(galley) = group {
        let pos = Pos2::new(
            rect.right() - 12.0 - galley.size().x,
            rect.center().y - galley.size().y / 2.0,
        );
        let chip = Rect::from_min_size(pos, galley.size()).expand2(Vec2::new(7.0, 3.0));
        ui.painter()
            .rect_filled(chip, CornerRadius::same(8), theme::GLASS_FILL);
        ui.painter().galley(pos, galley, theme::TEXT_FAINT);
    }
}

fn empty_state(
    ui: &mut Ui,
    glyph: &str,
    message: &str,
    offer_new: bool,
    cx: &ScreenCtx,
    actions: &mut Vec<Action>,
) {
    ui.add_space((ui.available_height() / 2.0 - 90.0).max(12.0));
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(glyph)
                .font(theme::icon_font(40.0))
                .color(theme::TEXT_FAINT),
        );
        ui.add_space(4.0);
        ui.label(RichText::new(message).color(theme::TEXT_MUTED));
        if offer_new {
            ui.add_space(8.0);
            let button = ActionButton::new(cx.i18n.tr("main-new-entry")).icon(icon::PLUS);
            if ui.add(button).clicked() {
                actions.push(Action::NewEntry);
            }
        }
    });
}
