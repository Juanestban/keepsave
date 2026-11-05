//! The panel on the right with an entry's fields and their copy and reveal
//! actions, and the modal form that edits an entry.

use chrono::{Local, NaiveDateTime, TimeZone};
use eframe::egui::{
  self, Align, Color32, ComboBox, CornerRadius, Frame, Id, Label, Layout, Margin, Modal, Rect, RichText, ScrollArea, Sense, Stroke, Ui, Vec2,
};
use egui_phosphor::regular as icon;
use keepsave_core::{
  CustomField, EntryData, EntryId, EntrySummary, ExposeSecret, GroupId, HistoryItem, SecretString, Vault, generator::estimate_entropy_bits,
};
use zeroize::Zeroizing;

use super::{Action, Field, MainScreen, generator::Generator};
use crate::{
  i18n::I18n,
  screens::ScreenCtx,
  theme,
  widgets::{
    ActionButton, avatar, button_pair, field_label, icon_button, icon_link, label_row, link, multiline_field, notice, password_field, strength_meter,
    text_field,
  },
};

/// Field names KeePass reserves; custom fields can't use them.
const RESERVED_FIELDS: [&str; 6] = ["Title", "UserName", "Password", "URL", "Notes", "otp"];
const MASK: &str = "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}";

pub enum Detail {
  View(EntryView),
  Edit(Box<Draft>),
}

/// A read-only entry, reloaded whenever the vault changes.
pub struct EntryView {
  pub id: EntryId,
  loaded: Option<Box<Loaded>>,
  reveal_password: bool,
  reveal_fields: Vec<bool>,
  show_history: bool,
}

struct Loaded {
  generation: u64,
  summary: EntrySummary,
  data: EntryData,
  history: Vec<HistoryItem>,
}

impl EntryView {
  pub fn new(id: EntryId) -> Self {
    Self { id, loaded: None, reveal_password: false, reveal_fields: Vec::new(), show_history: false }
  }

  fn load(&mut self, vault: &Vault, generation: u64) -> Option<&Loaded> {
    let stale = self.loaded.as_ref().is_none_or(|loaded| loaded.generation != generation);
    if stale {
      self.loaded = Some(Box::new(Loaded {
        generation,
        summary: vault.entry_summary(self.id)?,
        data: vault.entry(self.id)?,
        history: vault.entry_history(self.id).unwrap_or_default(),
      }));
    }
    self.loaded.as_deref()
  }
}

pub struct DraftField {
  key: String,
  value: Zeroizing<String>,
  protected: bool,
  reveal: bool,
}

/// An entry being edited. Secrets live in wiped-on-drop buffers.
pub struct Draft {
  /// `None` for a new entry.
  pub id: Option<EntryId>,
  pub group: GroupId,
  pub original_group: GroupId,
  title: String,
  username: String,
  password: Zeroizing<String>,
  url: String,
  notes: String,
  tags: String,
  fields: Vec<DraftField>,
  reveal: bool,
  generator: Option<Generator>,
  pub error: Option<String>,
  focus_title: bool,
}

/// What is wrong with a custom field name, which keeps a draft from saving.
#[derive(Debug)]
pub enum FieldName {
  Empty,
  Duplicate(String),
  Reserved(String),
}

impl FieldName {
  pub fn message(&self, i18n: &I18n) -> String {
    match self {
      FieldName::Empty => i18n.tr("edit-error-field-empty"),
      FieldName::Duplicate(name) => i18n.tr_arg("edit-error-field-duplicate", "name", name.clone()),
      FieldName::Reserved(name) => i18n.tr_arg("edit-error-field-reserved", "name", name.clone()),
    }
  }
}

impl Draft {
  pub fn new(group: GroupId) -> Self {
    Self {
      id: None,
      group,
      original_group: group,
      title: String::new(),
      username: String::new(),
      password: Zeroizing::default(),
      url: String::new(),
      notes: String::new(),
      tags: String::new(),
      fields: Vec::new(),
      reveal: false,
      generator: None,
      error: None,
      focus_title: true,
    }
  }

