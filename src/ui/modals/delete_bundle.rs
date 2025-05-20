use crate::ui::{
    IMG_DELETE,
    controller::{Action, Controller},
    show_error,
    sizes::MODAL_WIDTH,
};
use egui::{Color32, Context, FontId, Image, Modal, RichText, Sides};

pub fn delete_bundle(
    name: &String,
    error: Option<&str>,
    controller: &mut Controller,
    ctx: &Context,
) {
    let modal_response = Modal::new("delete_bundle".into()).show(ctx, |ui| {
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
                ui.add_space(50.);
                ui.label(RichText::new(t!("Delete entry")).size(24.));

                ui.add_space(15.);
                ui.label(
                    RichText::new(t!("_really_delete", name = &name))
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

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new(t!("_ok_with_icon")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    controller.set_action(Action::FinalizeDeleteBundle);
                }
                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    controller.set_action(Action::Cancel);
                }
            },
        );
    });
    if modal_response.should_close() {
        controller.set_action(Action::Cancel);
    }
}
