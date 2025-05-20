use crate::{
    data::{Documents, Transient},
    ui::{
        actionable::documents::{doc_content::doc_content, doc_header::doc_header},
        controller::Controller,
        sizes::DOCUMENT_NAME_HEIGHT,
        viz::{DocId, DocumentState, VDocument},
    },
};
use egui::{
    CentralPanel, Color32, Context, RichText, ScrollArea, scroll_area::ScrollBarVisibility,
};
use egui_extras::{Size, StripBuilder};

pub fn central_panel(
    documents: &Documents,
    doc_state: &mut DocumentState,
    show_buttons_active: bool,
    transient: &Transient,
    v_documents: &mut [VDocument],
    controller: &mut Controller,
    ctx: &Context,
) {
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
                .size(Size::exact(300.))
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
                                        for (index, (name, document)) in
                                            documents.iter().enumerate()
                                        {
                                            doc_header(
                                                doc_state,
                                                controller,
                                                &mut doc_strip,
                                                &DocId::new(index, name),
                                                document,
                                                &mut v_documents[index],
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

fn visible_documents(v_documents: &[VDocument]) -> usize {
    v_documents.iter().filter(|vdoc| !vdoc.suppressed).count()
}
