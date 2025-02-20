use egui::{Color32, Hyperlink, Image, RichText, Sides, TextEdit, Ui, Vec2};

use crate::ui::{viz::PlModal, IMG_LOGO};

pub(super) fn show_about(pl_modal: &mut PlModal, ui: &mut Ui) {
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
                    "{}\n\n{}\n\nVersion: {}",
                    "ProLock is a tool for securely storing secrets in a password-protected file.",
                    "ProLock is written in rust.",
                    env!("CARGO_PKG_VERSION")
                ))
                .background_color(Color32::TRANSPARENT),
            );

            ui.add_space(15.);

            ui.horizontal(|ui| {
                ui.label("Repository and README:");
                ui.hyperlink("https://github.com/emabee/rust-prolock");
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2 { x: 3., y: 0. };
                ui.label(RichText::new(
                    "Please provide your suggestions, proposals, wishes, complaints:",
                ));
            });
            ui.add(Hyperlink::from_label_and_url(
                ".../rust-prolock/issues",
                "https://github.com/emabee/rust-prolock/issues",
            ));

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
}
