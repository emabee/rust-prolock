use crate::ui::{
    controller::{Action, Controller},
    viz::VGeneratePassword,
};
use egui::{Color32, Context, FontFamily, FontId, Modal, RichText, Sides, TextEdit};

pub fn configure_password_generation(
    generate_pw: &mut VGeneratePassword,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new("generate_password".into()).show(ctx, |ui| {
        ui.heading(t!("Generate password"));
        ui.add_space(10.);

        ui.horizontal(|ui| {
            ui.label(t!("Length:"));
            ui.add(
                TextEdit::singleline(&mut generate_pw.length)
                    .desired_width(50.)
                    .font(FontId::new(12., FontFamily::Monospace)),
            );
        });

        ui.checkbox(&mut generate_pw.include_lowercase, t!("Include lowercase"));

        ui.checkbox(&mut generate_pw.include_uppercase, t!("Include uppercase"));

        ui.checkbox(&mut generate_pw.include_numbers, t!("Include digits"));

        ui.horizontal(|ui| {
            ui.checkbox(
                &mut generate_pw.include_special,
                t!("Include special characters"),
            );
            if generate_pw.include_special {
                ui.add(
                    TextEdit::singleline(&mut generate_pw.specials)
                        .desired_width(200.)
                        .font(FontId::new(12., FontFamily::Monospace)),
                );
            }
        });

        ui.separator();

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new(t!("_ok_with_icon")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    controller.set_action(Action::FinalizeGeneratePassword);
                }

                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
}
