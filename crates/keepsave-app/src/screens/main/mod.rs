//! The unlocked vault: groups, entries and the detail panel, plus autosave,
//! the copy actions and auto-lock.

mod detail;
mod dialogs;
mod generator;
mod list;
mod sidebar;

use std::time::Duration;

use eframe::egui::{
  self, Align, Align2, CornerRadius, Event as InputEvent, Frame, Id, Key, Layout, Margin, Modifiers, Order, Pos2, Rect, RichText, Sense, Stroke, Ui,
  UiBuilder, Vec2, ViewportCommand,
};
use egui_phosphor::regular as icon;
use keepsave_core::{CoreError, EntryId, EntrySummary, ExposeSecret, GroupId, Vault};
use zeroize::Zeroizing;

use super::{Nav, ScreenCtx, error_message};
use crate::{
  clipboard::SecureClipboard,
  drive, theme,
  widgets::{ActionButton, icon_button, search_field, single_line},
  worker::{Event, OpenVault},
};
use detail::{Detail, Draft, EntryView};
use dialogs::Dialog;

/// Wait this long after the last change before saving.
const AUTOSAVE_DELAY: f64 = 1.0;
const TOAST_TIME: f64 = 2.4;
const MARGIN: f32 = 16.0;
const GAP: f32 = 14.0;
const SIDEBAR_WIDTH: f32 = 236.0;
const DETAIL_WIDTH: f32 = 400.0;
const TOP_BAR_HEIGHT: f32 = 42.0;
/// Below this width the detail panel overlaps the list.
const SIDE_BY_SIDE_WIDTH: f32 = 1000.0;
const SLIDE_TIME: f32 = 0.22;
const SEARCH_ID: &str = "main-search";

/// A field of an entry that can be copied.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Field {
  Username,
  Password,
  Url,
  Custom(usize),
}

/// What the user asked for this frame. Collected while drawing and applied
/// afterwards, so drawing only ever reads the vault.
pub enum Action {
  SelectGroup(GroupId),
  SelectEntry(EntryId),
  MoveSelection(isize),
  CloseDetail,
  NewEntry,
  EditEntry,
  CancelEdit,
  SaveDraft,
  DeleteEntry(EntryId),
  DeleteEntryForever(EntryId),
  RestoreEntry(EntryId),
  RestoreHistory(EntryId, usize),
  Copy(EntryId, Field),
  OpenUrl(EntryId),
  NewGroup,
  RenameGroup(GroupId),
  CommitGroupName(Option<GroupId>, String),
  DeleteGroup(GroupId),
  EmptyBin,
  EmptyBinConfirmed,
  SaveNow,
  Lock,
  CloseWindow,
  ForceClose,
  ConnectDrive,
  CancelDrive,
  DisconnectDrive,
  UploadToDrive,
  UnlinkDrive,
  /// The user confirmed throwing away the draft; then do the action.
  Discard(Box<Action>),
}

