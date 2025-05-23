use crate::{
    data::{Documents, Key, Transient},
    ui::{
        controller::Controller,
        main_ui::documents::{doc_content, doc_header},
        sizes::DOCUMENT_NAME_HEIGHT,
        viz::{MainState, V, VDocument},
    },
};
use egui::{
    CentralPanel, Color32, Context, RichText, ScrollArea, scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};
use std::collections::BTreeMap;

pub fn central_panel(
    documents: &Documents,
    transient: &Transient,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    let MainState::Documents(ref mut doc_state) = v.main_state else {
        unreachable!()
    };
    let v_documents = &mut v.documents;
    let show_buttons_active = v.modal_state.no_modal_is_open();

    CentralPanel::default().show(ctx, |ui| {
        if documents.is_empty() {
            ui.horizontal(|ui| {
                ui.label(RichText::from("⬆ ").color(Color32::DARK_GRAY).size(22.));
                ui.label(
                    RichText::from(t!("Press this button to create a document"))
                        .color(Color32::DARK_GRAY)
                        .size(16.)
                        .italics(),
                );
            });
        } else {
            StripBuilder::new(ui)
                .size(Size::exact(330.))
                .size(Size::remainder())
                .horizontal(|mut doc_strip| {
                    doc_strip.cell(|ui| {
                        ScrollArea::vertical()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                            .show(ui, |ui| {
                                StripBuilder::new(ui)
                                    .sizes(
                                        Size::initial(DOCUMENT_NAME_HEIGHT)
                                            .at_most(DOCUMENT_NAME_HEIGHT + 100.),
                                        usize::max(1, visible_documents(v_documents)),
                                    )
                                    .vertical(|mut doc_strip| {
                                        for (key, document) in documents.iter() {
                                            doc_header(
                                                doc_state,
                                                controller,
                                                &mut doc_strip,
                                                key,
                                                document,
                                                v_documents.get_mut(key).unwrap(),
                                                show_buttons_active,
                                            );
                                        }
                                    });
                            });
                    });

                    doc_strip.cell(|ui| {
                        doc_content(documents, doc_state, transient, ui);
                    });
                });
        }
    });
}

fn visible_documents(v_documents: &BTreeMap<Key, VDocument>) -> usize {
    v_documents
        .iter()
        .filter(|(_key, vdoc)| !vdoc.suppressed)
        .count()
}
