use crate::ui::{
    IMG_CANCEL, IMG_DELETE, IMG_DELETE_INACTIVE, IMG_EDIT, IMG_EDIT_INACTIVE, IMG_OK,
    controller::{Action, Controller},
};
use egui::{Button, Color32, Image, Ui};

pub fn active_buttons_edit_and_delete(
    ui: &mut Ui,
    index: usize,
    name: &str,
    controller: &mut Controller,
) {
    if ui
        .add(
            Button::image(
                Image::new(IMG_EDIT)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.44),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label(t!("Edit entry"));
        })
        .clicked()
    {
        controller.set_action(Action::StartModifyDocument(index, name.to_string()));
    }

    ui.add_space(-4.);

    if ui
        .add(
            Button::image(
                Image::new(IMG_DELETE)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.22),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label(t!("Delete entry"));
        })
        .clicked()
    {
        controller.set_action(Action::StartDeleteDocument(name.to_string()));
    }
}

pub fn inactive_buttons_edit_and_delete(ui: &mut Ui) {
    ui.add_enabled(
        false,
        Button::image(
            Image::new(IMG_EDIT_INACTIVE)
                .maintain_aspect_ratio(true)
                .fit_to_original_size(0.44),
        )
        .fill(Color32::WHITE),
    );

    ui.add_space(-4.);

    ui.add_enabled(
        false,
        Button::image(
            Image::new(IMG_DELETE_INACTIVE)
                .maintain_aspect_ratio(true)
                .fit_to_original_size(0.26),
        )
        .fill(Color32::WHITE),
    );
}

pub fn active_buttons_save_and_cancel(ui: &mut Ui, controller: &mut Controller) {
    if ui
        .add(
            Button::image(
                Image::new(IMG_OK)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.23),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label(t!("Save changes"));
        })
        .clicked()
    {
        controller.set_action(Action::FinalizeModifyDocument);
    }

    ui.add_space(-4.);

    if ui
        .add(
            Button::image(
                Image::new(IMG_CANCEL)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.23),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label(t!("Discard changes"));
        })
        .clicked()
    {
        controller.set_action(Action::Cancel);
    }
}
