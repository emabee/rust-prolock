use super::v::VEditBundle;
use crate::data::PlFile;
use egui::{include_image, Button, Color32, Image, ImageSource, Ui};

pub(crate) const IMG_EDIT: ImageSource = include_image!("assets/edit.png");
pub(crate) const IMG_SAVE: ImageSource = include_image!("assets/save.png");
pub(crate) const IMG_DELETE: ImageSource = include_image!("assets/delete.png");
pub(crate) const IMG_CANCEL: ImageSource = include_image!("assets/cancel.png");
pub(crate) const IMG_EDIT_INACTIVE: ImageSource = include_image!("assets/edit inactive.png");
pub(crate) const IMG_SAVE_INACTIVE: ImageSource = include_image!("assets/save inactive.png");
pub(crate) const IMG_DELETE_INACTIVE: ImageSource = include_image!("assets/delete inactive.png");
pub(crate) const IMG_CANCEL_INACTIVE: ImageSource = include_image!("assets/cancel inactive.png");

pub(crate) fn show_bundle_buttons(
    v_edit_bundle: &mut VEditBundle,
    pl_file: &mut PlFile,
    index: usize,
    edit_idx: &mut Option<usize>,
    ui: &mut Ui,
) {
    if edit_idx.is_none() {
        active_buttons_edit_and_delete(index, edit_idx, ui);
    } else {
        if Some(index) == *edit_idx {
            active_buttons_save_and_cancel(pl_file, v_edit_bundle, edit_idx, ui);
        } else {
            inactive_buttons_edit_and_delete(ui);
        }
    }
}

pub(crate) fn active_buttons_edit_and_delete(
    index: usize,
    edit_idx: &mut Option<usize>,
    ui: &mut Ui,
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
            ui.label("Edit entry");
        })
        .clicked()
    {
        *edit_idx = Some(index);
    };

    ui.add_space(5.);

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
            ui.label("Delete entry");
        })
        .clicked()
    {
        println!("FIXME delete is not yet implemented");
        *edit_idx = Some(index);
    };
}

pub(crate) fn active_buttons_save_and_cancel(
    pl_file: &mut PlFile,
    v_edit_bundle: &mut VEditBundle,
    edit_idx: &mut Option<usize>,
    ui: &mut Ui,
) {
    if ui
        .add(
            Button::image(
                Image::new(IMG_SAVE)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.30),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label("Save changes");
        })
        .clicked()
    {
        let (orig_name, name, bundle) = v_edit_bundle.as_bundle();
        if let Err(e) = pl_file.save_with_updated_bundle(orig_name, name, bundle) {
            println!("FIXME 'Save changes' failed with {e}");
        }
        *edit_idx = None;
    };

    ui.add_space(5.);

    if ui
        .add(
            Button::image(
                Image::new(IMG_CANCEL)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.30),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label("Discard changes");
        })
        .clicked()
    {
        println!("FIXME 'Discard changes' is not yet implemented");
        *edit_idx = None;
    };
}

pub(crate) fn inactive_buttons_edit_and_delete(ui: &mut Ui) {
    ui.add_enabled(
        false,
        Button::image(
            Image::new(IMG_EDIT_INACTIVE)
                .maintain_aspect_ratio(true)
                .fit_to_original_size(0.44),
        )
        .fill(Color32::WHITE),
    );

    ui.add_space(5.);

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
