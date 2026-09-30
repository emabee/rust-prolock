use std::borrow::Cow;

use crate::{
    Controller, PROG_TITLE, PROG_VERSION,
    ui::{Action, IMG_LOGO, IMG_RUST_LOGO, modals::buttons, sizes::MODAL_WIDTH},
};
use egui::{Context, FontFamily, FontId, Image, RichText, Sides, Vec2};
use egui_modal_with_titlebar::ModalWithTitlebar;

pub fn show_about(controller: &mut Controller, ctx: &Context) {
    let modal_response = ModalWithTitlebar::new("show_about", PROG_TITLE, true).show(ctx, |ui| {
        ui.set_width(MODAL_WIDTH);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(220.);
                ui.set_height(280.);
                ui.add_space(35.);
                ui.add(Image::new(IMG_LOGO));
            });

            ui.vertical(|ui| {
                ui.add_space(35.);
                ui.label(format!(
                    "{}\n\n{}: {}",
                    t!("_about_1"),
                    t!("Version"),
                    PROG_VERSION
                ));

                ui.add_space(30.);
                ui.horizontal(|ui| {
                    ui.add(Image::new(IMG_RUST_LOGO).fit_to_exact_size(Vec2::new(16., 16.)));
                    ui.label(
                        RichText::new(t!("_about_2"))
                            .font(FontId::new(11., FontFamily::Proportional)),
                    );
                });

                ui.add_space(10.);

                ui.label(
                    RichText::new(t!("_about_3")).font(FontId::new(11., FontFamily::Proportional)),
                );
                ui.hyperlink("https://github.com/emabee/rust-prolock");
            });
        });

        ui.add_space(15.);
        ui.separator();
        ui.add_space(5.);

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui.button(buttons::ok(Some(Cow::Borrowed("")))).clicked() {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
    if modal_response.should_close() || modal_response.inner.1 {
        controller.set_action(Action::CloseModal);
    }
}
