//! The password generator shown under the password field while editing.

use eframe::egui::{self, CornerRadius, Frame, Id, Margin, RichText, Slider, Stroke, Ui};
use egui_phosphor::regular as icon;
use keepsave_core::{
  ExposeSecret,
  generator::{self, CharsetOptions, LENGTH_RANGE, PassphraseOptions, WORDS_RANGE},
};
use zeroize::Zeroizing;

use crate::{
  i18n::I18n,
  theme,
  widgets::{ActionButton, field_label, icon_button, notice, strength_meter, text_field},
};

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
  pub fn ui(&mut self, ui: &mut Ui, i18n: &I18n) -> Option<Zeroizing<String>> {
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

        let use_it = ActionButton::new(tr("gen-use")).icon(icon::CHECK).enabled(!self.failed).full_width();
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
