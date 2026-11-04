//! Recent vaults as glass cards and the master password field. Argon2 runs
//! on the worker while the emblem animates.

use std::path::{Path, PathBuf};

use eframe::egui::{
    self, Align, CornerRadius, Frame, Id, Label, Layout, RichText, ScrollArea, Sense, Stroke, Ui,
    UiBuilder, Vec2,
};
use egui_phosphor::regular as icon;
use keepsave_core::{CoreError, SecretString};
use zeroize::Zeroizing;

use super::{Nav, ScreenCtx, emblem, error_message, link_pair, title};
use crate::{
    config::Config,
    theme,
    widgets::{
        ActionButton, centered_column, field_label, glass_card, icon_button, notice, password_field,
    },
    worker::Event,
};

const SHAKE_TIME: f64 = 0.45;
const PASSWORD_ID: &str = "unlock-password";

pub struct Unlock {
    selected: Option<PathBuf>,
    password: Zeroizing<String>,
    reveal: bool,
    error: Option<String>,
    busy: bool,
    focus_password: bool,
    shake_requested: bool,
    shake_started: Option<f64>,
}

impl Unlock {
    pub fn new(select: Option<PathBuf>, config: &Config) -> Self {
        Self {
            selected: select.or_else(|| config.recent_vaults.first().cloned()),
            password: Zeroizing::default(),
            reveal: false,
            error: None,
            busy: false,
            focus_password: true,
            shake_requested: false,
            shake_started: None,
        }
    }

    /// Called after `path` was removed from the recent list.
    pub fn forgotten(&mut self, path: &Path, config: &Config) {
        if self.selected.as_deref() == Some(path) {
            self.selected = config.recent_vaults.first().cloned();
            self.error = None;
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
        let tr = |key| cx.i18n.tr(key);
        let mut nav = None;
        let mut submit = false;
        let offset = Vec2::new(self.shake_offset(ui), cx.lift);

        // A vault picked from the dialog may not be in the recent list yet.
        let mut vaults = cx.config.recent_vaults.clone();
        if let Some(selected) = &self.selected
            && !vaults.contains(selected)
        {
            vaults.insert(0, selected.clone());
        }

        centered_column(ui, Id::new("unlock"), 460.0, offset, |ui| {
            let icon = if self.busy {
                icon::LOCK_OPEN
            } else {
                icon::LOCK
            };
            ui.vertical_centered(|ui| emblem(ui, icon, self.busy));
            title(ui, &tr("unlock-title"), None);
            ui.add_space(16.0);

            glass_card(ui, |ui| {
                field_label(ui, &tr("unlock-recent"));
                ScrollArea::vertical()
                    .max_height(232.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 8.0;
                        for path in &vaults {
                            let selected = self.selected.as_ref() == Some(path);
                            match vault_card(ui, cx, path, selected, !self.busy) {
                                CardAction::Select if !selected => {
                                    self.selected = Some(path.clone());
                                    self.error = None;
                                    self.focus_password = true;
                                }
                                CardAction::Forget => nav = Some(Nav::ForgetVault(path.clone())),
                                _ => {}
                            }
                        }
                    });
                ui.add_space(6.0);

                field_label(ui, &tr("unlock-password"));
                let field = password_field(
                    ui,
                    Id::new(PASSWORD_ID),
                    &mut self.password,
                    &mut self.reveal,
                    !self.busy,
                    (&tr("reveal-show"), &tr("reveal-hide")),
                );
                if std::mem::take(&mut self.focus_password) {
                    field.response.request_focus();
                }
                submit |= field.submitted;

                match (&self.error, &self.selected) {
                    (Some(error), _) => notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER),
                    (None, None) => notice(ui, icon::INFO, &tr("unlock-select"), theme::TEXT_MUTED),
                    _ => {}
                }
                ui.add_space(4.0);

                let button = ActionButton::new(tr("unlock-submit"))
                    .icon(icon::LOCK_KEY_OPEN)
                    .primary()
                    .full_width()
                    .enabled(self.selected.is_some() && !self.password.is_empty())
                    .loading(self.busy);
                submit |= ui.add(button).clicked();
            });

            ui.add_space(8.0);
            if !self.busy {
                let (open_other, create) =
                    link_pair(ui, &tr("unlock-open-other"), &tr("unlock-create"));
                if open_other {
                    cx.worker
                        .pick_vault_file(tr("dialog-open-title"), tr("dialog-filter"));
                }
                if create {
                    nav = Some(Nav::Create);
                }
            }
        });

