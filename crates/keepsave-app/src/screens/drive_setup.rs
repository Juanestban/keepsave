//! The Google Drive setup guide: a stepper over the current screen that
//! walks through `docs/GOOGLE_SETUP.md` and ends by taking the user's OAuth
//! client (DESIGN.md Q16, item 6).

use eframe::egui::{
  self, Align, Align2, Color32, CornerRadius, FontId, Frame, Id, Label, Layout, Margin, Modal, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui,
  Vec2,
};
use egui_phosphor::regular as icon;
use keepsave_sync::Credentials;
use zeroize::Zeroizing;

use crate::{
  i18n::I18n,
  theme,
  widgets::{ActionButton, HOVER_TIME, field_label, icon_button, icon_link, notice, password_field, single_line, text_field},
};

const WIDTH: f32 = 640.0;
const BODY_HEIGHT: f32 = 330.0;
const STEP_TIME: f64 = 0.22;
const COPIED_TIME: f64 = 1.5;
const FULL_GUIDE: &str = "https://github.com/Juanestban/keepsave/blob/main/docs/GOOGLE_SETUP.md";
const SCOPE: &str = "https://www.googleapis.com/auth/drive.file";

/// One page of the guide. Its text lives under `gsetup-<key>-*`.
struct Step {
  key: &'static str,
  glyph: &'static str,
  /// Numbered instructions: `gsetup-<key>-1` to `-<items>`.
  items: usize,
  link: Option<&'static str>,
  /// Values to type in Google Cloud: label key and the exact text.
  chips: &'static [(&'static str, &'static str)],
  tip: bool,
}

const STEPS: [Step; 8] = [
  Step { key: "intro", glyph: icon::GOOGLE_DRIVE_LOGO, items: 3, link: None, chips: &[], tip: true },
  Step {
    key: "project",
    glyph: icon::FOLDER_PLUS,
    items: 3,
    link: Some("https://console.cloud.google.com/projectcreate"),
    chips: &[("gsetup-chip-project", "KeepSave")],
    tip: false,
  },
  Step {
    key: "api",
    glyph: icon::PLUG,
    items: 2,
    link: Some("https://console.cloud.google.com/apis/library/drive.googleapis.com"),
    chips: &[],
    tip: false,
  },
  Step {
    key: "branding",
    glyph: icon::IDENTIFICATION_CARD,
    items: 4,
    link: Some("https://console.cloud.google.com/auth/branding"),
    chips: &[("gsetup-chip-app", "KeepSave")],
    tip: false,
  },
  Step {
    key: "scope",
    glyph: icon::KEY,
    items: 3,
    link: Some("https://console.cloud.google.com/auth/scopes"),
    chips: &[("gsetup-chip-scope", SCOPE)],
    tip: true,
  },
  Step { key: "publish", glyph: icon::ROCKET_LAUNCH, items: 2, link: Some("https://console.cloud.google.com/auth/audience"), chips: &[], tip: true },
  Step {
    key: "client",
    glyph: icon::DESKTOP,
    items: 3,
    link: Some("https://console.cloud.google.com/auth/clients/create"),
    chips: &[("gsetup-chip-client", "KeepSave desktop")],
    tip: true,
  },
  Step { key: "paste", glyph: icon::CLIPBOARD_TEXT, items: 0, link: None, chips: &[], tip: true },
];

/// What the user asked of the app from the guide.
pub enum Request {
  Close,
  /// Store this client and connect with it.
  Save(Credentials),
}

pub struct DriveSetup {
  step: usize,
  /// When the current step appeared, for its entrance.
  step_since: Option<f64>,
  client_id: String,
  client_secret: Zeroizing<String>,
  reveal: bool,
  /// Why saving failed, from the app.
  pub error: Option<String>,
}

impl DriveSetup {
  pub fn new() -> Self {
    Self { step: 0, step_since: None, client_id: String::new(), client_secret: Zeroizing::new(String::new()), reveal: false, error: None }
  }

  /// Straight to the last step, to replace a client.
  pub fn credentials_only() -> Self {
    Self { step: STEPS.len() - 1, ..Self::new() }
  }

  fn go(&mut self, step: usize) {
    if step != self.step {
      self.step = step;
      self.step_since = None;
    }
  }

