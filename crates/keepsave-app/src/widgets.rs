//! Glass building blocks shared by every screen.

use eframe::egui::{
    self, Align2, Color32, CornerRadius, Frame, Id, InnerResponse, Mesh, Pos2, Rect, Response,
    RichText, Sense, Shadow, Stroke, StrokeKind, TextEdit, Ui, UiBuilder, Vec2, WidgetInfo,
    WidgetText, WidgetType,
    text::{LayoutJob, TextFormat},
};
use egui_phosphor::regular as icon;
use zeroize::Zeroizing;

use crate::theme;

const HOVER_TIME: f32 = 0.12;

/// A translucent card with a luminous border, a top-edge highlight and a
/// soft shadow.
pub fn glass_card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let response = Frame::new()
        .fill(theme::GLASS_FILL)
        .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
        .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
        .inner_margin(theme::CARD_MARGIN)
        .shadow(Shadow {
            offset: [0, 18],
            blur: 48,
            spread: 0,
            color: theme::SHADOW,
        })
        .show(ui, add);
    paint_highlight(ui, response.response.rect, theme::RADIUS_PANEL as f32);
    response
}

/// A thin line along the top edge that fades out toward the corners.
fn paint_highlight(ui: &Ui, rect: Rect, radius: f32) {
    let left = rect.left() + radius;
    let right = rect.right() - radius;
    if right <= left {
        return;
    }
    let top = rect.top() + 0.5;
    let bottom = top + 1.0;
    let middle = (left + right) / 2.0;
    let mut mesh = Mesh::default();
    for (x, color) in [
        (left, Color32::TRANSPARENT),
        (middle, theme::GLASS_HIGHLIGHT),
        (right, Color32::TRANSPARENT),
    ] {
        mesh.colored_vertex(Pos2::new(x, top), color);
        mesh.colored_vertex(Pos2::new(x, bottom), color);
    }
    for i in [0, 2] {
        mesh.add_triangle(i, i + 1, i + 2);
        mesh.add_triangle(i + 1, i + 2, i + 3);
    }
    ui.painter().add(mesh);
}

/// Lays out `add` in a column of `width`, centered in the available space.
/// The height comes from the previous frame, so the first frame of a new
/// layout requests one more repaint to settle. `offset` shifts the column,
/// for shake and slide animations.
pub fn centered_column<R>(
    ui: &mut Ui,
    id: Id,
    width: f32,
    offset: Vec2,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    // Room for card shadows, which the scroll area would otherwise clip.
    const PADDING: f32 = 40.0;

    let available = ui.available_rect_before_wrap();
    let width = width.min(available.width() - 32.0).max(0.0);
    let last_height: f32 = ui.data(|data| data.get_temp(id)).unwrap_or(0.0);
    let top = (available.center().y - last_height / 2.0).max(available.top());
    // Full window width, so the scrollbar sits at the edge and shadows on
    // the sides are not clipped.
    let viewport = Rect::from_min_max(Pos2::new(available.left(), top), available.max);

    let inner = ui.scope_builder(UiBuilder::new().id_salt(id).max_rect(viewport), |ui| {
        egui::ScrollArea::vertical()
            .max_height(viewport.height())
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.add_space(PADDING + offset.y);
                let column = Rect::from_min_size(
                    Pos2::new(
                        viewport.center().x - width / 2.0 + offset.x,
                        ui.cursor().top(),
                    ),
                    Vec2::new(width, viewport.height()),
                );
                let inner = ui
                    .scope_builder(UiBuilder::new().max_rect(column), add)
                    .inner;
                ui.add_space(PADDING - offset.y);
                inner
            })
            .inner
    });
    let height = inner.response.rect.height();
    if (height - last_height).abs() > 0.5 {
        ui.data_mut(|data| data.insert_temp(id, height));
        ui.ctx().request_repaint();
    }
    inner.inner
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Solid accent fill, for the main action of a screen.
    Primary,
    /// Translucent glass, for everything else.
    Glass,
}

/// A large rounded button with an icon, a hover fade and a loading state.
pub struct ActionButton<'a> {
    icon: Option<&'a str>,
    label: String,
    style: ButtonStyle,
    enabled: bool,
    loading: bool,
    full_width: bool,
}

