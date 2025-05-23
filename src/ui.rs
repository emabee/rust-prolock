mod assets;
pub mod colors;
mod controller;
mod main_ui;
mod modals;
mod password;
pub mod pl_app;
pub mod sizes;
mod top_panel;
pub mod viz;

use crate::ui::assets::{
    IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_BURGER, IMG_CANCEL, IMG_DELETE, IMG_DELETE_INACTIVE,
    IMG_EDIT, IMG_EDIT_INACTIVE, IMG_ERASE, IMG_LOGO, IMG_OK, IMG_RUST_LOGO, IMG_SAVE,
};
use egui::{Color32, RichText};

pub const LIGHT_GRAY: Color32 = Color32::from_rgb(230, 230, 230);
pub const VERY_LIGHT_GRAY: Color32 = Color32::from_rgb(235, 235, 235);

pub fn show_error(e: &str, ui: &mut egui::Ui) {
    ui.add_space(20.);
    ui.separator();
    ui.add_space(10.);
    ui.label(RichText::new(e).color(Color32::RED));
    ui.add_space(15.);
}
