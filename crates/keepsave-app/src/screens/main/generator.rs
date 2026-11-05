//! The password generator: under the password field while editing, and on
//! its own from the top bar.

use eframe::egui::{self, Align, Color32, CornerRadius, Frame, Id, Layout, Margin, Modal, RichText, Slider, Stroke, Ui, Vec2};
use egui_phosphor::regular as icon;
use keepsave_core::{
  ExposeSecret,
  generator::{self, CharsetOptions, LENGTH_RANGE, PassphraseOptions, WORDS_RANGE},
};
use zeroize::Zeroizing;

use super::{Action, MainScreen, ScreenCtx};
use crate::{
  i18n::I18n,
  theme,
  widgets::{ActionButton, field_label, icon_button, notice, strength_meter, text_field},
};

/// What the generator's button does with the password.
#[derive(Clone, Copy)]
pub enum Pick {
  /// Puts it in the entry being edited.
  Use,
  /// Copies it to the clipboard.
  Copy,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
  Characters,
  Passphrase,
}

pub struct Generator {
  mode: Mode,
  charset: CharsetOptions,
  passphrase: PassphraseOptions,
  preview: Zeroizing<String>,
  failed: bool,
}

impl Generator {
  pub fn new() -> Self {
    let mut generator = Self {
      mode: Mode::Characters,
      charset: CharsetOptions::default(),
      passphrase: PassphraseOptions::default(),
      preview: Zeroizing::default(),
      failed: false,
    };
    generator.regenerate();
    generator
  }

  fn regenerate(&mut self) {
    let result = match self.mode {
      Mode::Characters => generator::generate_password(&self.charset),
      Mode::Passphrase => generator::generate_passphrase(&self.passphrase),
    };
    self.preview.clear();
    match result {
      Ok(secret) => {
        self.preview.push_str(secret.expose_secret());
        self.failed = false;
      }
      Err(_) => self.failed = true,
    }
  }

  fn entropy_bits(&self) -> f64 {
    match self.mode {
      Mode::Characters => self.charset.entropy_bits(),
      Mode::Passphrase => self.passphrase.entropy_bits(),
    }
  }

  /// Draws the generator. Returns the value when the user picks it.
  pub fn ui(&mut self, ui: &mut Ui, i18n: &I18n, pick: Pick) -> Option<Zeroizing<String>> {
    let tr = |key| i18n.tr(key);
    let before = (self.mode, self.charset.clone(), self.passphrase.clone());
    let mut picked = None;

    Frame::new()
      .fill(theme::GLASS_FILL)
      .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
      .corner_radius(CornerRadius::same(14))
      .inner_margin(Margin::same(14))
      .show(ui, |ui| {
        ui.horizontal(|ui| {
          ui.selectable_value(&mut self.mode, Mode::Characters, tr("gen-chars"));
          ui.selectable_value(&mut self.mode, Mode::Passphrase, tr("gen-words"));
        });
        ui.add_space(4.0);
        ui.spacing_mut().slider_width = ui.available_width() - 64.0;

        match self.mode {
          Mode::Characters => {
            field_label(ui, &tr("gen-length"));
            ui.add(Slider::new(&mut self.charset.length, LENGTH_RANGE));
            ui.horizontal_wrapped(|ui| {
              ui.checkbox(&mut self.charset.lowercase, "a\u{2013}z");
              ui.checkbox(&mut self.charset.uppercase, "A\u{2013}Z");
              ui.checkbox(&mut self.charset.digits, "0\u{2013}9");
              ui.checkbox(&mut self.charset.symbols, "!#$%");
            });
            ui.checkbox(&mut self.charset.exclude_ambiguous, tr("gen-ambiguous"));
          }
          Mode::Passphrase => {
            field_label(ui, &tr("gen-word-count"));
            ui.add(Slider::new(&mut self.passphrase.words, WORDS_RANGE));
            field_label(ui, &tr("gen-separator"));
            ui.scope(|ui| {
              ui.set_max_width(120.0);
              text_field(ui, Id::new("gen-separator"), &mut self.passphrase.separator, true);
            });
          }
        }
        ui.add_space(4.0);

        if self.failed {
          notice(ui, icon::WARNING_CIRCLE, &tr("gen-error-sets"), theme::DANGER);
        } else {
          Frame::new().fill(theme::INPUT_FILL).corner_radius(CornerRadius::same(theme::RADIUS_CONTROL)).inner_margin(Margin::symmetric(12, 8)).show(
            ui,
            |ui| {
              ui.set_width(ui.available_width());
              ui.horizontal(|ui| {
                let width = ui.available_width() - 40.0;
                ui.allocate_ui(egui::vec2(width, 0.0), |ui| {
                  ui.add(egui::Label::new(RichText::new(self.preview.as_str()).monospace().color(theme::TEXT)).wrap());
                });
                if icon_button(ui, icon::ARROWS_CLOCKWISE, &tr("gen-regenerate")).clicked() {
                  self.regenerate();
                }
              });
            },
          );
          let bits = self.entropy_bits();
          let labels = [tr("strength-weak"), tr("strength-fair"), tr("strength-good"), tr("strength-strong")];
          strength_meter(ui, bits, labels.each_ref().map(String::as_str), &i18n.tr_arg("strength-bits", "bits", bits.round() as i64));
        }
        ui.add_space(2.0);

        let (glyph, label) = match pick {
          Pick::Use => (icon::CHECK, "gen-use"),
          Pick::Copy => (icon::COPY, "gen-copy"),
        };
        let use_it = ActionButton::new(tr(label)).icon(glyph).enabled(!self.failed).full_width();
        if ui.add(use_it).clicked() {
          picked = Some(self.preview.clone());
        }
      });

    if before != (self.mode, self.charset.clone(), self.passphrase.clone()) {
      self.regenerate();
    }
    picked
  }
}

/// The generator on its own, in a modal. Copying keeps it open, so another
/// password is one click away.
pub fn show_tool(ui: &Ui, screen: &mut MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let Some(generator) = &mut screen.generator else {
    return;
  };
  let tr = |key| cx.i18n.tr(key);
  let frame = Frame::new()
    .fill(theme::GLASS_FILL_SOLID)
    .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
    .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
    .inner_margin(Margin::same(24));
  let modal = Modal::new(Id::new("generator-tool")).frame(frame).backdrop_color(Color32::from_black_alpha(140)).show(ui.ctx(), |ui| {
    ui.set_width(440.0);
    let row = Vec2::new(ui.available_width(), 40.0);
    ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
      if icon_button(ui, icon::X, &tr("settings-close")).clicked() {
        actions.push(Action::CloseGenerator);
      }
      ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.label(RichText::new(icon::DICE_FIVE).font(theme::icon_font(22.0)).color(theme::ACCENT));
        ui.label(RichText::new(tr("generator-title")).font(theme::heading(20.0)).color(theme::TEXT));
      });
    });
    ui.add_space(8.0);
    if let Some(value) = generator.ui(ui, cx.i18n, Pick::Copy) {
      actions.push(Action::CopyGenerated(value));
    }
  });
  if modal.backdrop_response.clicked() {
    actions.push(Action::CloseGenerator);
  }
}