impl Action {
  /// Actions that would throw away an entry being edited.
  fn leaves_draft(&self) -> bool {
    matches!(
      self,
      Action::SelectGroup(_)
        | Action::SelectEntry(_)
        | Action::MoveSelection(_)
        | Action::CloseDetail
        | Action::NewEntry
        | Action::CancelEdit
        | Action::CloseWindow
    )
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GroupKind {
  Root,
  Group,
  Bin,
}

pub struct GroupRow {
  pub id: GroupId,
  pub name: String,
  pub count: usize,
  pub kind: GroupKind,
}

pub struct Row {
  pub entry: EntrySummary,
  /// Set when the entry lives in another group than the one shown.
  pub group: Option<String>,
}

/// What the sidebar and the list show, rebuilt only when its key changes.
struct Lists {
  key: (u64, GroupId, String),
  groups: Vec<GroupRow>,
  bin: Option<GroupRow>,
  rows: Vec<Row>,
}

/// Tracks which changes are on disk. Generations count changes; a save
/// writes a snapshot of one generation.
#[derive(Default)]
struct Autosave {
  changed: u64,
  saved: u64,
  in_flight: Option<u64>,
  last_change: f64,
  /// Skip the delay, e.g. for `Ctrl+S` or before locking.
  now: bool,
  error: Option<String>,
  /// The generation whose save failed; it isn't retried on its own.
  failed: Option<u64>,
}

impl Autosave {
  fn mark_changed(&mut self, now: f64) {
    self.changed += 1;
    self.last_change = now;
  }

  fn is_clean(&self) -> bool {
    self.saved >= self.changed
  }

  /// Everything is on disk and nothing is being written.
  fn settled(&self) -> bool {
    self.is_clean() && self.in_flight.is_none()
  }

  fn due(&self, now: f64) -> bool {
    let waited = now - self.last_change >= AUTOSAVE_DELAY;
    !self.is_clean() && self.in_flight.is_none() && (self.now || (waited && self.failed != Some(self.changed)))
  }

  /// Marks the current generation as being written and returns it.
  fn start(&mut self) -> u64 {
    self.in_flight = Some(self.changed);
    self.now = false;
    self.changed
  }

  fn finished(&mut self, generation: u64, result: Result<(), String>) {
    self.in_flight = None;
    match result {
      Ok(()) => {
        self.saved = self.saved.max(generation);
        self.error = None;
        self.failed = None;
      }
      Err(error) => {
        self.error = Some(error);
        self.failed = Some(generation);
      }
    }
  }
}

struct Toast {
  text: String,
  error: bool,
  since: Option<f64>,
}

pub struct MainScreen {
  open: Box<OpenVault>,
  group: GroupId,
  search: String,
  selected: Option<EntryId>,
  detail: Option<Detail>,
  dialog: Option<Dialog>,
  clipboard: SecureClipboard,
  toast: Option<Toast>,
  save: Autosave,
  lists: Option<Lists>,
  focus_search: bool,
  scroll_to_selected: bool,
  last_activity: Option<f64>,
  lock_pending: bool,
  /// Auto-lock waits after a failed save until the next change.
  auto_lock_paused_at: Option<u64>,
  close_pending: bool,
  allow_close: bool,
  /// A copy is being uploaded to Drive.
  uploading: bool,
}

impl MainScreen {
  pub fn new(open: Box<OpenVault>) -> Self {
    Self {
      group: open.vault.root_group(),
      open,
      search: String::new(),
      selected: None,
      detail: None,
      dialog: None,
      clipboard: SecureClipboard::new(),
      toast: None,
      save: Autosave::default(),
      lists: None,
      focus_search: false,
      scroll_to_selected: false,
      last_activity: None,
      lock_pending: false,
      auto_lock_paused_at: None,
      close_pending: false,
      allow_close: false,
      uploading: false,
    }
  }

  fn vault(&self) -> &Vault {
    &self.open.vault
  }

  pub fn on_event(&mut self, event: Event, cx: &ScreenCtx) -> Option<Nav> {
    match event {
      Event::VaultSaved { generation, result } => self.saved(generation, result, cx),
      Event::DriveUploaded { path, result } if path == self.open.path => {
        self.uploading = false;
        match result {
          Ok(file) => {
            self.show_toast(cx.i18n.tr("toast-drive-uploaded"), false);
            return Some(Nav::LinkDrive { path, file });
          }
          Err(error) => self.show_toast(drive::error_message(cx.i18n, &error), true),
        }
      }
      _ => {}
    }
    None
  }

  fn saved(&mut self, generation: u64, result: Result<(), CoreError>, cx: &ScreenCtx) {
    let failed = result.is_err();
    self.save.finished(generation, result.map_err(|error| error_message(cx.i18n, &error)));
    if !failed {
      return;
    }
    if self.lock_pending {
      self.lock_pending = false;
      self.auto_lock_paused_at = Some(self.save.changed);
      self.show_toast(cx.i18n.tr("toast-lock-save-failed"), true);
    }
    if self.close_pending {
      self.close_pending = false;
      let details = self.save.error.clone().unwrap_or_default();
      self.dialog = Some(Dialog::CloseUnsaved { details });
    }
  }

  pub fn ui(&mut self, ui: &mut Ui, cx: &ScreenCtx) -> Option<Nav> {
    let now = ui.input(|input| input.time);
    let mut actions = Vec::new();

    if self.should_auto_lock(ui, cx, now) {
      actions.push(Action::Lock);
    }
    if ui.input(|input| input.viewport().close_requested()) && !self.allow_close {
      ui.ctx().send_viewport_cmd(ViewportCommand::CancelClose);
      actions.push(Action::CloseWindow);
    }
    self.shortcuts(ui, &mut actions);
    self.refresh_lists();

    let area = ui.available_rect_before_wrap().shrink(MARGIN).translate(Vec2::new(0.0, cx.lift));
    let regions = Regions::compute(ui, area, self.detail.is_some());
    sidebar::show(ui, regions.sidebar, self, cx, &mut actions);
    self.top_bar(ui, regions.top_bar, cx, &mut actions);
    list::show(ui, regions.list, self, cx, &mut actions);
    if let Some(rect) = regions.detail {
      detail::show(ui, rect, regions.overlay, regions.open, self, cx, &mut actions);
    }
    dialogs::show(ui, self, cx, &mut actions);
    self.paint_toast(ui, now);

    for action in actions {
      if let Some(nav) = self.apply(action, ui, cx) {
        return Some(nav);
      }
    }
    self.tick(ui, cx, now)
  }

  // ---- per-frame bookkeeping ----

  fn should_auto_lock(&mut self, ui: &Ui, cx: &ScreenCtx, now: f64) -> bool {
    let (active, minimized) = ui.input(|input| (!input.events.is_empty() || input.pointer.is_moving(), input.viewport().minimized == Some(true)));
    let last = *self.last_activity.get_or_insert(now);
    if active {
      self.last_activity = Some(now);
    }

    let limit = f64::from(cx.config.lock_after_minutes) * 60.0;
    let idle = limit > 0.0 && !active && now - last >= limit;
    if limit > 0.0 {
      let remaining = (limit - (now - last)).max(0.5);
      ui.ctx().request_repaint_after(Duration::from_secs_f64(remaining));
    }

    let paused = self.lock_pending || self.auto_lock_paused_at == Some(self.save.changed);
    let wanted = idle || (cx.config.lock_on_minimize && minimized) || (cx.config.lock_on_os_sleep && cx.os_lock);
    wanted && !paused
  }

  fn shortcuts(&mut self, ui: &Ui, actions: &mut Vec<Action>) {
    if ui.input_mut(|input| input.consume_key(Modifiers::COMMAND, Key::L)) {
      actions.push(Action::Lock);
    }
    if self.dialog.is_some() {
      return;
    }

    let editing = matches!(self.detail, Some(Detail::Edit(_)));
    let search_id = Id::new(SEARCH_ID);
    let focused = ui.memory(|memory| memory.focused());
    let search_focused = focused == Some(search_id);
    // Only text fields: a row or button focused with Tab keeps the shortcuts.
    let typing = ui.ctx().text_edit_focused();
    let selected = self.selected.filter(|_| !editing);
    let mut clear_search = false;

    ui.input_mut(|input| {
      if input.consume_key(Modifiers::COMMAND, Key::F) {
        self.focus_search = true;
      }
      if input.consume_key(Modifiers::COMMAND, Key::S) {
        if editing {
          actions.push(Action::SaveDraft);
        }
        actions.push(Action::SaveNow);
      }
      if !editing && input.consume_key(Modifiers::COMMAND, Key::N) {
        actions.push(Action::NewEntry);
      }

      if let Some(id) = selected.filter(|_| !typing) {
        if input.consume_key(Modifiers::COMMAND, Key::B) {
          actions.push(Action::Copy(id, Field::Username));
        }
        if input.consume_key(Modifiers::COMMAND, Key::U) {
          actions.push(Action::OpenUrl(id));
        }
        // egui turns Ctrl+C into a copy event instead of a key press.
        let copy = input.events.iter().position(|event| matches!(event, InputEvent::Copy));
        if let Some(index) = copy {
          input.events.remove(index);
          actions.push(Action::Copy(id, Field::Password));
        }
        if input.consume_key(Modifiers::NONE, Key::Delete) {
          actions.push(Action::DeleteEntry(id));
        }
        if input.consume_key(Modifiers::NONE, Key::Enter) {
          actions.push(Action::EditEntry);
        }
      }
      if !editing && (!typing || search_focused) {
        if input.consume_key(Modifiers::NONE, Key::ArrowDown) {
          actions.push(Action::MoveSelection(1));
        }
        if input.consume_key(Modifiers::NONE, Key::ArrowUp) {
          actions.push(Action::MoveSelection(-1));
        }
      }

      if input.consume_key(Modifiers::NONE, Key::Escape) {
        if editing {
          actions.push(Action::CancelEdit);
        } else if search_focused || !self.search.is_empty() {
          clear_search = true;
        } else if self.detail.is_some() {
          actions.push(Action::CloseDetail);
        }
      }
    });
    // Outside `input_mut`: the context is locked while it runs.
    if clear_search {
      self.search.clear();
      ui.memory_mut(|memory| memory.surrender_focus(search_id));
    }
  }

  fn refresh_lists(&mut self) {
    if self.vault().group(self.group).is_none() {
      self.group = self.vault().root_group();
    }
    let key = (self.save.changed, self.group, self.search.trim().to_owned());
    if self.lists.as_ref().is_some_and(|lists| lists.key == key) {
      return;
    }
    self.lists = Some(build_lists(self.vault(), key));
  }

  fn tick(&mut self, ui: &Ui, cx: &ScreenCtx, now: f64) -> Option<Nav> {
    if self.save.due(now) {
      let generation = self.save.start();
      cx.worker.save_vault(self.vault().clone(), self.open.path.clone(), generation);
    } else if !self.save.is_clean() && self.save.in_flight.is_none() {
      let wait = (AUTOSAVE_DELAY - (now - self.save.last_change)).max(0.05);
      ui.ctx().request_repaint_after(Duration::from_secs_f64(wait));
    }
    if let Some(wait) = self.clipboard.tick() {
      ui.ctx().request_repaint_after(wait);
    }

    if self.close_pending && self.save.settled() {
      self.allow_close = true;
      ui.ctx().send_viewport_cmd(ViewportCommand::Close);
    }
    if self.lock_pending && self.save.settled() {
      return Some(Nav::Lock(self.open.path.clone()));
    }
    None
  }

  // ---- actions ----

  fn draft_changed(&self) -> bool {
    match &self.detail {
      Some(Detail::Edit(draft)) => draft.is_changed(self.vault()),
      _ => false,
    }
  }

  fn apply(&mut self, action: Action, ui: &Ui, cx: &ScreenCtx) -> Option<Nav> {
    if action.leaves_draft() && self.draft_changed() {
      self.dialog = Some(Dialog::Discard(Box::new(action)));
      return None;
    }
    let now = ui.input(|input| input.time);
    match action {
      Action::SelectGroup(id) => {
        self.group = id;
        self.search.clear();
        self.selected = None;
        self.detail = None;
      }
      Action::SelectEntry(id) => self.select(id),
      Action::MoveSelection(step) => self.move_selection(step),
      Action::CloseDetail => {
        self.detail = None;
        self.selected = None;
      }
      Action::NewEntry => {
        let in_bin = self.vault().is_in_recycle_bin(self.group);
        let group = if in_bin || !self.search.trim().is_empty() { self.vault().root_group() } else { self.group };
        self.selected = None;
        self.detail = Some(Detail::Edit(Box::new(Draft::new(group))));
      }
      Action::EditEntry => {
        let id = self.selected?;
        let (data, summary) = (self.vault().entry(id)?, self.vault().entry_summary(id)?);
        self.detail = Some(Detail::Edit(Box::new(Draft::from_entry(id, summary.group, &data))));
      }
      Action::CancelEdit => self.cancel_edit(),
      Action::SaveDraft => self.save_draft(cx, now),
      Action::DeleteEntry(id) => {
        let group = self.vault().entry_summary(id)?.group;
        if self.vault().is_in_recycle_bin(group) {
          let title = self.vault().entry_summary(id)?.title;
          self.dialog = Some(Dialog::DeleteForever { id, title });
        } else {
          let result = self.open.vault.delete_entry(id);
          if self.changed(result, cx, now) {
            self.forget_entry(id);
            self.show_toast(cx.i18n.tr("toast-moved-to-bin"), false);
          }
        }
      }
      Action::DeleteEntryForever(id) => {
        let result = self.open.vault.delete_entry(id);
        if self.changed(result, cx, now) {
          self.forget_entry(id);
        }
      }
      Action::RestoreEntry(id) => {
        let root = self.vault().root_group();
        let result = self.open.vault.move_entry(id, root);
        if self.changed(result, cx, now) {
          self.show_toast(cx.i18n.tr("toast-restored"), false);
        }
      }
      Action::RestoreHistory(id, index) => {
        let result = self.open.vault.restore_history(id, index);
        if self.changed(result, cx, now) {
          self.show_toast(cx.i18n.tr("toast-version-restored"), false);
        }
      }
      Action::Copy(id, field) => self.copy(id, field, cx),
      Action::OpenUrl(id) => {
        let url = self.vault().entry(id)?.url;
        match browsable_url(&url) {
          Some(url) => ui.ctx().open_url(egui::OpenUrl::new_tab(url)),
          None if url.trim().is_empty() => {}
          None => self.show_toast(cx.i18n.tr("toast-url-invalid"), true),
        }
      }
      Action::NewGroup => {
        self.dialog = Some(Dialog::GroupName { target: None, name: String::new(), error: false });
      }
      Action::RenameGroup(id) => {
        let name = self.vault().group(id)?.name;
        self.dialog = Some(Dialog::GroupName { target: Some(id), name, error: false });
      }
      Action::CommitGroupName(target, name) => match target {
        Some(id) => {
          let result = self.open.vault.rename_group(id, &name);
          self.changed(result, cx, now);
        }
        None => {
          let root = self.vault().root_group();
          let result = self.open.vault.add_group(root, &name);
          if let Some(id) = self.changed_with(result, cx, now) {
            self.group = id;
            self.search.clear();
            self.selected = None;
            self.detail = None;
          }
        }
      },
      Action::DeleteGroup(id) => {
        let result = self.open.vault.delete_group(id);
        if self.changed(result, cx, now) {
          if self.group == id {
            self.group = self.vault().root_group();
          }
          self.drop_missing_selection();
          self.show_toast(cx.i18n.tr("toast-moved-to-bin"), false);
        }
      }
      Action::EmptyBin => self.dialog = Some(Dialog::EmptyBin),
      Action::EmptyBinConfirmed => {
        self.open.vault.empty_recycle_bin();
        self.changed(Ok(()), cx, now);
        self.drop_missing_selection();
      }
      Action::SaveNow => {
        if !self.save.is_clean() {
          self.save.now = true;
        }
      }
      Action::Lock => return self.request_lock(cx),
      Action::CloseWindow => {
        self.detail = None;
        if self.save.settled() {
          self.allow_close = true;
          ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        } else {
          self.close_pending = true;
          self.save.now = true;
        }
      }
      Action::ConnectDrive => return Some(Nav::ConnectDrive),
      Action::CancelDrive => return Some(Nav::CancelDrive),
      Action::DisconnectDrive => return Some(Nav::DisconnectDrive),
      Action::UnlinkDrive => return Some(Nav::UnlinkDrive(self.open.path.clone())),
      Action::UploadToDrive => {
        let client = cx.drive.client()?.clone();
        self.uploading = true;
        cx.worker.upload_to_drive(client, self.vault().clone(), self.open.path.clone());
      }
      Action::ForceClose => {
        self.allow_close = true;
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
      }
      Action::Discard(next) => {
        let draft_id = match self.detail.take() {
          Some(Detail::Edit(draft)) => draft.id,
          _ => None,
        };
        match *next {
          Action::CancelEdit => {
            if let Some(id) = draft_id {
              self.select(id);
            }
          }
          next => return self.apply(next, ui, cx),
        }
      }
    }
    None
  }

  /// Records a change to the vault, or shows the error. True on success.
  fn changed(&mut self, result: Result<(), CoreError>, cx: &ScreenCtx, now: f64) -> bool {
    self.changed_with(result, cx, now).is_some()
  }

  fn changed_with<T>(&mut self, result: Result<T, CoreError>, cx: &ScreenCtx, now: f64) -> Option<T> {
    match result {
      Ok(value) => {
        self.save.mark_changed(now);
        Some(value)
      }
      Err(error) => {
        self.show_toast(error_message(cx.i18n, &error), true);
        None
      }
    }
  }

  fn select(&mut self, id: EntryId) {
    self.selected = Some(id);
    let already = matches!(&self.detail, Some(Detail::View(view)) if view.id == id);
    if !already {
      self.detail = Some(Detail::View(EntryView::new(id)));
    }
  }

  fn move_selection(&mut self, step: isize) {
    let Some(lists) = &self.lists else {
      return;
    };
    if lists.rows.is_empty() {
      return;
    }
    let current = self.selected.and_then(|id| lists.rows.iter().position(|row| row.entry.id == id));
    let last = lists.rows.len() as isize - 1;
    let next = match current {
      Some(index) => (index as isize + step).clamp(0, last),
      None if step > 0 => 0,
      None => last,
    };
    let id = lists.rows[next as usize].entry.id;
    self.select(id);
    self.scroll_to_selected = true;
  }

  fn forget_entry(&mut self, id: EntryId) {
    if self.selected == Some(id) {
      self.selected = None;
      self.detail = None;
    }
  }

  fn drop_missing_selection(&mut self) {
    if let Some(id) = self.selected
      && self.vault().entry_summary(id).is_none()
    {
      self.forget_entry(id);
    }
  }

  fn cancel_edit(&mut self) {
    let Some(Detail::Edit(draft)) = &self.detail else {
      return;
    };
    match draft.id {
      Some(id) => self.detail = Some(Detail::View(EntryView::new(id))),
      None => {
        self.detail = None;
        self.selected = None;
      }
    }
  }

  fn save_draft(&mut self, cx: &ScreenCtx, now: f64) {
    let Some(Detail::Edit(draft)) = &mut self.detail else {
      return;
    };
    let data = match draft.to_data() {
      Ok(data) => data,
      Err(problem) => {
        draft.error = Some(problem.message(cx.i18n));
        return;
      }
    };
    let (id, group, moved) = (draft.id, draft.group, draft.group != draft.original_group);
    let vault = &mut self.open.vault;
    let result = match id {
      Some(id) => vault.update_entry(id, &data).and_then(|()| if moved { vault.move_entry(id, group) } else { Ok(()) }).map(|()| id),
      None => vault.add_entry(group, &data),
    };
    match result {
      Ok(id) => {
        self.save.mark_changed(now);
        self.detail = None;
        self.select(id);
        self.scroll_to_selected = true;
      }
      Err(error) => {
        if let Some(Detail::Edit(draft)) = &mut self.detail {
          draft.error = Some(error_message(cx.i18n, &error));
        }
      }
    }
  }

  fn copy(&mut self, id: EntryId, field: Field, cx: &ScreenCtx) {
    let Some(data) = self.vault().entry(id) else {
      return;
    };
    let tr = |key| cx.i18n.tr(key);
    let (value, label) = match field {
      Field::Username => (Zeroizing::new(data.username), tr("toast-copied-username")),
      Field::Password => (Zeroizing::new(data.password.expose_secret().to_owned()), tr("toast-copied-password")),
      Field::Url => (Zeroizing::new(data.url), tr("toast-copied-url")),
      Field::Custom(index) => {
        let Some(field) = data.custom_fields.get(index) else {
          return;
        };
        (Zeroizing::new(field.value.expose_secret().to_owned()), cx.i18n.tr_arg("toast-copied-field", "field", field.key.clone()))
      }
    };
    if value.is_empty() {
      return;
    }

    let seconds = cx.config.clipboard_clear_seconds;
    let clear_after = (seconds > 0).then(|| Duration::from_secs(seconds.into()));
    if self.clipboard.copy(&value, clear_after) {
      let text = if seconds > 0 { format!("{label} \u{b7} {}", cx.i18n.tr_arg("toast-clears-in", "seconds", seconds)) } else { label };
      self.show_toast(text, false);
    } else {
      self.show_toast(tr("toast-copy-failed"), true);
    }
  }

  /// Locks now, or after the pending changes are on disk. A draft is
  /// discarded: locking must not wait for the user.
  fn request_lock(&mut self, cx: &ScreenCtx) -> Option<Nav> {
    self.detail = None;
    self.dialog = None;
    if self.save.settled() {
      return Some(Nav::Lock(self.open.path.clone()));
    }
    self.lock_pending = true;
    self.save.now = true;
    self.show_toast(cx.i18n.tr("toast-saving-before-lock"), false);
    None
  }

  fn show_toast(&mut self, text: String, error: bool) {
    self.toast = Some(Toast { text, error, since: None });
  }

  // ---- drawing ----

  fn top_bar(&mut self, ui: &mut Ui, rect: Rect, cx: &ScreenCtx, actions: &mut Vec<Action>) {
    let tr = |key| cx.i18n.tr(key);
    let mut bar = ui.new_child(UiBuilder::new().id_salt("main-top-bar").max_rect(rect).layout(Layout::right_to_left(Align::Center)));
    if icon_button(&mut bar, icon::LOCK, &tr("main-lock")).clicked() {
      actions.push(Action::Lock);
    }
    self.save_status(&mut bar, cx, actions);
    let new_entry = ActionButton::new(tr("main-new-entry")).icon(icon::PLUS).primary();
    if bar.add(new_entry).clicked() {
      actions.push(Action::NewEntry);
    }
    bar.add_space(4.0);
    let response = search_field(&mut bar, Id::new(SEARCH_ID), &mut self.search, &tr("main-search"), &tr("main-search-clear"));
    if self.focus_search {
      response.request_focus();
      self.focus_search = false;
    }
  }

  fn save_status(&self, ui: &mut Ui, cx: &ScreenCtx, actions: &mut Vec<Action>) {
    let failed = self.save.error.is_some() && !self.save.is_clean();
    let busy = !self.save.is_clean() && !failed;
    let (glyph, text, color) = if failed {
      (icon::WARNING_CIRCLE, "status-failed", theme::DANGER)
    } else if busy {
      ("", "status-saving", theme::TEXT_MUTED)
    } else {
      (icon::CHECK, "status-saved", theme::TEXT_FAINT)
    };
    let text = single_line(ui, &cx.i18n.tr(text), egui::FontId::proportional(12.5), color, 160.0);
    let size = Vec2::new(text.size().x + 30.0, 30.0);
    let sense = if failed { Sense::click() } else { Sense::hover() };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    let icon_rect = Rect::from_center_size(Pos2::new(rect.left() + 9.0, rect.center().y), Vec2::splat(12.0));
    if busy {
      egui::Spinner::new().size(12.0).color(color).paint_at(ui, icon_rect);
    } else {
      ui.painter().text(icon_rect.center(), Align2::CENTER_CENTER, glyph, theme::icon_font(14.0), color);
    }
    let text_pos = Pos2::new(rect.left() + 22.0, rect.center().y - text.size().y / 2.0);
    ui.painter().galley(text_pos, text, color);

    if failed {
      let details = self.save.error.clone().unwrap_or_default();
      let tooltip = cx.i18n.tr_arg("status-retry", "details", details);
      if response.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(tooltip).clicked() {
        actions.push(Action::SaveNow);
      }
    }
  }

  fn paint_toast(&mut self, ui: &Ui, now: f64) {
    let Some(toast) = &mut self.toast else {
      return;
    };
    let age = now - *toast.since.get_or_insert(now);
    if age > TOAST_TIME {
      self.toast = None;
      return;
    }
    let fade = ((TOAST_TIME - age).min(age) / 0.18).clamp(0.0, 1.0) as f32;
    let (glyph, color) = if toast.error { (icon::WARNING_CIRCLE, theme::DANGER) } else { (icon::CHECK_CIRCLE, theme::SUCCESS) };
    egui::Area::new(Id::new("main-toast"))
      .order(Order::Foreground)
      .anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -28.0 + (1.0 - fade) * 10.0))
      .interactable(false)
      .show(ui.ctx(), |ui| {
        ui.multiply_opacity(fade);
        Frame::new()
          .fill(theme::GLASS_FILL_SOLID)
          .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
          .corner_radius(CornerRadius::same(18))
          .inner_margin(Margin::symmetric(16, 9))
          .show(ui, |ui| {
            ui.horizontal(|ui| {
              ui.label(RichText::new(glyph).font(theme::icon_font(15.0)).color(color));
              ui.label(RichText::new(&toast.text).color(theme::TEXT));
            });
          });
      });
    ui.ctx().request_repaint();
  }
}

