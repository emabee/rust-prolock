use crate::{
    data::Settings,
    ui::{
        assets::IMG_CHANGE_FILE,
        viz::{FileAction, FileSelection, PlModal},
    },
};
use egui::{Color32, Context, Image, Modal, RichText, Sides, TextEdit, TextStyle};

pub fn change_file(
    pl_modal: &mut PlModal,
    file_selection: &mut FileSelection,
    settings: &mut Settings,
    ctx: &Context,
) {
    Modal::new("change_file".into()).show(ctx, |ui| {
        ui.set_width(500.0);

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

                if let Some(e) = &file_selection.err {
                    ui.label(RichText::new(e).color(Color32::RED));
                    ui.add_space(15.);
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
                    file_selection.o_action = if file_selection.current < settings.files.len() {
                        Some(FileAction::SwitchToKnown(file_selection.current))
                    } else {
                        Some(FileAction::SwitchToNew(file_selection.new.clone()))
                    };
                    *pl_modal = PlModal::None;
                }
                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    *pl_modal = PlModal::None;
                }
            },
        );
    });
}
