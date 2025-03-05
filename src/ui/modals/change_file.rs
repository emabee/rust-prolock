use crate::{
    data::FileList,
    ui::{
        assets::IMG_CHANGE_FILE,
        modals::title_of_modal,
        viz::{FileAction, FileSelection, PlModal},
    },
};
use egui::{Color32, Context, Image, Modal, RichText, Sides, TextEdit, TextStyle};

pub fn change_file(
    pl_modal: &mut PlModal,
    file_selection: &mut FileSelection,
    file_list: &mut FileList,
    ctx: &Context,
) {
    Modal::new("change_file".into()).show(ctx, |ui| {
        title_of_modal(
            Some(
                Image::new(IMG_CHANGE_FILE)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.25),
            ),
            &t!("Switch to another Prolock file"),
            ui,
        );

        if let Some(e) = &file_selection.err {
            ui.label(RichText::new(e).color(Color32::RED));
            ui.add_space(15.);
        }

        for (i, s) in file_list.files.iter().enumerate() {
            ui.radio_value(
                &mut file_selection.current,
                i,
                RichText::new(s.display().to_string()).monospace(),
            );
        }

        ui.horizontal(|ui| {
            ui.radio_value(&mut file_selection.current, file_list.files.len(), "");
            ui.add(TextEdit::singleline(&mut file_selection.new).font(TextStyle::Monospace));
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
                    file_selection.o_action = if file_selection.current < file_list.files.len() {
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