/// Where each part of the screen goes this frame.
struct Regions {
  sidebar: Rect,
  top_bar: Rect,
  list: Rect,
  detail: Option<Rect>,
  /// The detail panel floats over the list instead of beside it.
  overlay: bool,
  /// Eased 0→1 progress of the detail panel sliding in.
  open: f32,
}

impl Regions {
  fn compute(ui: &Ui, area: Rect, detail_open: bool) -> Self {
    let t = ui.ctx().animate_bool_with_time(Id::new("main-detail-open"), detail_open, SLIDE_TIME);
    let open = 1.0 - (1.0 - t).powi(3);

    let sidebar = Rect::from_min_size(area.min, Vec2::new(SIDEBAR_WIDTH, area.height()));
    let content = Rect::from_min_max(Pos2::new(sidebar.right() + GAP, area.top()), area.max);
    let overlay = area.width() < SIDE_BY_SIDE_WIDTH;
    let detail_width = DETAIL_WIDTH.min(content.width());
    let list_right = if overlay { content.right() } else { content.right() - (detail_width + GAP) * open };

    let top_bar = Rect::from_min_size(content.min, Vec2::new(content.width(), TOP_BAR_HEIGHT));
    let below = top_bar.bottom() + GAP;
    let list = Rect::from_min_max(Pos2::new(content.left(), below), Pos2::new(list_right, content.bottom()));
    let slide = (1.0 - open) * (detail_width + MARGIN + GAP);
    let detail = detail_open
      .then(|| Rect::from_min_max(Pos2::new(content.right() - detail_width + slide, below), Pos2::new(content.right() + slide, content.bottom())));
    Self { sidebar, top_bar, list, detail, overlay, open }
  }
}

