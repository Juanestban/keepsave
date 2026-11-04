//! Design tokens for the dark liquid-glass look, and the egui style built
//! from them. Translucent colors are premultiplied so they can be `const`.

use eframe::egui::{
    self, Color32, CornerRadius, FontFamily, FontId, Margin, Stroke, TextStyle, Theme, Vec2,
};

use crate::fonts;

/// Behind the shader background, and the fallback when there is no GPU.
pub const BASE: Color32 = Color32::from_rgb(9, 10, 16);

pub const TEXT: Color32 = Color32::from_rgb(238, 240, 247);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(150, 156, 175);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(98, 104, 124);

pub const ACCENT: Color32 = Color32::from_rgb(132, 160, 255);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(162, 184, 255);
/// Text on accent-filled surfaces.
pub const ON_ACCENT: Color32 = Color32::from_rgb(12, 16, 36);
pub const DANGER: Color32 = Color32::from_rgb(255, 120, 140);
pub const SUCCESS: Color32 = Color32::from_rgb(110, 231, 183);
pub const WARNING: Color32 = Color32::from_rgb(252, 211, 77);

/// White at the given opacity, premultiplied.
const fn white(alpha: u8) -> Color32 {
    Color32::from_rgba_premultiplied(alpha, alpha, alpha, alpha)
}

/// Black at the given opacity, premultiplied.
const fn black(alpha: u8) -> Color32 {
    Color32::from_rgba_premultiplied(0, 0, 0, alpha)
}

pub const GLASS_FILL: Color32 = white(14);
pub const GLASS_FILL_HOVER: Color32 = white(26);
pub const GLASS_FILL_ACTIVE: Color32 = white(36);
pub const GLASS_STROKE: Color32 = white(34);
/// The specular line along a panel's top edge.
pub const GLASS_HIGHLIGHT: Color32 = white(110);
pub const INPUT_FILL: Color32 = black(70);
pub const SHADOW: Color32 = black(110);

pub const RADIUS_PANEL: u8 = 22;
pub const RADIUS_CONTROL: u8 = 12;
pub const CONTROL_HEIGHT: f32 = 42.0;

pub fn heading(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(fonts::SEMIBOLD.into()))
}

/// Use for every Phosphor glyph, never a text font.
pub fn icon_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(fonts::ICONS.into()))
}

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(Theme::Dark);
    ctx.style_mut_of(Theme::Dark, |style| {
        style.text_styles = [
            (TextStyle::Heading, heading(26.0)),
            (TextStyle::Body, FontId::proportional(15.0)),
            (TextStyle::Button, FontId::proportional(15.0)),
            (TextStyle::Small, FontId::proportional(12.5)),
            (TextStyle::Monospace, FontId::monospace(14.0)),
        ]
        .into();

        let spacing = &mut style.spacing;
        spacing.item_spacing = Vec2::new(10.0, 10.0);
        spacing.button_padding = Vec2::new(14.0, 8.0);
        spacing.interact_size.y = 32.0;

        let visuals = &mut style.visuals;
        visuals.dark_mode = true;
        visuals.override_text_color = None;
        visuals.panel_fill = Color32::TRANSPARENT;
        visuals.window_fill = Color32::from_rgb(22, 24, 34);
        visuals.window_stroke = Stroke::new(1.0, GLASS_STROKE);
        visuals.window_corner_radius = CornerRadius::same(RADIUS_PANEL);
        visuals.extreme_bg_color = INPUT_FILL;
        visuals.faint_bg_color = GLASS_FILL;
        visuals.hyperlink_color = ACCENT;
        visuals.selection.bg_fill = ACCENT.gamma_multiply(0.35);
        visuals.selection.stroke = Stroke::new(1.0, ACCENT);
        visuals.text_cursor.stroke = Stroke::new(2.0, ACCENT);
        visuals.error_fg_color = DANGER;
        visuals.warn_fg_color = WARNING;

        let radius = CornerRadius::same(RADIUS_CONTROL);
        let widgets = &mut visuals.widgets;
        for (state, fill, text) in [
            (
                &mut widgets.noninteractive,
                Color32::TRANSPARENT,
                TEXT_MUTED,
            ),
            (&mut widgets.inactive, GLASS_FILL, TEXT),
            (&mut widgets.hovered, GLASS_FILL_HOVER, TEXT),
            (&mut widgets.active, GLASS_FILL_ACTIVE, TEXT),
            (&mut widgets.open, GLASS_FILL_HOVER, TEXT),
        ] {
            state.bg_fill = fill;
            state.weak_bg_fill = fill;
            state.fg_stroke = Stroke::new(1.0, text);
            state.corner_radius = radius;
            state.expansion = 0.0;
        }
        widgets.noninteractive.bg_stroke = Stroke::new(1.0, GLASS_STROKE);
        widgets.inactive.bg_stroke = Stroke::NONE;
        widgets.hovered.bg_stroke = Stroke::new(1.0, GLASS_STROKE);
        widgets.active.bg_stroke = Stroke::new(1.0, ACCENT.gamma_multiply(0.6));
    });
}

/// Inner padding of glass cards.
pub const CARD_MARGIN: Margin = Margin::same(28);