  pub fn from_entry(id: EntryId, group: GroupId, data: &EntryData) -> Self {
    Self {
      id: Some(id),
      title: data.title.clone(),
      username: data.username.clone(),
      password: Zeroizing::new(data.password.expose_secret().to_owned()),
      url: data.url.clone(),
      notes: data.notes.clone(),
      tags: data.tags.join(", "),
      fields: data
        .custom_fields
        .iter()
        .map(|field| DraftField {
          key: field.key.clone(),
          value: Zeroizing::new(field.value.expose_secret().to_owned()),
          protected: field.protected,
          reveal: false,
        })
        .collect(),
      focus_title: false,
      ..Self::new(group)
    }
  }

  fn tag_list(&self) -> Vec<String> {
    self.tags.split([',', ';']).map(str::trim).filter(|tag| !tag.is_empty()).map(str::to_owned).collect()
  }

  fn data_unchecked(&self) -> EntryData {
    EntryData {
      title: self.title.trim().to_owned(),
      username: self.username.clone(),
      password: SecretString::from(self.password.as_str()),
      url: self.url.trim().to_owned(),
      notes: self.notes.clone(),
      tags: self.tag_list(),
      custom_fields: self
        .fields
        .iter()
        .map(|field| CustomField { key: field.key.trim().to_owned(), value: SecretString::from(field.value.as_str()), protected: field.protected })
        .collect(),
    }
  }

  pub fn to_data(&self) -> Result<EntryData, FieldName> {
    let data = self.data_unchecked();
    for (index, field) in data.custom_fields.iter().enumerate() {
      if field.key.is_empty() {
        return Err(FieldName::Empty);
      }
      if RESERVED_FIELDS.contains(&field.key.as_str()) {
        return Err(FieldName::Reserved(field.key.clone()));
      }
      if data.custom_fields[..index].iter().any(|other| other.key == field.key) {
        return Err(FieldName::Duplicate(field.key.clone()));
      }
    }
    Ok(data)
  }

  pub fn is_changed(&self, vault: &Vault) -> bool {
    let data = self.data_unchecked();
    match self.id.and_then(|id| vault.entry(id)) {
      Some(stored) => self.group != self.original_group || !same_data(&data, &stored),
      None => !same_data(&data, &EntryData::default()),
    }
  }
}

fn same_data(a: &EntryData, b: &EntryData) -> bool {
  a.title == b.title
    && a.username == b.username
    && a.password.expose_secret() == b.password.expose_secret()
    && a.url == b.url
    && a.notes == b.notes
    && a.tags == b.tags
    && a.custom_fields.len() == b.custom_fields.len()
    && a
      .custom_fields
      .iter()
      .zip(&b.custom_fields)
      .all(|(x, y)| x.key == y.key && x.protected == y.protected && x.value.expose_secret() == y.value.expose_secret())
}

pub fn show(ui: &mut Ui, rect: Rect, overlay: bool, open: f32, screen: &mut MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let fill = if overlay { theme::GLASS_FILL_SOLID } else { theme::GLASS_FILL };
  // Registered before the panel's widgets, so it sits under them but over
  // the list when the panel floats on top of it.
  ui.interact(rect, Id::new("main-detail-blocker"), Sense::click());

  let generation = screen.save.changed;
  let vault = &screen.open.vault;
  let Some(Detail::View(view)) = &mut screen.detail else {
    return;
  };

  crate::widgets::glass_panel(ui, rect, fill, "main-detail", |ui| {
    ui.multiply_opacity(open);
    view_ui(ui, view, vault, generation, cx, actions);
  });
}

// ---- view ----

