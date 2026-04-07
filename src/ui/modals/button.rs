use std::borrow::Cow;

use egui::{Color32, RichText};

pub(super) fn ok(o_text: Option<Cow<'_, str>>) -> RichText {
    RichText::new(format!("✅ {}", o_text.unwrap_or(t!("_ok")))).color(Color32::DARK_GREEN)
}

pub(super) fn cancel() -> RichText {
    RichText::new(format!("❌ {}", t!("_cancel"))).color(Color32::DARK_RED)
}
