//! Embedded Inter (SIL OFL 1.1, see `assets/fonts/OFL.txt`) and Phosphor icons.

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

const INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");

/// Family name for headings and buttons.
pub const SEMIBOLD: &str = "inter-semibold";
/// Family name for Phosphor icons; see [`crate::theme::icon_font`].
pub const ICONS: &str = "icons";

pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert("inter".into(), FontData::from_static(INTER_REGULAR).into());
    fonts.font_data.insert(
        SEMIBOLD.into(),
        FontData::from_static(INTER_SEMIBOLD).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "inter".into());

    // Icons right after Inter, ahead of egui's emoji fallbacks.
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    // Semibold first, then everything proportional text falls back to,
    // so missing glyphs still render in headings.
    let mut semibold = vec![SEMIBOLD.to_owned()];
    semibold.extend(fonts.families[&FontFamily::Proportional].iter().cloned());
    fonts
        .families
        .insert(FontFamily::Name(SEMIBOLD.into()), semibold);

    // Inter has its own glyphs in part of the private-use range Phosphor
    // uses, so icons are drawn from a family that holds only Phosphor.
    fonts
        .families
        .insert(FontFamily::Name(ICONS.into()), vec!["phosphor".into()]);

    ctx.set_fonts(fonts);
}