impl<'a> ActionButton<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            icon: None,
            label: label.into(),
            style: ButtonStyle::Glass,
            enabled: true,
            loading: false,
            full_width: false,
        }
    }

    pub fn primary(mut self) -> Self {
        self.style = ButtonStyle::Primary;
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
}

impl egui::Widget for ActionButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let format = |font_id| TextFormat {
            font_id,
            color: Color32::PLACEHOLDER,
            valign: egui::Align::Center,
            ..TextFormat::default()
        };
        let mut job = LayoutJob::default();
        if let Some(icon) = self.icon {
            job.append(icon, 0.0, format(theme::icon_font(17.0)));
        }
        let gap = if self.icon.is_some() { 10.0 } else { 0.0 };
        job.append(&self.label, gap, format(theme::heading(15.0)));
        let galley = WidgetText::from(job).into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            ui.available_width() - 32.0,
            egui::TextStyle::Button,
        );
        let width = if self.full_width {
            ui.available_width()
        } else {
            galley.size().x + 40.0
        };
        let interactive = self.enabled && !self.loading;
        let sense = if interactive {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, theme::CONTROL_HEIGHT), sense);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, interactive, &self.label));

        if ui.is_rect_visible(rect) {
            let hover = ui.ctx().animate_bool_with_time(
                response.id.with("hover"),
                interactive && response.hovered(),
                HOVER_TIME,
            );
            let pressed = interactive && response.is_pointer_button_down_on();
            let (fill, text_color, stroke) = match self.style {
                ButtonStyle::Primary => (
                    theme::ACCENT.lerp_to_gamma(theme::ACCENT_HOVER, hover),
                    theme::ON_ACCENT,
                    Stroke::NONE,
                ),
                ButtonStyle::Glass => (
                    theme::GLASS_FILL.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover),
                    theme::TEXT,
                    Stroke::new(1.0, theme::GLASS_STROKE),
                ),
            };
            let dim = if self.enabled { 1.0 } else { 0.4 };
            let rect = if pressed { rect.shrink(1.0) } else { rect };
            ui.painter().rect(
                rect,
                CornerRadius::same(theme::RADIUS_CONTROL),
                fill.gamma_multiply(dim),
                Stroke::new(stroke.width, stroke.color.gamma_multiply(dim)),
                StrokeKind::Inside,
            );
            if self.loading {
                let spinner = Rect::from_center_size(rect.center(), Vec2::splat(18.0));
                egui::Spinner::new().color(text_color).paint_at(ui, spinner);
            } else {
                let pos = rect.center() - galley.size() / 2.0;
                ui.painter()
                    .galley(pos, galley, text_color.gamma_multiply(dim));
            }
        }
        response
    }
}

/// A frameless icon button with a circular hover glow.
pub fn icon_button(ui: &mut Ui, icon: &str, tooltip: &str) -> Response {
    let size = Vec2::splat(30.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, tooltip));
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
    ui.painter().circle_filled(
        rect.center(),
        size.x / 2.0,
        Color32::TRANSPARENT.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover),
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        icon,
        theme::icon_font(17.0),
        theme::TEXT_MUTED.lerp_to_gamma(theme::TEXT, hover),
    );
    response.on_hover_text(tooltip)
}

/// A text-only button in the accent color.
pub fn link(ui: &mut Ui, text: &str) -> Response {
    let response = ui.add(
        egui::Button::new(RichText::new(text).color(theme::ACCENT))
            .frame(false)
            .min_size(Vec2::new(0.0, 28.0)),
    );
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A muted caption above a field.
pub fn field_label(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).small().color(theme::TEXT_MUTED));
}

pub struct PasswordFieldResponse {
    pub response: Response,
    /// Enter was pressed while the field had focus.
    pub submitted: bool,
}