fn view_ui(ui: &mut Ui, view: &mut EntryView, vault: &Vault, generation: u64, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let tr = |key| cx.i18n.tr(key);
  let id = view.id;
  let bin = vault.recycle_bin();
  let Some(loaded) = view.load(vault, generation) else {
    actions.push(Action::CloseDetail);
    return;
  };
  let in_bin = vault.is_in_recycle_bin(loaded.summary.group);
  let group_name = if Some(loaded.summary.group) == bin {
    tr("main-recycle-bin")
  } else {
    vault.group(loaded.summary.group).map(|group| group.name).unwrap_or_default()
  };
  let title = if loaded.data.title.is_empty() { tr("detail-untitled") } else { loaded.data.title.clone() };

  header(ui, &title, &group_name, |ui| {
    if icon_button(ui, icon::X, &tr("detail-close")).clicked() {
      actions.push(Action::CloseDetail);
    }
    if in_bin {
      if icon_button(ui, icon::TRASH, &tr("detail-delete-forever")).clicked() {
        actions.push(Action::DeleteEntry(id));
      }
      if icon_button(ui, icon::ARROW_COUNTER_CLOCKWISE, &tr("detail-restore")).clicked() {
        actions.push(Action::RestoreEntry(id));
      }
    } else {
      if icon_button(ui, icon::TRASH, &tr("detail-delete")).clicked() {
        actions.push(Action::DeleteEntry(id));
      }
      if icon_button(ui, icon::PENCIL_SIMPLE, &tr("detail-edit")).clicked() {
        actions.push(Action::EditEntry);
      }
    }
  });
  ui.add_space(12.0);

  // Copy what we draw, so the reveal toggles below can borrow `view`.
  let data = loaded.data.clone();
  let history: Vec<(Option<NaiveDateTime>, String)> = loaded.history.iter().map(|item| (item.modified, item.data.title.clone())).collect();
  let modified = loaded.summary.modified;
  view.reveal_fields.resize(data.custom_fields.len(), false);

  ScrollArea::vertical().id_salt("detail-view").auto_shrink([false, false]).show(ui, |ui| {
    // Keep the buttons clear of the scrollbar.
    ui.set_width(ui.available_width() - 8.0);
    value_row(ui, &tr("field-username"), &data.username, false, |ui| {
      copy_button(ui, cx, actions, id, Field::Username, !data.username.is_empty());
    });

    let password = data.password.expose_secret();
    let shown = if view.reveal_password || password.is_empty() { password } else { MASK };
    value_row(ui, &tr("field-password"), shown, true, |ui| {
      copy_button(ui, cx, actions, id, Field::Password, !password.is_empty());
      reveal_button(ui, cx, &mut view.reveal_password);
    });

    value_row(ui, &tr("field-url"), &data.url, false, |ui| {
      copy_button(ui, cx, actions, id, Field::Url, !data.url.is_empty());
      if !data.url.trim().is_empty() && icon_button(ui, icon::ARROW_SQUARE_OUT, &tr("detail-open-url")).clicked() {
        actions.push(Action::OpenUrl(id));
      }
    });

    if !data.notes.is_empty() {
      field_label(ui, &tr("field-notes"));
      Frame::new().fill(theme::INPUT_FILL).corner_radius(CornerRadius::same(theme::RADIUS_CONTROL)).inner_margin(Margin::symmetric(12, 10)).show(
        ui,
        |ui| {
          ui.set_width(ui.available_width());
          ui.add(Label::new(RichText::new(&data.notes).color(theme::TEXT)).wrap());
        },
      );
      ui.add_space(6.0);
    }

    if !data.tags.is_empty() {
      field_label(ui, &tr("field-tags"));
      ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::splat(6.0);
        for tag in &data.tags {
          tag_chip(ui, tag);
        }
      });
      ui.add_space(6.0);
    }

    for (index, field) in data.custom_fields.iter().enumerate() {
      let value = field.value.expose_secret();
      let reveal = &mut view.reveal_fields[index];
      let shown = if !field.protected || *reveal || value.is_empty() { value } else { MASK };
      value_row(ui, &field.key, shown, field.protected, |ui| {
        copy_button(ui, cx, actions, id, Field::Custom(index), !value.is_empty());
        if field.protected {
          reveal_button(ui, cx, reveal);
        }
      });
    }

    ui.add_space(6.0);
    if let Some(modified) = modified {
      ui.label(RichText::new(cx.i18n.tr_arg("detail-modified", "date", local_time(modified))).small().color(theme::TEXT_FAINT));
    }

    if !history.is_empty() && !in_bin {
      ui.add_space(6.0);
      let caret = if view.show_history { icon::CARET_DOWN } else { icon::CARET_RIGHT };
      let label = cx.i18n.tr_arg("detail-history", "count", history.len());
      if icon_link(ui, caret, &label).clicked() {
        view.show_history = !view.show_history;
      }
      if view.show_history {
        for (index, (modified, title)) in history.iter().enumerate() {
          history_row(ui, cx, actions, id, index, *modified, title);
        }
      }
    }
  });
}

