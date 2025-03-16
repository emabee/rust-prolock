use crate::controller::Controller;
use egui::{Color32, Context, Modal, RichText, Sides};

pub fn delete_bundle(name: &str, controller: &mut Controller, ctx: &Context) {
    Modal::new("delete_bundle".into()).show(ctx, |ui| {
        ui.set_width(300.);
        ui.heading(t!("Delete entry"));
        ui.add_space(10.);
        ui.label(t!("_really_delete", name = name));
        ui.label(t!("_no_undo"));
        ui.add_space(15.);
        ui.separator();

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new(t!("_ok_with_icon")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    controller.finalize_delete(name.to_string());
                }
                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    controller.cancel();
                }
            },
        );
    });
}