  pub fn show(&mut self, ctx: &egui::Context, i18n: &I18n) -> Option<Request> {
    let tr = |key: &str| i18n.tr(key);
    let screen = ctx.content_rect();
    let width = WIDTH.min(screen.width() - 48.0);
    let frame = Frame::new()
      .fill(theme::GLASS_FILL_SOLID)
      .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
      .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
      .inner_margin(Margin::same(24));

    let mut request = None;
    let modal = Modal::new(Id::new("drive-setup")).frame(frame).backdrop_color(Color32::from_black_alpha(140)).show(ctx, |ui| {
      ui.set_width(width);
      ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), 36.0), Layout::right_to_left(Align::Center), |ui| {
        if icon_button(ui, icon::X, &tr("gsetup-close")).clicked() {
          request = Some(Request::Close);
        }
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          ui.label(RichText::new(icon::GOOGLE_DRIVE_LOGO).font(theme::icon_font(22.0)).color(theme::ACCENT));
          ui.label(RichText::new(tr("gsetup-title")).font(theme::heading(20.0)).color(theme::TEXT));
        });
      });
      ui.add_space(14.0);
      if let Some(step) = stepper(ui, self.step) {
        self.go(step);
      }
      ui.add_space(18.0);

      let now = ui.input(|input| input.time);
      let since = *self.step_since.get_or_insert(now);
      let t = ((now - since) / STEP_TIME).clamp(0.0, 1.0) as f32;
      if t < 1.0 {
        ui.ctx().request_repaint();
      }
      let eased = 1.0 - (1.0 - t).powi(3);

      let body = Vec2::new(ui.available_width(), BODY_HEIGHT);
      let saved = ui
        .allocate_ui_with_layout(body, Layout::top_down(Align::Min), |ui| {
          ui.set_min_size(body);
          ui.multiply_opacity(eased);
          ui.add_space((1.0 - eased) * 10.0);
          ScrollArea::vertical()
            .id_salt(("gsetup-body", self.step))
            .max_height(BODY_HEIGHT)
            .auto_shrink([false, false])
            .show(ui, |ui| {
              // Keep the content clear of the scrollbar.
              ui.set_width(ui.available_width() - 12.0);
              self.step_ui(ui, i18n)
            })
            .inner
        })
        .inner;
      if let Some(credentials) = saved {
        request = Some(Request::Save(credentials));
      }

      ui.add_space(16.0);
      if let Some(footer) = self.footer(ui, i18n) {
        request = Some(footer);
      }
    });
    if modal.should_close() && request.is_none() {
      request = Some(Request::Close);
    }
    request
  }

  /// The current step's content. Returns the client when the last step is
  /// submitted with Enter.
  fn step_ui(&mut self, ui: &mut Ui, i18n: &I18n) -> Option<Credentials> {
    let step = &STEPS[self.step];
    let tr = |suffix: &str| i18n.tr(&format!("gsetup-{}-{suffix}", step.key));
    ui.spacing_mut().item_spacing.y = 10.0;

    ui.horizontal(|ui| {
      ui.spacing_mut().item_spacing.x = 12.0;
      let (rect, _) = ui.allocate_exact_size(Vec2::splat(44.0), Sense::hover());
      ui.painter().rect_filled(rect, CornerRadius::same(theme::RADIUS_CONTROL), theme::ACCENT.gamma_multiply(0.16));
      ui.painter().text(rect.center(), Align2::CENTER_CENTER, step.glyph, theme::icon_font(24.0), theme::ACCENT);
      ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 2.0;
        let counter = if self.step == 0 {
          i18n.tr("gsetup-time")
        } else {
          let mut args = fluent_bundle::FluentArgs::new();
          args.set("step", self.step);
          args.set("total", STEPS.len() - 1);
          i18n.tr_args("gsetup-step-of", &args)
        };
        ui.label(RichText::new(counter).small().color(theme::TEXT_FAINT));
        ui.label(RichText::new(tr("title")).font(theme::heading(18.0)).color(theme::TEXT));
      });
    });
    ui.add(Label::new(RichText::new(tr("body")).color(theme::TEXT_MUTED)).wrap());

    for item in 1..=step.items {
      numbered(ui, item, &tr(&item.to_string()));
    }
    if !step.chips.is_empty() {
      ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
        for (label, value) in step.chips {
          copy_chip(ui, &i18n.tr(label), value, &i18n.tr("gsetup-copied"));
        }
      });
    }

    let submitted = if step.key == "paste" { self.credentials_ui(ui, i18n) } else { false };
    if step.tip {
      notice(ui, icon::LIGHTBULB, &tr("tip"), theme::WARNING);
    }
    if let Some(link) = step.link {
      ui.add_space(4.0);
      if ui.add(ActionButton::new(i18n.tr("gsetup-open-cloud")).icon(icon::ARROW_SQUARE_OUT)).clicked() {
        let _ = webbrowser::open(link);
      }
    }
    if self.step == 0 && icon_link(ui, icon::BOOK_OPEN, &i18n.tr("gsetup-full-guide")).clicked() {
      let _ = webbrowser::open(FULL_GUIDE);
    }
    submitted.then(|| self.credentials()).flatten()
  }

  /// The two fields of the last step. Returns whether Enter submitted them.
  fn credentials_ui(&mut self, ui: &mut Ui, i18n: &I18n) -> bool {
    let tr = |key| i18n.tr(key);
    field_label(ui, &tr("gsetup-client-id"));
    text_field(ui, Id::new("gsetup-client-id"), &mut self.client_id, true);
    field_label(ui, &tr("gsetup-client-secret"));
    let secret =
      password_field(ui, Id::new("gsetup-client-secret"), &mut self.client_secret, &mut self.reveal, true, (&tr("reveal-show"), &tr("reveal-hide")));

    let id = self.client_id.trim();
    if let Some(error) = &self.error {
      notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
    } else if !id.is_empty() && !id.ends_with(keepsave_sync::GOOGLE_CLIENT_SUFFIX) {
      notice(ui, icon::WARNING_CIRCLE, &tr("gsetup-client-id-invalid"), theme::DANGER);
    }
    secret.submitted
  }

  fn credentials(&self) -> Option<Credentials> {
    Credentials::parse(&self.client_id, &self.client_secret)
  }

  fn footer(&mut self, ui: &mut Ui, i18n: &I18n) -> Option<Request> {
    let tr = |key| i18n.tr(key);
    let last = self.step == STEPS.len() - 1;
    let mut request = None;
    ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), theme::CONTROL_HEIGHT), Layout::right_to_left(Align::Center), |ui| {
      ui.spacing_mut().item_spacing.x = 10.0;
      if last {
        let credentials = self.credentials();
        let save = ActionButton::new(tr("gsetup-save")).icon(icon::PLUGS_CONNECTED).primary().enabled(credentials.is_some());
        if ui.add(save).clicked()
          && let Some(credentials) = credentials
        {
          request = Some(Request::Save(credentials));
        }
      } else {
        let label = if self.step == 0 { tr("gsetup-start") } else { tr("gsetup-next") };
        if ui.add(ActionButton::new(label).icon(icon::ARROW_RIGHT).primary()).clicked() {
          self.go(self.step + 1);
        }
      }
      if self.step > 0 && ui.add(ActionButton::new(tr("gsetup-back")).icon(icon::ARROW_LEFT)).clicked() {
        self.go(self.step - 1);
      }
    });
    request
  }
}

