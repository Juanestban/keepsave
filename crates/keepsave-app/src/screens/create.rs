//! New vault form: name, master password with a strength meter, then a save
//! dialog. Argon2 calibration and the first save run on the worker.

use eframe::egui::{Id, RichText, Ui, Vec2};
use egui_phosphor::regular as icon;
use fluent_bundle::FluentArgs;
use keepsave_core::{SecretString, generator::estimate_entropy_bits};
use zeroize::Zeroizing;

use super::{Nav, ScreenCtx, emblem, error_message, title};
use crate::{
    i18n::I18n,
    theme,
    widgets::{
        ActionButton, centered_column, field_label, glass_card, icon_button, notice,
        password_field, strength_meter, text_field,
    },
    worker::Event,
};

#[derive(PartialEq, Eq)]
enum Stage {
    Editing,
    /// The save dialog is open.
    Picking,
    /// Calibrating Argon2 and writing the file.
    Creating,
}

pub struct CreateVault {
    name: String,
    password: Zeroizing<String>,
    confirm: Zeroizing<String>,
    reveal: bool,
    error: Option<String>,
    stage: Stage,
    focus_name: bool,
}

impl CreateVault {
    pub fn new(i18n: &I18n) -> Self {
        Self {
            name: i18n.tr("create-name-default"),
            password: Zeroizing::default(),
            confirm: Zeroizing::default(),
            reveal: false,
            error: None,
            stage: Stage::Editing,
            focus_name: true,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
        let tr = |key| cx.i18n.tr(key);
        let editing = self.stage == Stage::Editing;
        let mut nav = None;
        let mut submit = false;

        centered_column(
            ui,
            Id::new("create"),
            460.0,
            Vec2::new(0.0, cx.lift),
            |ui| {
                ui.horizontal(|ui| {
                    if editing && icon_button(ui, icon::ARROW_LEFT, &tr("common-back")).clicked() {
                        nav = Some(Nav::Welcome);
                    }
                });
                ui.vertical_centered(|ui| emblem(ui, icon::SHIELD_CHECK, !editing));
                title(ui, &tr("create-title"), Some(&tr("create-subtitle")));
                ui.add_space(16.0);

                glass_card(ui, |ui| {
                    field_label(ui, &tr("create-name"));
                    let name = text_field(ui, Id::new("create-name"), &mut self.name, editing);
                    if std::mem::take(&mut self.focus_name) {
                        name.request_focus();
                    }
                    ui.add_space(4.0);

                    let tooltips = (tr("reveal-show"), tr("reveal-hide"));
                    let tooltips = (tooltips.0.as_str(), tooltips.1.as_str());
                    field_label(ui, &tr("create-password"));
                    let password = password_field(
                        ui,
                        Id::new("create-password"),
                        &mut self.password,
                        &mut self.reveal,
                        editing,
                        tooltips,
                    );
                    let bits = estimate_entropy_bits(&self.password);
                    let mut args = FluentArgs::new();
                    args.set("bits", format!("{bits:.0}"));
                    strength_meter(
                        ui,
                        bits,
                        [
                            &tr("strength-weak"),
                            &tr("strength-fair"),
                            &tr("strength-good"),
                            &tr("strength-strong"),
                        ],
                        &cx.i18n.tr_args("strength-bits", &args),
                    );
                    if password.submitted {
                        ui.memory_mut(|memory| memory.request_focus(Id::new("create-confirm")));
                    }

                    field_label(ui, &tr("create-confirm"));
                    let confirm = password_field(
                        ui,
                        Id::new("create-confirm"),
                        &mut self.confirm,
                        &mut self.reveal,
                        editing,
                        tooltips,
                    );
                    submit |= confirm.submitted;
                    ui.add_space(4.0);

                    notice(ui, icon::WARNING, &tr("create-warning"), theme::TEXT_MUTED);
                    if let Some(error) = &self.error {
                        notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
                    }
                    ui.add_space(4.0);

                    let button = ActionButton::new(tr("create-submit"))
                        .icon(icon::FLOPPY_DISK)
                        .primary()
                        .full_width()
                        .loading(!editing);
                    submit |= ui.add(button).clicked();
                });

                if self.stage == Stage::Creating {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(tr("create-busy")).color(theme::TEXT_MUTED));
                    });
                }
            },
        );

        if submit && editing {
            self.submit(cx);
        }
        nav
    }

    fn submit(&mut self, cx: &ScreenCtx) {
        let tr = |key| cx.i18n.tr(key);
        self.error = if self.name.trim().is_empty() {
            Some(tr("create-error-name"))
        } else if self.password.is_empty() {
            Some(tr("create-error-empty"))
        } else if *self.password != *self.confirm {
            Some(tr("create-error-mismatch"))
        } else {
            None
        };
        if self.error.is_none() {
            self.stage = Stage::Picking;
            cx.worker.pick_new_vault_path(
                tr("dialog-create-title"),
                tr("dialog-filter"),
                format!("{}.kdbx", file_stem(&self.name)),
            );
        }
    }

    pub fn on_event(&mut self, event: Event, cx: &ScreenCtx) -> Option<Nav> {
        match event {
            Event::NewVaultPathPicked(Some(path)) if self.stage == Stage::Picking => {
                self.stage = Stage::Creating;
                cx.worker.create_vault(
                    path,
                    self.name.trim().to_owned(),
                    SecretString::from(self.password.as_str()),
                );
                None
            }
            Event::NewVaultPathPicked(None) => {
                self.stage = Stage::Editing;
                None
            }
            Event::VaultCreated(Ok(open)) => Some(Nav::Opened(Box::new(open))),
            Event::VaultCreated(Err(error)) => {
                self.stage = Stage::Editing;
                self.error = Some(error_message(cx.i18n, &error));
                None
            }
            _ => None,
        }
    }
}

/// The vault name with characters Windows rejects in file names replaced.
fn file_stem(name: &str) -> String {
    name.trim()
        .chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::file_stem;

    #[test]
    fn file_stem_replaces_reserved_characters() {
        assert_eq!(file_stem("  Work: 2026/Q4  "), "Work_ 2026_Q4");
        assert_eq!(file_stem("Mi bóveda"), "Mi bóveda");
    }
}
