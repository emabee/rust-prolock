use egui::{Color32, RichText, Sides, Ui};
use std::borrow::Cow;

use crate::{ctrl::Controller, ui::Action};

pub(super) fn ok(o_text: Option<Cow<'_, str>>) -> RichText {
    RichText::new(format!("✅ {}", o_text.unwrap_or(t!("_ok")))).color(Color32::DARK_GREEN)
}

pub(super) fn cancel() -> RichText {
    RichText::new(format!("❌ {}", t!("_cancel"))).color(Color32::DARK_RED)
}

pub fn buttons(
    ui: &mut Ui,
    controller: &mut Controller,
    o_text: Option<Cow<'_, str>>,
    ok_action: Action,
) {
    Sides::new().show(
        ui,
        |_ui| {},
        |ui| {
            if ui.button(ok(o_text)).clicked() {
                controller.set_action(ok_action);
            }

            if ui.button(cancel()).clicked() {
                controller.set_action(Action::CloseModal);
            }
        },
    );
}