/// Avatar, title and group on the left; `buttons` laid out right to left.
fn header(ui: &mut Ui, title: &str, subtitle: &str, buttons: impl FnOnce(&mut Ui)) {
  let row = Vec2::new(ui.available_width(), 48.0);
  ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
    ui.spacing_mut().item_spacing.x = 2.0;
    buttons(ui);
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
      let (rect, _) = ui.allocate_exact_size(Vec2::splat(44.0), Sense::hover());
      avatar(ui, rect.center(), 22.0, title);
      ui.add_space(4.0);
      ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add(Label::new(RichText::new(title).font(theme::heading(18.0)).color(theme::TEXT)).truncate());
        ui.add(Label::new(RichText::new(subtitle).small().color(theme::TEXT_FAINT)).truncate());
      });
    });
  });
}

/// A labeled read-only value with action buttons on its right.
fn value_row(ui: &mut Ui, label: &str, value: &str, monospace: bool, buttons: impl FnOnce(&mut Ui)) {
  field_label(ui, label);
  Frame::new().fill(theme::INPUT_FILL).corner_radius(CornerRadius::same(theme::RADIUS_CONTROL)).inner_margin(Margin::symmetric(12, 4)).show(
    ui,
    |ui| {
      let row = Vec2::new(ui.available_width(), 32.0);
      ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        buttons(ui);
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          let text = if value.is_empty() {
            RichText::new("\u{2014}").color(theme::TEXT_FAINT)
          } else if monospace {
            RichText::new(value).monospace().color(theme::TEXT)
          } else {
            RichText::new(value).color(theme::TEXT)
          };
          ui.add(Label::new(text).truncate());
        });
      });
    },
  );
  ui.add_space(6.0);
}

fn copy_button(ui: &mut Ui, cx: &ScreenCtx, actions: &mut Vec<Action>, id: EntryId, field: Field, enabled: bool) {
  if enabled && icon_button(ui, icon::COPY, &cx.i18n.tr("detail-copy")).clicked() {
    actions.push(Action::Copy(id, field));
  }
}

fn reveal_button(ui: &mut Ui, cx: &ScreenCtx, reveal: &mut bool) {
  let (glyph, tooltip) = if *reveal { (icon::EYE_SLASH, cx.i18n.tr("reveal-hide")) } else { (icon::EYE, cx.i18n.tr("reveal-show")) };
  if icon_button(ui, glyph, &tooltip).clicked() {
    *reveal = !*reveal;
  }
}

fn tag_chip(ui: &mut Ui, tag: &str) {
  Frame::new()
    .fill(theme::ACCENT.gamma_multiply(0.14))
    .stroke(Stroke::new(1.0, theme::ACCENT.gamma_multiply(0.3)))
    .corner_radius(CornerRadius::same(10))
    .inner_margin(Margin::symmetric(9, 3))
    .show(ui, |ui| {
      ui.label(RichText::new(tag).small().color(theme::ACCENT_HOVER));
    });
}

fn history_row(ui: &mut Ui, cx: &ScreenCtx, actions: &mut Vec<Action>, id: EntryId, index: usize, modified: Option<NaiveDateTime>, title: &str) {
  let row = Vec2::new(ui.available_width(), 30.0);
  ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
    if link(ui, &cx.i18n.tr("detail-history-restore")).clicked() {
      actions.push(Action::RestoreHistory(id, index));
    }
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
      ui.label(RichText::new(icon::CLOCK_COUNTER_CLOCKWISE).font(theme::icon_font(14.0)).color(theme::TEXT_FAINT));
      let date = modified.map(local_time).unwrap_or_default();
      ui.add(Label::new(RichText::new(format!("{date}  {title}")).small().color(theme::TEXT_MUTED)).truncate());
    });
  });
}

