use crate::{
    Controller,
    data::Key,
    ui::{Action, IMG_DELETE, modals::buttons, show_error, sizes::MODAL_WIDTH},
};
use egui::{Color32, Context, FontId, Image, RichText};
use egui_modal_with_titlebar::ModalWithTitlebar;

pub fn delete_bundle(key: &Key, error: Option<&str>, controller: &mut Controller, ctx: &Context) {
    let modal_response =
        ModalWithTitlebar::new("delete_bundle", t!("Delete entry"), true).show(ctx, |ui| {
            ui.set_width(MODAL_WIDTH);

            ui.horizontal(|ui| {
                ui.add_space(20.);
                ui.vertical(|ui| {
                    ui.set_width(120.);
                    ui.set_height(140.);
                    ui.add_space(50.);
                    ui.add(
                        Image::new(IMG_DELETE)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(1.25),
                    );
                });
                ui.vertical(|ui| {
                    // ui.add_space(50.);
                    // ui.label(RichText::new(t!("Delete entry")).size(24.));

                    ui.add_space(35.);
                    ui.label(
                        RichText::new(t!("_really_delete", name = &key.0))
                            .font(FontId::proportional(14.0))
                            .color(Color32::DARK_RED),
                    );

                    ui.label(t!("_no_undo"));
                    ui.add_space(40.);
                    ui.separator();
                });
            });

            if let Some(e) = error {
                show_error(e, ui);
            }

            buttons::cancel_and_action(ui, controller, None, Action::FinalizeDeleteBundle);
        });
    if modal_response.should_close() || modal_response.inner.1 {
        controller.set_action(Action::CloseModal);
    }
}
