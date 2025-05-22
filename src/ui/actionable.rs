mod bundles;
mod documents;
mod top_panels;

use crate::{
    data::{Bundles, Documents, Transient},
    ui::{
        controller::Controller,
        viz::{MainState, V},
    },
};
use egui::Context;

pub(super) fn panels_for_actionable_ui(
    bundles: &Bundles,
    documents: &Documents,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    top_panels::panel_with_tabs(v, documents, controller, ctx);
    top_panels::panel_with_create_and_filter(v, controller, ctx);

    match v.main_state {
        MainState::Bundles(_) => {
            bundles::central_panel(bundles, transient, v, controller, ctx);
        }
        MainState::Documents(ref mut doc_state) => {
            let show_buttons_active = v.modal_state.no_modal_is_open();
            documents::central_panel(
                documents,
                doc_state,
                show_buttons_active,
                transient,
                &mut v.documents,
                controller,
                ctx,
            );
        }
    }
}