/// KDBX times are UTC; show them in the local time zone.
fn local_time(time: NaiveDateTime) -> String {
  Local.from_utc_datetime(&time).format("%Y-%m-%d %H:%M").to_string()
}

// ---- edit ----

const FIELD_GAP: f32 = 14.0;
const COLUMN_GAP: f32 = 20.0;
const EDITOR_WIDTH: f32 = 780.0;

/// The entry form, in a modal over the main screen. Clicking the backdrop or
/// pressing Esc cancels, which asks first if something changed.
pub fn show_editor(ui: &Ui, screen: &mut MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let groups: Vec<(GroupId, String)> =
    screen.lists.as_ref().map(|lists| lists.groups.iter().map(|group| (group.id, group.name.clone())).collect()).unwrap_or_default();
  let vault = &screen.open.vault;
  let Some(Detail::Edit(draft)) = &mut screen.detail else {
    return;
  };

  let screen_rect = ui.ctx().content_rect();
  let width = EDITOR_WIDTH.min(screen_rect.width() - 64.0);
  let body_height = (screen_rect.height() - 230.0).max(160.0);
  let frame = Frame::new()
    .fill(theme::GLASS_FILL_SOLID)
    .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
    .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
    .inner_margin(Margin::same(28));

  let modal = Modal::new(Id::new("entry-editor")).frame(frame).backdrop_color(Color32::from_black_alpha(140)).show(ui.ctx(), |ui| {
    ui.set_width(width);
    editor_header(ui, draft, cx, actions);
    ui.add_space(12.0);
    ScrollArea::vertical().id_salt("entry-editor-body").max_height(body_height).auto_shrink([false, true]).show(ui, |ui| {
      // Keep the right edge clear of the scrollbar.
      ui.set_width(ui.available_width() - 12.0);
      form(ui, draft, vault, &groups, cx);
    });
    ui.add_space(16.0);
    editor_footer(ui, cx, actions);
  });
  if modal.backdrop_response.clicked() {
    actions.push(Action::CancelEdit);
  }
}

fn editor_header(ui: &mut Ui, draft: &Draft, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let tr = |key| cx.i18n.tr(key);
  let heading = if draft.id.is_some() { tr("edit-title") } else { tr("edit-new-title") };
  let row = Vec2::new(ui.available_width(), 40.0);
  ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
    if icon_button(ui, icon::X, &tr("edit-cancel")).clicked() {
      actions.push(Action::CancelEdit);
    }
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
      ui.label(RichText::new(icon::NOTE_PENCIL).font(theme::icon_font(22.0)).color(theme::ACCENT));
      ui.label(RichText::new(heading).font(theme::heading(20.0)).color(theme::TEXT));
    });
  });
}

/// Cancel and Save, aligned to the right.
fn editor_footer(ui: &mut Ui, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let tr = |key| cx.i18n.tr(key);
  let row = Vec2::new(ui.available_width(), theme::CONTROL_HEIGHT);
  ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
    // Line up with the form, which keeps clear of its scrollbar.
    ui.add_space(12.0);
    ui.set_max_width(320.0);
    let (cancel, save) = button_pair(ui, ActionButton::new(tr("edit-cancel")), ActionButton::new(tr("edit-save")).icon(icon::CHECK).primary());
    if cancel.clicked() {
      actions.push(Action::CancelEdit);
    }
    if save.clicked() {
      actions.push(Action::SaveDraft);
    }
  });
}

/// Two fields side by side, each in half the width.
fn pair(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
  ui.scope(|ui| {
    let spacing = ui.spacing().item_spacing.x;
    // `columns` uses the item spacing as the gap between columns; the
    // widgets inside keep the normal spacing.
    ui.spacing_mut().item_spacing.x = COLUMN_GAP;
    ui.columns(2, |columns| {
      for column in columns.iter_mut() {
        column.spacing_mut().item_spacing.x = spacing;
      }
      left(&mut columns[0]);
      right(&mut columns[1]);
    });
  });
  ui.add_space(FIELD_GAP);
}

