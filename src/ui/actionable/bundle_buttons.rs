use super::super::viz::{EditIdx, VBundle, VEditBundle};
use crate::data::PlFile;
use anyhow::anyhow;
use egui::{include_image, Button, Color32, Image, ImageSource, Ui};

const IMG_EDIT: ImageSource = include_image!("./../assets/edit.png");
const IMG_SAVE: ImageSource = include_image!("./../assets/save.png");
const IMG_DELETE: ImageSource = include_image!("./../assets/delete.png");
const IMG_CANCEL: ImageSource = include_image!("./../assets/cancel.png");
const IMG_EDIT_INACTIVE: ImageSource = include_image!("./../assets/edit inactive.png");
const IMG_SAVE_INACTIVE: ImageSource = include_image!("./../assets/save inactive.png");
const IMG_DELETE_INACTIVE: ImageSource = include_image!("./../assets/delete inactive.png");
const IMG_CANCEL_INACTIVE: ImageSource = include_image!("./../assets/cancel inactive.png");

pub(super) fn active_buttons_edit_and_delete(
    index: usize,
    v_bundle: &VBundle,
    edit_idx: &mut EditIdx,
    v_edit_bundle: &mut VEditBundle,
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
        *edit_idx = EditIdx::Mod(index);
        *v_edit_bundle = VEditBundle {
            orig_name: v_bundle.name.clone(),
            name: v_bundle.name.clone(),
            description: v_bundle.description.to_string(),
            v_named_secrets: v_bundle.v_named_secrets.clone(),
        };
    }

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
        // *edit_idx = EditIdx::Delete(index);
    }
}

pub(super) fn active_buttons_save_and_cancel(
    pl_file: &mut PlFile,
    v_edit_bundle: &mut VEditBundle,
    edit_idx: &mut EditIdx,
    need_refresh: &mut bool,
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
        if let Err(e) = if edit_idx.is_mod() {
            pl_file.save_with_updated_bundle(&orig_name, name, &bundle)
        } else if edit_idx.is_new() {
            pl_file.save_with_added_bundle(name, bundle)
        } else {
            Err(anyhow!("save: only mod and new are expected"))
        } {
            println!("FIXME 'Save changes' failed with {e:?}");
        }
        *edit_idx = EditIdx::None;
        *need_refresh = true;
    }
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
        *edit_idx = EditIdx::None;
    }
}

pub(super) fn inactive_buttons_edit_and_delete(ui: &mut Ui) {
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
