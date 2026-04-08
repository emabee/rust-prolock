use crate::{
    Controller,
    ui::{Action, modals::buttons, show_error, viz::VEditDocument},
};
use egui::{Color32, Context, FontFamily, FontId, Modal, Rgba, TextEdit};
use egui_extras::{Size, StripBuilder};

pub fn create_document(
    v_edit_document: &mut VEditDocument,
    error: &mut Option<String>,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new("create_document".into()).show(ctx, |ui| {
        ui.vertical(|ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.9))
                .vertical(|mut document_strip| {
                    document_strip.cell(|ui| {
                        let response = ui.add(
                            TextEdit::singleline(&mut v_edit_document.key.0)
                                .hint_text(t!("_unique_document_name"))
                                .desired_width(400.)
                                .clip_text(true)
                                .font(FontId {
                                    size: 16.,
                                    family: FontFamily::Proportional,
                                })
                                .background_color(
                                    egui::lerp(
                                        Rgba::from(Color32::DARK_GRAY)
                                            ..=Rgba::from(ui.visuals().window_fill()),
                                        0.91,
                                    )
                                    .into(),
                                )
                                .interactive(true),
                        );
                        if v_edit_document.request_focus {
                            v_edit_document.request_focus = false;
                            response.request_focus();
                        }

                        ui.add(
                            TextEdit::multiline(&mut v_edit_document.text)
                                .hint_text(t!("Protected text"))
                                .desired_width(600.)
                                .desired_rows(20)
                                .font(FontId::new(12., FontFamily::Monospace))
                                .background_color(Color32::from_black_alpha(0))
                                .interactive(true),
                        );
                    });
                });
        });

        if let Some(e) = error {
            show_error(e, ui);
        }

        buttons::buttons(
            ui,
            controller,
            Some(t!("_save")),
            Action::FinalizeAddDocument,
        );
    });
}