fn build_lists(vault: &Vault, key: (u64, GroupId, String)) -> Lists {
  let (_, group, query) = &key;
  let root = vault.root_group();
  let bin = vault.recycle_bin();
  let count_under = |id| vault.entries_under(id).map_or(0, |entries| entries.len());

  let mut groups = vec![GroupRow {
    id: root,
    name: vault.group(root).map(|g| g.name).unwrap_or_default(),
    count: vault.entries_in(root).map_or(0, |entries| entries.len()),
    kind: GroupKind::Root,
  }];
  for child in vault.child_groups(root).unwrap_or_default() {
    if Some(child.id) != bin {
      groups.push(GroupRow { id: child.id, count: count_under(child.id), name: child.name, kind: GroupKind::Group });
    }
  }
  let bin_row = bin.map(|id| GroupRow { id, name: String::new(), count: count_under(id), kind: GroupKind::Bin });

  let (entries, shown_group) = if !query.is_empty() {
    (vault.search(query), None)
  } else if *group == root {
    (vault.entries_in(root).unwrap_or_default(), Some(*group))
  } else {
    (vault.entries_under(*group).unwrap_or_default(), Some(*group))
  };
  let rows = entries
    .into_iter()
    .map(|entry| Row { group: (Some(entry.group) != shown_group).then(|| vault.group(entry.group).map(|g| g.name)).flatten(), entry })
    .collect();

  Lists { key, groups, bin: bin_row, rows }
}