/// A masked single-line input with a reveal toggle and an animated focus ring.
/// The buffer is wiped when dropped.
pub fn password_field(
    ui: &mut Ui,
    id: Id,
    buffer: &mut Zeroizing<String>,
    reveal: &mut bool,
    enabled: bool,
    tooltips: (&str, &str),
) -> PasswordFieldResponse {
    let focused = ui.memory(|memory| memory.has_focus(id));
    let focus = ui
        .ctx()
        .animate_bool_with_time(id.with("focus"), focused, HOVER_TIME);
    let stroke = theme::GLASS_STROKE.lerp_to_gamma(theme::ACCENT, focus);

    let inner = input_frame(stroke).show(ui, |ui| {
        ui.horizontal(|ui| {
            let edit_width = ui.available_width() - 40.0;
            let response = ui.add_enabled(
                enabled,
                TextEdit::singleline(&mut **buffer)
                    .id(id)
                    .password(!*reveal)
                    .frame(Frame::NONE)
                    .desired_width(edit_width)
                    .font(egui::TextStyle::Body)
                    .min_size(Vec2::new(0.0, 30.0))
                    .vertical_align(egui::Align::Center),
            );
            let (show, hide) = tooltips;
            let toggle = if *reveal {
                icon_button(ui, icon::EYE_SLASH, hide)
            } else {
                icon_button(ui, icon::EYE, show)
            };
            if toggle.clicked() {
                *reveal = !*reveal;
            }
            response
        })
        .inner
    });
    let response = inner.inner;
    let submitted = response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    PasswordFieldResponse {
        response,
        submitted,
    }
}

/// A single-line text input styled like [`password_field`].
pub fn text_field(ui: &mut Ui, id: Id, text: &mut String, enabled: bool) -> Response {
    let focused = ui.memory(|memory| memory.has_focus(id));
    let focus = ui
        .ctx()
        .animate_bool_with_time(id.with("focus"), focused, HOVER_TIME);
    let stroke = theme::GLASS_STROKE.lerp_to_gamma(theme::ACCENT, focus);
    input_frame(stroke)
        .show(ui, |ui| {
            ui.add_enabled(
                enabled,
                TextEdit::singleline(text)
                    .id(id)
                    .frame(Frame::NONE)
                    .desired_width(ui.available_width())
                    .font(egui::TextStyle::Body)
                    .min_size(Vec2::new(0.0, 30.0))
                    .vertical_align(egui::Align::Center),
            )
        })
        .inner
}

fn input_frame(stroke: Color32) -> Frame {
    Frame::new()
        .fill(theme::INPUT_FILL)
        .stroke(Stroke::new(1.0, stroke))
        .corner_radius(CornerRadius::same(theme::RADIUS_CONTROL))
        .inner_margin(egui::Margin::symmetric(12, 4))
}

/// Four-step strength bar driven by an entropy estimate in bits.
pub fn strength_meter(ui: &mut Ui, bits: f64, labels: [&str; 4], bits_label: &str) {
    let level = match bits {
        b if b < 45.0 => 0,
        b if b < 65.0 => 1,
        b if b < 90.0 => 2,
        _ => 3,
    };
    let color = [theme::DANGER, theme::WARNING, theme::ACCENT, theme::SUCCESS][level];
    let filled = if bits <= 0.0 { 0 } else { level + 1 };

    // A short row: `horizontal` would reserve a full control height.
    let row = Vec2::new(ui.available_width(), 16.0);
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    ui.allocate_ui_with_layout(row, layout, |ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let width = ((ui.available_width() - 140.0) / 4.0).max(12.0);
        for step in 0..4 {
            let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 4.0), Sense::hover());
            let target = if step < filled { 1.0 } else { 0.0 };
            let t = ui
                .ctx()
                .animate_value_with_time(ui.id().with(("strength", step)), target, 0.2);
            ui.painter()
                .rect_filled(rect, CornerRadius::same(2), theme::GLASS_FILL_ACTIVE);
            let mut fill = rect;
            fill.set_width(rect.width() * t);
            ui.painter().rect_filled(fill, CornerRadius::same(2), color);
        }
        if bits > 0.0 {
            ui.label(RichText::new(labels[level]).small().color(color));
            ui.label(RichText::new(bits_label).small().color(theme::TEXT_FAINT));
        }
    });
}

/// An inline message with an icon, for errors and warnings.
pub fn notice(ui: &mut Ui, icon: &str, text: &str, color: Color32) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(
            RichText::new(icon)
                .font(theme::icon_font(14.0))
                .color(color),
        );
        ui.label(RichText::new(text).small().color(color));
    });
}
