use super::{
    super::viz::{EditIdx, VBundle, VEditBundle},
    IMG_CANCEL, IMG_DELETE, IMG_DELETE_INACTIVE, IMG_EDIT, IMG_EDIT_INACTIVE, IMG_OK,
};
use crate::data::PlFile;
use anyhow::anyhow;
use egui::{Button, Color32, Image, Ui};

pub(super) fn active_buttons_edit_and_delete(
    ui: &mut Ui,
    pl_file: &mut PlFile,
    index: usize,
    v_bundle: &VBundle,
    edit_idx: &mut EditIdx,
    edit_bundle: &mut VEditBundle,
    need_refresh: &mut bool,
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
        *edit_idx = EditIdx::Mod(index);
        *edit_bundle = VEditBundle {
            orig_name: v_bundle.name.clone(),
            name: v_bundle.name.clone(),
            description: v_bundle.description.to_string(),
            v_named_secrets: v_bundle.v_named_secrets.clone(),
            err: None,
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
            ui.label(t!("Delete entry"));
        })
        .clicked()
    {
        if let Err(e) = pl_file.save_with_deleted_bundle(v_bundle.name.clone()) {
            println!("FIXME 'Delete entry' failed with {e:?}");
        }
        *edit_idx = EditIdx::None;
        *need_refresh = true;
    }
}

pub(super) fn active_buttons_save_and_cancel(
    ui: &mut Ui,
    pl_file: &mut PlFile,
    edit_bundle: &mut VEditBundle,
    edit_idx: &mut EditIdx,
    need_refresh: &mut bool,
) {
    if ui
        .add(
            Button::image(
                Image::new(IMG_OK)
                    .maintain_aspect_ratio(true)
                    .fit_to_original_size(0.30),
            )
            .fill(Color32::WHITE),
        )
        .on_hover_ui(|ui| {
            ui.label(t!("Save changes"));
        })
        .clicked()
    {
        let (orig_name, name, bundle) = edit_bundle.as_bundle();
        if let Err(e) = if edit_idx.is_mod() {
            pl_file.save_with_updated_bundle(&orig_name, name, &bundle)
        } else {
            Err(anyhow!("save: only mod is expected"))
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
            ui.label(t!("Discard changes"));
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