/// The URL to open for an entry: only web links, adding `https://` when
/// there is no scheme. Anything else could launch arbitrary handlers.
fn browsable_url(url: &str) -> Option<String> {
  let url = url.trim();
  if url.is_empty() {
    return None;
  }
  if let Some((scheme, rest)) = url.split_once(':') {
    // `host:port` is not a scheme.
    let is_scheme =
      !scheme.is_empty() && scheme.chars().all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c)) && !rest.starts_with(|c: char| c.is_ascii_digit());
    if is_scheme {
      let web = ["http", "https"].contains(&scheme.to_ascii_lowercase().as_str());
      return (web && rest.starts_with("//")).then(|| url.to_owned());
    }
  }
  Some(format!("https://{url}"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn only_web_urls_are_opened() {
    assert_eq!(browsable_url(" https://example.com/a "), Some("https://example.com/a".into()));
    assert_eq!(browsable_url("HTTP://example.com"), Some("HTTP://example.com".into()));
    assert_eq!(browsable_url("example.com/login"), Some("https://example.com/login".into()));
    assert_eq!(browsable_url("localhost:8080"), Some("https://localhost:8080".into()));
    assert_eq!(browsable_url("file:///C:/Windows"), None);
    assert_eq!(browsable_url("cmd://calc.exe"), None);
    assert_eq!(browsable_url("javascript:alert(1)"), None);
    assert_eq!(browsable_url("   "), None);
  }

  #[test]
  fn autosave_waits_then_saves_the_latest_generation() {
    let mut save = Autosave::default();
    assert!(save.settled());

    save.mark_changed(10.0);
    assert!(!save.due(10.5));
    assert!(save.due(11.0));
    assert_eq!(save.start(), 1);

    // A change while saving waits for the save in flight.
    save.mark_changed(11.2);
    assert!(!save.due(20.0));
    save.finished(1, Ok(()));
    assert!(!save.is_clean());
    assert!(save.due(20.0));
    assert_eq!(save.start(), 2);
    save.finished(2, Ok(()));
    assert!(save.settled());
  }

  #[test]
  fn a_failed_save_is_retried_only_on_change_or_request() {
    let mut save = Autosave::default();
    save.mark_changed(0.0);
    let generation = save.start();
    save.finished(generation, Err("disk full".into()));
    assert!(!save.settled());
    assert!(!save.due(100.0));

    save.now = true;
    assert!(save.due(100.0));
    save.now = false;

    save.mark_changed(100.0);
    assert!(save.due(101.0));
  }
}
