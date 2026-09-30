use crate::{
    Controller,
    ui::{Action, modals::buttons, viz::VGeneratePassword},
};
use egui::{Context, FontFamily, FontId, TextEdit};
use egui_modal_with_titlebar::ModalWithTitlebar;

pub fn configure_password_generation(
    generate_pw: &mut VGeneratePassword,
    controller: &mut Controller,
    ctx: &Context,
) {
    ModalWithTitlebar::new("generate_password", t!("Generate password"), true).show(ctx, |ui| {
        ui.add_space(20.);

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
        ui.add_space(20.);
        ui.separator();

        buttons::cancel_and_action(ui, controller, None, Action::FinalizeGeneratePassword);
    });
}
