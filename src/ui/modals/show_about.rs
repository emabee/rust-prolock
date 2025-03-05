use crate::ui::{IMG_LOGO, viz::PlModal};
use egui::{Color32, Context, Image, Modal, RichText, Sides, TextEdit};

pub fn show_about(pl_modal: &mut PlModal, ctx: &Context) {
    let modal_response = Modal::new("show_about".into()).show(ctx, |ui| {
        ui.set_width(400.0);
        ui.add_space(10.);
        ui.heading("ProLock");
        ui.add_space(5.);
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.set_width(150.);
                ui.set_height(150.);
                ui.add(Image::new(IMG_LOGO));
            });
            ui.vertical(|ui| {
                ui.add(
                    TextEdit::multiline(&mut format!(
                        "{}\n\n{}\n\n{}: {}",
                        t!("_about_1"),
                        t!("_about_2"),
                        t!("Version"),
                        env!("CARGO_PKG_VERSION")
                    ))
                    .background_color(Color32::TRANSPARENT),
                );

                ui.add_space(15.);

                // ui.horizontal(|ui| {
                ui.label(t!("_about_3"));
                ui.hyperlink("https://github.com/emabee/rust-prolock");
                // });

                ui.add_space(15.);
            });
        });
        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new("✅").color(Color32::DARK_GREEN))
                    .clicked()
                {
                    *pl_modal = PlModal::None;
                }
            },
        );
    });
    if modal_response.should_close() && matches!(pl_modal, PlModal::About | PlModal::ShowPrintable)
    {
        *pl_modal = PlModal::None;
    }
}