fn form(ui: &mut Ui, draft: &mut Draft, vault: &Vault, groups: &[(GroupId, String)], cx: &ScreenCtx) {
  let tr = |key| cx.i18n.tr(key);

  pair(
    ui,
    |ui| {
      field_label(ui, &tr("field-title"));
      let title = text_field(ui, Id::new("draft-title"), &mut draft.title, true);
      if std::mem::take(&mut draft.focus_title) {
        title.request_focus();
      }
    },
    |ui| {
      field_label(ui, &tr("field-username"));
      text_field(ui, Id::new("draft-username"), &mut draft.username, true);
    },
  );

  let mut toggle_generator = false;
  pair(
    ui,
    |ui| {
      let open = draft.generator.is_some();
      toggle_generator = label_row(ui, &tr("field-password"), |ui| {
        if open { link(ui, &tr("gen-hide")).clicked() } else { icon_link(ui, icon::DICE_FIVE, &tr("gen-toggle")).clicked() }
      });
      password_field(ui, Id::new("draft-password"), &mut draft.password, &mut draft.reveal, true, (&tr("reveal-show"), &tr("reveal-hide")));
      ui.add_space(4.0);
      let bits = estimate_entropy_bits(&draft.password);
      let labels = [tr("strength-weak"), tr("strength-fair"), tr("strength-good"), tr("strength-strong")];
      strength_meter(ui, bits, labels.each_ref().map(String::as_str), &cx.i18n.tr_arg("strength-bits", "bits", bits.round() as i64));
    },
    |ui| {
      // Matches the height of the password's label row.
      label_row(ui, &tr("field-url"), |_| {});
      text_field(ui, Id::new("draft-url"), &mut draft.url, true);
    },
  );
  if toggle_generator {
    draft.generator = match draft.generator {
      Some(_) => None,
      None => Some(Generator::new()),
    };
  }
  if let Some(generator) = &mut draft.generator {
    if let Some(value) = generator.ui(ui, cx.i18n) {
      draft.password = value;
      draft.generator = None;
    }
    ui.add_space(FIELD_GAP);
  }

  pair(
    ui,
    |ui| {
      field_label(ui, &tr("field-group"));
      group_combo(ui, &mut draft.group, vault, groups, cx);
    },
    |ui| {
      field_label(ui, &tr("field-tags"));
      text_field(ui, Id::new("draft-tags"), &mut draft.tags, true);
      ui.label(RichText::new(tr("field-tags-hint")).small().color(theme::TEXT_FAINT));
    },
  );

  field_label(ui, &tr("field-notes"));
  multiline_field(ui, Id::new("draft-notes"), &mut draft.notes, 4);
  ui.add_space(FIELD_GAP + 4.0);

  label_row(ui, &tr("field-custom"), |ui| {
    if icon_link(ui, icon::PLUS, &tr("field-add")).clicked() {
      draft.fields.push(DraftField { key: String::new(), value: Zeroizing::default(), protected: false, reveal: false });
    }
  });
  ui.add_space(4.0);
  if draft.fields.is_empty() {
    ui.label(RichText::new(tr("field-custom-empty")).small().color(theme::TEXT_FAINT));
  }
  let mut remove = None;
  for (index, field) in draft.fields.iter_mut().enumerate() {
    custom_field_ui(ui, index, field, cx, &mut remove);
  }
  if let Some(index) = remove {
    draft.fields.remove(index);
  }

  if let Some(error) = &draft.error {
    ui.add_space(FIELD_GAP);
    notice(ui, icon::WARNING_CIRCLE, error, theme::DANGER);
  }
}

fn group_combo(ui: &mut Ui, group: &mut GroupId, vault: &Vault, groups: &[(GroupId, String)], cx: &ScreenCtx) {
  let current = groups
    .iter()
    .find(|(id, _)| *id == *group)
    .map(|(_, name)| name.clone())
    .or_else(|| if Some(*group) == vault.recycle_bin() { Some(cx.i18n.tr("main-recycle-bin")) } else { vault.group(*group).map(|found| found.name) })
    .unwrap_or_default();
  ui.spacing_mut().interact_size.y = theme::CONTROL_HEIGHT - 2.0;
  ComboBox::from_id_salt("draft-group").selected_text(current).width(ui.available_width()).height(260.0).show_ui(ui, |ui| {
    for (id, name) in groups {
      ui.selectable_value(group, *id, name);
    }
  });
}

