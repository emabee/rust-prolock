mod bundles;
mod documents;
mod top_panels;

use crate::{
    data::{Bundles, Documents, Transient},
    ui::{controller::Controller, viz::V},
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
