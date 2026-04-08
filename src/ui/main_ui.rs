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
use egui::Context;

pub(super) fn main_ui(
    bundles: &Bundles,
    documents: &Documents,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    top_panels::panel_with_tabs(v, documents, controller, ctx);
    top_panels::panel_with_create_and_filter(v, controller, ctx);

    if v.main_state.is_bundles() {
        bundles::central_panel(bundles, transient, v, controller, ctx);
    } else {
        documents::central_panel(documents, transient, v, controller, ctx);
    }
}
