use super::buttons::active_buttons_save_and_cancel;
use crate::{
    data::Document,
    ui::{
        actionable::documents::buttons::{
            active_buttons_edit_and_delete, inactive_buttons_edit_and_delete,
        },
        controller::Controller,
        show_error,
        viz::{DocId, DocumentState, OSelected, VDocument, VEditDocument},
    },
};
use egui::{Color32, FontFamily, FontId, RichText, TextEdit, Ui};

pub fn doc_header(
    doc_state: &mut DocumentState,
    controller: &mut Controller,
    doc_strip: &mut egui_extras::Strip<'_, '_>,
    doc_id: &DocId,
    document: &Document,
    v_document: &mut VDocument,
    show_buttons_active: bool,
) {
    if !v_document.suppressed {
        let mut show = true;

        if let DocumentState::ModifyDocument {
            idx,
            v_edit_document,
            error,
        } = doc_state
        {
            if *idx == doc_id.0 {
                doc_strip.cell(|ui| {
                    edit_doc_header(v_edit_document, error.as_deref(), controller, ui);
                });
                show = false;
            }
        }

        if show {
            let o_selected_doc = match doc_state {
                DocumentState::Default(o_selected) => o_selected,
                DocumentState::ModifyDocument { .. } => &mut None,
            };

            doc_strip.cell(|ui| {
                let show_as_selected = o_selected_doc
                    .as_ref()
                    .is_some_and(|DocId(idx, _name)| *idx == doc_id.idx());
                show_doc_header(
                    doc_id,
                    show_as_selected,
                    show_buttons_active,
                    document,
                    o_selected_doc,
                    controller,
                    ui,
                );
            });
        }
    }
}

fn edit_doc_header(
    v_edit_document: &mut VEditDocument,
    error: Option<&str>,
    controller: &mut Controller,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        // FIXME do we need something like  <<let show_buttons_active = v.modal_state.no_modal_is_open();>>
        active_buttons_save_and_cancel(ui, controller);

        ui.add_space(-4.);

        ui.add(
            TextEdit::multiline(&mut v_edit_document.name)
                .hint_text(t!("_unique_document_name"))
                .font(FontId::new(18., FontFamily::Proportional))
                .desired_width(290.)
                .desired_rows(1)
                .background_color(Color32::WHITE)
                .interactive(true),
        );
    });

    ui.add_space(17.);

    if let Some(e) = error {
        show_error(e, ui);
    }
}

fn show_doc_header(
    doc_id: &DocId,
    show_as_selected: bool,
    show_buttons_active: bool,
    document: &Document,
    selected_doc: &mut OSelected,
    controller: &mut Controller,
    ui: &mut Ui,
) {
    let mut name1 = doc_id.name();
    ui.horizontal(|ui| {
        if show_buttons_active {
            active_buttons_edit_and_delete(ui, doc_id, controller);
        } else {
            inactive_buttons_edit_and_delete(ui);
        }

        ui.add_space(-4.);
        let response = ui.add(
            TextEdit::multiline(&mut name1)
                .hint_text(t!("_unique_document_name"))
                .font(FontId::new(18., FontFamily::Proportional))
                .desired_width(290.)
                .desired_rows(1)
                .background_color(Color32::from_black_alpha(if show_as_selected {
                    0
                } else {
                    25
                }))
                .interactive(true),
        );
        if response.clicked() {
            log::info!("Clicked on document header: {}", doc_id.name());
            *selected_doc = Some(DocId(doc_id.idx(), doc_id.name().to_string()));
        }
    });
    ui.add_space(-8.);

    ui.horizontal(|ui| {
        ui.label(
            RichText::new(t!("_last_update_at"))
                .color(Color32::GRAY)
                .font(FontId::new(8., FontFamily::Proportional)),
        );
        ui.label(
            RichText::new(document.last_changed_at().to_string())
                .color(Color32::GRAY)
                .font(FontId::new(8., FontFamily::Proportional)),
        );
    });

    ui.add_space(4.);
}
