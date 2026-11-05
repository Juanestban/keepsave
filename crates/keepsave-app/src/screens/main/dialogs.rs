//! Modal confirmations and the group name prompt.

use eframe::egui::{self, Color32, CornerRadius, Frame, Id, Margin, Modal, RichText, Stroke, Ui};
use egui_phosphor::regular as icon;
use keepsave_core::{EntryId, GroupId};

use super::{Action, MainScreen};
use crate::{
  screens::ScreenCtx,
  theme,
  widgets::{ActionButton, button_pair, notice, text_field},
};

pub enum Dialog {
  /// `target` is `None` for a new group.
  GroupName {
    target: Option<GroupId>,
    name: String,
    error: bool,
  },
  DeleteForever {
    id: EntryId,
    title: String,
  },
  EmptyBin,
  /// Holds the action to run once the draft is discarded.
  Discard(Box<Action>),
  CloseUnsaved {
    details: String,
  },
}

enum Outcome {
  Open,
  Cancel,
  Confirm,
}

pub fn show(ui: &Ui, screen: &mut MainScreen, cx: &ScreenCtx, actions: &mut Vec<Action>) {
  let Some(dialog) = &mut screen.dialog else {
    return;
  };
  let tr = |key| cx.i18n.tr(key);
  let frame = Frame::new()
    .fill(theme::GLASS_FILL_SOLID)
    .stroke(Stroke::new(1.0, theme::GLASS_STROKE))
    .corner_radius(CornerRadius::same(theme::RADIUS_PANEL))
    .inner_margin(Margin::same(24));

  let modal = Modal::new(Id::new("main-dialog")).frame(frame).backdrop_color(Color32::from_black_alpha(150)).show(ui.ctx(), |ui| {
    ui.set_width(380.0);
    match dialog {
      Dialog::GroupName { target, name, error } => {
        let heading = if target.is_some() { "dialog-rename-group-title" } else { "dialog-new-group-title" };
        title(ui, &tr(heading), None);
        let response = text_field(ui, Id::new("dialog-group-name"), name, true);
        if !response.has_focus() && !response.lost_focus() {
          response.request_focus();
        }
        let submitted = response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        if *error {
          notice(ui, icon::WARNING_CIRCLE, &tr("dialog-group-error-empty"), theme::DANGER);
        }
        ui.add_space(8.0);
        let confirm = if target.is_some() { "dialog-rename" } else { "dialog-create" };
        let outcome = buttons(ui, &tr("dialog-cancel"), confirm_button(&tr(confirm)));
        if submitted || matches!(outcome, Outcome::Confirm) {
          if name.trim().is_empty() {
            *error = true;
            return Outcome::Open;
          }
          return Outcome::Confirm;
        }
        outcome
      }
      Dialog::DeleteForever { title: entry, .. } => {
        let body = cx.i18n.tr_arg("dialog-delete-forever-body", "title", entry.clone());
        title(ui, &tr("dialog-delete-forever-title"), Some(&body));
        buttons(ui, &tr("dialog-cancel"), danger_button(&tr("dialog-delete")))
      }
      Dialog::EmptyBin => {
        title(ui, &tr("dialog-empty-bin-title"), Some(&tr("dialog-empty-bin-body")));
        buttons(ui, &tr("dialog-cancel"), danger_button(&tr("dialog-empty")))
      }
      Dialog::Discard(_) => {
        title(ui, &tr("dialog-discard-title"), Some(&tr("dialog-discard-body")));
        buttons(ui, &tr("dialog-keep-editing"), danger_button(&tr("dialog-discard")))
      }
      Dialog::CloseUnsaved { details } => {
        let body = cx.i18n.tr_arg("dialog-close-body", "details", details.clone());
        title(ui, &tr("dialog-close-title"), Some(&body));
        buttons(ui, &tr("dialog-stay"), danger_button(&tr("dialog-close-anyway")))
      }
    }
  });

  let outcome = match modal.inner {
    Outcome::Open if modal.should_close() => Outcome::Cancel,
    outcome => outcome,
  };
  match outcome {
    Outcome::Open => {}
    Outcome::Cancel => screen.dialog = None,
    Outcome::Confirm => {
      let action = match screen.dialog.take() {
        Some(Dialog::GroupName { target, name, .. }) => Action::CommitGroupName(target, name.trim().to_owned()),
        Some(Dialog::DeleteForever { id, .. }) => Action::DeleteEntryForever(id),
        Some(Dialog::EmptyBin) => Action::EmptyBinConfirmed,
        Some(Dialog::Discard(next)) => Action::Discard(next),
        Some(Dialog::CloseUnsaved { .. }) => Action::ForceClose,
        None => return,
      };
      actions.push(action);
    }
  }
}

fn title(ui: &mut Ui, title: &str, body: Option<&str>) {
  ui.label(RichText::new(title).font(theme::heading(19.0)).color(theme::TEXT));
  if let Some(body) = body {
    ui.label(RichText::new(body).color(theme::TEXT_MUTED));
  }
  ui.add_space(8.0);
}

fn confirm_button(label: &str) -> ActionButton<'static> {
  ActionButton::new(label).primary()
}

fn danger_button(label: &str) -> ActionButton<'static> {
  ActionButton::new(label).danger()
}

fn buttons(ui: &mut Ui, cancel: &str, confirm: ActionButton) -> Outcome {
  let (cancel, confirm) = button_pair(ui, ActionButton::new(cancel), confirm);
  if confirm.clicked() {
    Outcome::Confirm
  } else if cancel.clicked() {
    Outcome::Cancel
  } else {
    Outcome::Open
  }
}
