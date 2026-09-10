mod ask_for_password_to_open;
mod bundles;
mod documents;
mod show_log;
mod top_panels;

pub use ask_for_password_to_open::ask_for_password_to_open;
pub use show_log::show_log;

use crate::{
    Controller,
    data::{Bundles, Documents, Transient},
    ui::viz::V,
};
use egui::Ui;

pub(super) fn main_ui(
    bundles: &Bundles,
    documents: &Documents,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ui: &mut Ui,
) {
    top_panels::panel_with_tabs(v, documents, controller, ui);
    top_panels::panel_with_create_and_filter(v, controller, ui);

    if v.main_state.is_bundles() {
        bundles::central_panel(bundles, transient, v, controller, ui);
    } else {
        documents::central_panel(documents, transient, v, controller, ui);
    }
}
