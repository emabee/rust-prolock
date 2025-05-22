use crate::{
    data::{Documents, Transient},
    ui::viz::{DocId, DocumentState},
};
use egui::{Color32, FontFamily, FontId, ScrollArea, TextEdit, Ui, Vec2};

pub fn doc_content(
    documents: &Documents,
    doc_state: &mut DocumentState,
    transient: &Transient,
    ui: &mut Ui,
) {
    match doc_state {
        DocumentState::Default(o_selected) => {
            if let Some(DocId(_index, name)) = o_selected {
                ScrollArea::both().show(ui, |ui| {
                    let text = documents.get(name).unwrap(/*OKish*/).text(transient);
                    ui.add_sized(
                        ui.available_size() - Vec2 { x: 25., y: 5. },
                        TextEdit::multiline(&mut text.to_string())
                            .font(FontId::new(12., FontFamily::Monospace))
                            .background_color(Color32::from_black_alpha(10)),
                    );
                });
            }
        }
        DocumentState::ModifyDocument {
            idx: _,
            v_edit_document,
            error: _,
        } => {
            ScrollArea::both().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size() - Vec2 { x: 25., y: 5. },
                    TextEdit::multiline(&mut v_edit_document.text)
                        .hint_text(t!("Protected text"))
                        .font(FontId::new(12., FontFamily::Monospace))
                        .background_color(Color32::from_black_alpha(1)),
                );
            });
        }
    }
}
