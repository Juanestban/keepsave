//! Shown after unlocking until the main screen lands in M3.

use eframe::egui::{self, Id, Label, RichText, Ui, Vec2};
use egui_phosphor::regular as icon;
use fluent_bundle::FluentArgs;

use super::{Nav, ScreenCtx, emblem, title};
use crate::{
    theme,
    widgets::{ActionButton, centered_column, glass_card},
    worker::OpenVault,
};

pub struct VaultHome {
    open: Box<OpenVault>,
}

impl VaultHome {
    pub fn new(open: Box<OpenVault>) -> Self {
        Self { open }
    }

    pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
        let tr = |key| cx.i18n.tr(key);
        let mut lock =
            ui.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::L));

        centered_column(
            ui,
            Id::new("vault-home"),
            420.0,
            Vec2::new(0.0, cx.lift),
            |ui| {
                ui.vertical_centered(|ui| emblem(ui, icon::LOCK_OPEN, false));
                title(ui, &tr("vault-unlocked"), None);
                ui.add_space(16.0);

                glass_card(ui, |ui| {
                    let vault = &self.open.vault;
                    ui.label(
                        RichText::new(vault.name())
                            .font(theme::heading(20.0))
                            .color(theme::TEXT),
                    );
                    ui.add(
                        Label::new(
                            RichText::new(self.open.path.display().to_string())
                                .small()
                                .color(theme::TEXT_FAINT),
                        )
                        .truncate(),
                    );
                    let mut args = FluentArgs::new();
                    args.set("count", vault.database().iter_all_entries().count());
                    ui.label(
                        RichText::new(cx.i18n.tr_args("vault-entries", &args))
                            .color(theme::TEXT_MUTED),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(tr("vault-coming-soon"))
                            .small()
                            .color(theme::TEXT_FAINT),
                    );
                    ui.add_space(8.0);

                    let button = ActionButton::new(tr("vault-lock"))
                        .icon(icon::LOCK)
                        .primary()
                        .full_width();
                    lock |= ui.add(button).clicked();
                });
            },
        );

        lock.then(|| Nav::Lock(self.open.path.clone()))
    }
}
