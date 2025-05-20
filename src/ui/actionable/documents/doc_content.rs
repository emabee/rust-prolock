use crate::{
    data::{Documents, Transient},
    ui::viz::DocumentState,
};
use egui::{Color32, FontFamily, FontId, TextEdit, Ui};

pub fn doc_content(
    documents: &Documents,
    doc_state: &mut DocumentState,
    transient: &Transient,
    ui: &mut Ui,
) {
    match doc_state {
        DocumentState::Default(o_selected) => {
            if let Some((_index, name)) = o_selected {
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(
                        &mut documents.get(name).unwrap(/*FIXME*/).text(transient).to_string(),
                    )
                    .hint_text(t!("Protected text"))
                    // .desired_width(300.)
                    // .desired_rows(40)
                    .clip_text(true)
                    .font(FontId::new(12., FontFamily::Monospace))
                    .background_color(Color32::LIGHT_GRAY)
                    .interactive(true),
                );
            }
        }
        DocumentState::ModifyDocument {
            idx: _,
            v_edit_document,
            error: _,
        } => {
            // modifiable
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut v_edit_document.text)
                    .hint_text(t!("Protected text"))
                    // .desired_width(300.)
                    // .desired_rows(40)
                    .clip_text(true)
                    .font(FontId::new(12., FontFamily::Monospace))
                    .background_color(Color32::WHITE)
                    .interactive(true),
            );
        }
    }
}