/// One custom field on a row: name, value, protect and remove.
fn custom_field_ui(ui: &mut Ui, index: usize, field: &mut DraftField, cx: &ScreenCtx, remove: &mut Option<usize>) {
  let tr = |key| cx.i18n.tr(key);
  let id = Id::new(("draft-field", index));
  let row = Vec2::new(ui.available_width(), theme::CONTROL_HEIGHT);
  ui.allocate_ui_with_layout(row, Layout::right_to_left(Align::Center), |ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    if icon_button(ui, icon::X, &tr("field-remove")).clicked() {
      *remove = Some(index);
    }
    let (glyph, tooltip) = if field.protected { (icon::LOCK_SIMPLE, tr("field-unprotect")) } else { (icon::LOCK_SIMPLE_OPEN, tr("field-protect")) };
    if icon_button(ui, glyph, &tooltip).clicked() {
      field.protected = !field.protected;
    }
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
      let rest = ui.available_width() - 12.0;
      let name_width = rest * 0.36;
      ui.allocate_ui(Vec2::new(name_width, theme::CONTROL_HEIGHT), |ui| {
        let response = text_field(ui, id.with("key"), &mut field.key, true);
        if field.key.is_empty() && !response.has_focus() {
          ui.painter().text(
            response.rect.left_center(),
            egui::Align2::LEFT_CENTER,
            tr("field-name"),
            egui::FontId::proportional(15.0),
            theme::TEXT_FAINT,
          );
        }
      });
      ui.allocate_ui(Vec2::new(rest - name_width, theme::CONTROL_HEIGHT), |ui| {
        if field.protected {
          password_field(ui, id.with("value"), &mut field.value, &mut field.reveal, true, (&tr("reveal-show"), &tr("reveal-hide")));
        } else {
          text_field(ui, id.with("value"), &mut field.value, true);
        }
      });
    });
  });
  ui.add_space(8.0);
}

#[cfg(test)]
mod tests {
  use keepsave_core::KdfParams;

  use super::*;

  fn vault() -> Vault {
    Vault::create("Test", SecretString::from("pw"), KdfParams::insecure_fast())
  }

  #[test]
  fn a_new_draft_is_unchanged_until_typed_into() {
    let vault = vault();
    let mut draft = Draft::new(vault.root_group());
    assert!(!draft.is_changed(&vault));
    draft.password.push('x');
    assert!(draft.is_changed(&vault));
  }

  #[test]
  fn an_edit_draft_tracks_changes_against_the_vault() {
    let mut vault = vault();
    let root = vault.root_group();
    let data = EntryData { title: "Mail".into(), tags: vec!["a".into(), "b".into()], ..EntryData::default() };
    let id = vault.add_entry(root, &data).unwrap();
    let mut draft = Draft::from_entry(id, root, &vault.entry(id).unwrap());
    assert!(!draft.is_changed(&vault));

    draft.tags = "a;  b".into();
    assert!(!draft.is_changed(&vault), "tag spacing is not a change");
    draft.title.push('!');
    assert!(draft.is_changed(&vault));
  }

  #[test]
  fn custom_field_names_are_validated() {
    let vault = vault();
    let mut draft = Draft::new(vault.root_group());
    let field = |key: &str| DraftField { key: key.into(), value: Zeroizing::default(), protected: false, reveal: false };

    draft.fields = vec![field(" ")];
    assert!(matches!(draft.to_data(), Err(FieldName::Empty)));
    draft.fields = vec![field("Password")];
    assert!(matches!(draft.to_data(), Err(FieldName::Reserved(_))));
    draft.fields = vec![field("PIN"), field(" PIN ")];
    assert!(matches!(draft.to_data(), Err(FieldName::Duplicate(_))));
    draft.fields = vec![field("PIN"), field("Recovery")];
    assert_eq!(draft.to_data().unwrap().custom_fields.len(), 2);
  }
}