/// The row of dots: an info dot for the intro, then the numbered steps.
/// Returns the step that was clicked.
fn stepper(ui: &mut Ui, current: usize) -> Option<usize> {
  const DOT: f32 = 26.0;
  let width = ui.available_width();
  let (rect, _) = ui.allocate_exact_size(Vec2::new(width, DOT), Sense::hover());
  let gap = (width - DOT) / (STEPS.len() - 1) as f32;
  let center = |index: usize| egui::pos2(rect.left() + DOT / 2.0 + gap * index as f32, rect.center().y);

  for index in 0..STEPS.len() - 1 {
    let color = if index < current { theme::ACCENT } else { theme::GLASS_STROKE };
    let from = center(index) + Vec2::new(DOT / 2.0 + 4.0, 0.0);
    let to = center(index + 1) - Vec2::new(DOT / 2.0 + 4.0, 0.0);
    ui.painter().line_segment([from, to], Stroke::new(2.0, color));
  }

  let mut clicked = None;
  for index in 0..STEPS.len() {
    let dot = egui::Rect::from_center_size(center(index), Vec2::splat(DOT));
    let response = ui.interact(dot, Id::new(("gsetup-dot", index)), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
    let hover = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered(), HOVER_TIME);
    let painter = ui.painter();
    let radius = DOT / 2.0;
    // The intro has no number, so the dots match "Step n of 7".
    let (label, font) = if index == 0 { (icon::INFO.to_owned(), theme::icon_font(15.0)) } else { (index.to_string(), FontId::proportional(13.0)) };
    if index == current {
      painter.circle_filled(dot.center(), radius + 4.0, theme::ACCENT.gamma_multiply(0.18));
      painter.circle_filled(dot.center(), radius, theme::ACCENT);
      painter.text(dot.center(), Align2::CENTER_CENTER, label, font, theme::ON_ACCENT);
    } else if index < current {
      painter.circle(dot.center(), radius, theme::ACCENT.gamma_multiply(0.16 + 0.1 * hover), Stroke::new(1.5, theme::ACCENT));
      painter.text(dot.center(), Align2::CENTER_CENTER, icon::CHECK, theme::icon_font(14.0), theme::ACCENT);
    } else {
      painter.circle(dot.center(), radius, theme::GLASS_FILL_HOVER.gamma_multiply(hover), Stroke::new(1.5, theme::GLASS_STROKE));
      painter.text(dot.center(), Align2::CENTER_CENTER, label, font, theme::TEXT_MUTED);
    }
    if response.clicked() {
      clicked = Some(index);
    }
  }
  clicked
}

