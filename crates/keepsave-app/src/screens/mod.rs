//! Screens and the navigation between them.

mod create;
mod unlock;
mod vault;
mod welcome;

use std::{f32::consts::TAU, io, path::PathBuf};

use eframe::egui::{self, Align, Color32, Layout, Pos2, RichText, Sense, Shape, Stroke, Ui, Vec2};
use fluent_bundle::FluentArgs;
use keepsave_core::CoreError;

pub use create::CreateVault;
pub use unlock::Unlock;
pub use vault::VaultHome;
pub use welcome::Welcome;

use crate::{
    config::Config,
    i18n::{I18n, Language},
    theme,
    worker::{OpenVault, Worker},
};

/// What a screen can read while drawing.
pub struct ScreenCtx<'a> {
    pub i18n: &'a I18n,
    pub worker: &'a Worker,
    pub config: &'a Config,
    /// Vertical offset of the entrance animation, in points.
    pub lift: f32,
}

/// A request from a screen to the app.
pub enum Nav {
    Welcome,
    Create,
    /// `select` preselects a vault, e.g. one picked from a file dialog.
    Unlock {
        select: Option<PathBuf>,
    },
    Opened(Box<OpenVault>),
    Lock(PathBuf),
    SetLanguage(Language),
    ForgetVault(PathBuf),
}

pub fn error_message(i18n: &I18n, error: &CoreError) -> String {
    match error {
        CoreError::InvalidKey => i18n.tr("unlock-error-wrong-password"),
        CoreError::Io(io_error) if io_error.kind() == io::ErrorKind::NotFound => {
            i18n.tr("unlock-error-not-found")
        }
        other => {
            let mut args = FluentArgs::new();
            args.set("details", other.to_string());
            i18n.tr_args("error-generic", &args)
        }
    }
}

/// The round glass emblem at the top of a screen. While `busy`, an accent
/// arc orbits it and the glow breathes.
pub fn emblem(ui: &mut Ui, icon: &str, busy: bool) {
    let radius = 38.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(radius * 2.0 + 32.0), Sense::hover());
    let center = rect.center();
    let time = ui.input(|input| input.time) as f32;
    let busy_t = ui
        .ctx()
        .animate_bool_with_time(ui.id().with("emblem-busy"), busy, 0.3);
    let breath = busy_t * (0.5 + 0.5 * (time * 3.0).sin());

    let painter = ui.painter();
    for ring in 1..=4 {
        let spread = ring as f32 * (4.0 + 2.0 * breath);
        let alpha = 0.06 + 0.04 * breath;
        painter.circle_filled(center, radius + spread, theme::ACCENT.gamma_multiply(alpha));
    }
    painter.circle_filled(center, radius, Color32::from_rgb(28, 32, 52));
    painter.circle_stroke(center, radius, Stroke::new(1.0, theme::GLASS_STROKE));
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        icon,
        theme::icon_font(34.0),
        theme::TEXT,
    );

    if busy_t > 0.0 {
        let start = time * 4.0;
        let points: Vec<Pos2> = (0..=24)
            .map(|i| {
                let angle = start + i as f32 / 24.0 * TAU * 0.3;
                center + (radius + 6.0) * Vec2::angled(angle)
            })
            .collect();
        painter.add(Shape::line(
            points,
            Stroke::new(2.5, theme::ACCENT.gamma_multiply(busy_t)),
        ));
        ui.ctx().request_repaint();
    }
}

/// Screen title and an optional muted subtitle, centered.
pub fn title(ui: &mut Ui, title: &str, subtitle: Option<&str>) {
    ui.vertical_centered(|ui| {
        ui.label(
            RichText::new(title)
                .font(theme::heading(26.0))
                .color(theme::TEXT),
        );
        if let Some(subtitle) = subtitle {
            ui.label(RichText::new(subtitle).color(theme::TEXT_MUTED));
        }
    });
}

/// Two links side by side, centered as a pair.
pub fn link_pair(ui: &mut Ui, left: &str, right: &str) -> (bool, bool) {
    let mut clicked = (false, false);
    // `Align::Min`: a centered cross axis would center the links in all the
    // height left below them.
    ui.columns(2, |columns| {
        columns[0].with_layout(Layout::right_to_left(Align::Min), |ui| {
            clicked.0 = crate::widgets::link(ui, left).clicked();
        });
        columns[1].with_layout(Layout::left_to_right(Align::Min), |ui| {
            clicked.1 = crate::widgets::link(ui, right).clicked();
        });
    });
    clicked
}
