use crate::{
    data::Settings,
    ui::{
        assets::IMG_CHANGE_FILE,
        controller::{Action, Controller},
        show_error,
        sizes::MODAL_WIDTH,
        viz::FileSelection,
    },
};
use egui::{Color32, Context, Image, Modal, RichText, Sides, TextEdit, TextStyle};

pub fn change_file(
    settings: &mut Settings,
    file_selection: &mut FileSelection,
    controller: &mut Controller,
    ctx: &Context,
) {
    Modal::new("change_file".into()).show(ctx, |ui| {
        ui.set_width(MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.add_space(20.);
            ui.vertical(|ui| {
                ui.set_width(120.);
                ui.set_height(140.);
                ui.add_space(50.);
                ui.add(
                    Image::new(IMG_CHANGE_FILE)
                        .maintain_aspect_ratio(true)
                        .fit_to_original_size(1.25),
                );
            });
            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(RichText::new(t!("Switch to another Prolock file")).size(24.));

                ui.add_space(15.);

                if let Some(e) = &file_selection.error {
                    show_error(e, ui);
                }

                for (i, s) in settings.files.iter().enumerate() {
                    ui.radio_value(
                        &mut file_selection.current,
                        i,
                        RichText::new(s.display().to_string()).monospace(),
                    );
                }

                ui.horizontal(|ui| {
                    ui.radio_value(&mut file_selection.current, settings.files.len(), "");
                    if ui
                        .add(
                            TextEdit::singleline(&mut file_selection.new)
                                .hint_text(t!("File path"))
                                .font(TextStyle::Monospace),
                        )
                        .has_focus()
                    {
                        file_selection.current = settings.files.len();
                    }
                });
                ui.add_space(20.);
            });
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
                    if file_selection.current < settings.files.len() {
                        controller.set_action(Action::SwitchToKnownFile(file_selection.current));
                    } else {
                        controller.set_action(Action::SwitchToNewFile(file_selection.new.clone()));
                    }
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