/// An instruction with its number in a small circle.
fn numbered(ui: &mut Ui, number: usize, text: &str) {
  ui.horizontal_top(|ui| {
    ui.spacing_mut().item_spacing.x = 10.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(22.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 11.0, theme::GLASS_FILL_ACTIVE);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, number.to_string(), FontId::proportional(12.0), theme::TEXT);
    ui.add(Label::new(RichText::new(text).color(theme::TEXT)).wrap());
  });
}

/// A value to type in Google Cloud: label, the text in monospace, and a
/// copy icon that turns into a check for a moment after copying.
fn copy_chip(ui: &mut Ui, label: &str, value: &str, copied_tooltip: &str) {
  let id = Id::new(("gsetup-chip", value));
  let now = ui.input(|input| input.time);
  let copied_at: Option<f64> = ui.data(|data| data.get_temp(id));
  let copied = copied_at.is_some_and(|at| now - at < COPIED_TIME);
  if copied {
    ui.ctx().request_repaint();
  }

  let label = single_line(ui, &format!("{label}  "), FontId::proportional(12.5), theme::TEXT_FAINT, 200.0);
  let text = single_line(ui, value, FontId::monospace(13.0), theme::TEXT, ui.available_width().min(420.0));
  let size = Vec2::new(label.size().x + text.size().x + 48.0, 34.0);
  let (rect, response) = ui.allocate_exact_size(size, Sense::click());
  let hover = ui.ctx().animate_bool_with_time(id.with("hover"), response.hovered(), HOVER_TIME);
  let stroke = if copied { theme::SUCCESS } else { theme::GLASS_STROKE };
  ui.painter().rect(
    rect,
    CornerRadius::same(theme::RADIUS_CONTROL),
    theme::INPUT_FILL.lerp_to_gamma(theme::GLASS_FILL_HOVER, hover),
    Stroke::new(1.0, stroke),
    StrokeKind::Inside,
  );
  let y = rect.center().y;
  let label_width = label.size().x;
  ui.painter().galley(egui::pos2(rect.left() + 12.0, y - label.size().y / 2.0), label, theme::TEXT_FAINT);
  ui.painter().galley(egui::pos2(rect.left() + 12.0 + label_width, y - text.size().y / 2.0), text, theme::TEXT);
  let (glyph, color) = if copied { (icon::CHECK, theme::SUCCESS) } else { (icon::COPY, theme::TEXT_MUTED) };
  ui.painter().text(egui::pos2(rect.right() - 18.0, y), Align2::CENTER_CENTER, glyph, theme::icon_font(15.0), color);

  let response = response.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(if copied { copied_tooltip } else { value });
  if response.clicked() {
    ui.ctx().copy_text(value.to_owned());
    ui.data_mut(|data| data.insert_temp(id, now));
  }
}