        if submit {
            self.submit(cx);
        }
        nav
    }

    fn submit(&mut self, cx: &ScreenCtx) {
        let Some(path) = self.selected.clone() else {
            return;
        };
        if self.busy || self.password.is_empty() {
            return;
        }
        self.busy = true;
        self.error = None;
        cx.worker
            .unlock_vault(path, SecretString::from(self.password.as_str()));
    }

    pub fn on_event(&mut self, event: Event, cx: &ScreenCtx) -> Option<Nav> {
        match event {
            Event::VaultUnlocked(result) => {
                self.busy = false;
                self.password = Zeroizing::default();
                match result {
                    Ok(open) => return Some(Nav::Opened(Box::new(open))),
                    Err(error) => {
                        if matches!(error, CoreError::InvalidKey) {
                            self.shake_requested = true;
                        }
                        self.error = Some(error_message(cx.i18n, &error));
                        self.focus_password = true;
                    }
                }
            }
            Event::VaultFilePicked(Some(path)) => {
                self.selected = Some(path);
                self.error = None;
                self.focus_password = true;
            }
            _ => {}
        }
        None
    }

    /// A damped horizontal wobble after a wrong password.
    fn shake_offset(&mut self, ui: &Ui) -> f32 {
        let now = ui.input(|input| input.time);
        if std::mem::take(&mut self.shake_requested) {
            self.shake_started = Some(now);
        }
        let Some(started) = self.shake_started else {
            return 0.0;
        };
        let elapsed = now - started;
        if elapsed >= SHAKE_TIME {
            self.shake_started = None;
            return 0.0;
        }
        ui.ctx().request_repaint();
        let damping = 1.0 - elapsed / SHAKE_TIME;
        ((elapsed * 48.0).sin() * 10.0 * damping) as f32
    }
}

enum CardAction {
    None,
    Select,
    Forget,
}

fn vault_card(
    ui: &mut Ui,
    cx: &ScreenCtx,
    path: &Path,
    selected: bool,
    enabled: bool,
) -> CardAction {
    let id = Id::new(("vault-card", path));
    let hovered: bool = ui.data(|data| data.get_temp(id)).unwrap_or(false);
    let hover = ui
        .ctx()
        .animate_bool_with_time(id.with("hover"), hovered, 0.12);
    let select = ui
        .ctx()
        .animate_bool_with_time(id.with("selected"), selected, 0.18);
    let fill = theme::GLASS_FILL
        .lerp_to_gamma(theme::GLASS_FILL_HOVER, hover)
        .lerp_to_gamma(theme::GLASS_FILL_ACTIVE, select);
    let stroke = theme::GLASS_STROKE.lerp_to_gamma(theme::ACCENT, select);

    let name = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let exists = path.exists();
    let mut forget = false;

    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let response = ui
        .scope_builder(UiBuilder::new().id_salt(id).sense(sense), |ui| {
            Frame::new()
                .fill(fill)
                .stroke(Stroke::new(1.0, stroke))
                .corner_radius(CornerRadius::same(theme::RADIUS_CONTROL))
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let row = Vec2::new(ui.available_width(), 36.0);
                    ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
                        if enabled {
                            forget =
                                icon_button(ui, icon::X, &cx.i18n.tr("unlock-forget")).clicked();
                        }
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.label(
                                RichText::new(icon::DATABASE)
                                    .font(theme::icon_font(22.0))
                                    .color(theme::TEXT_MUTED.lerp_to_gamma(theme::ACCENT, select)),
                            );
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.add(
                                    Label::new(
                                        RichText::new(name)
                                            .font(theme::heading(15.0))
                                            .color(theme::TEXT),
                                    )
                                    .truncate(),
                                );
                                let detail = if exists {
                                    RichText::new(
                                        path.parent().unwrap_or(path).display().to_string(),
                                    )
                                    .color(theme::TEXT_FAINT)
                                } else {
                                    RichText::new(cx.i18n.tr("unlock-missing-file"))
                                        .color(theme::DANGER)
                                };
                                ui.add(Label::new(detail.small()).truncate());
                            });
                        });
                    });
                });
        })
        .response;

    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    ui.data_mut(|data| data.insert_temp(id, response.hovered()));
    if forget {
        CardAction::Forget
    } else if response.clicked() {
        CardAction::Select
    } else {
        CardAction::None
    }
}
