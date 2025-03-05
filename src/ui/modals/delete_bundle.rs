use crate::{
    PlFile,
    ui::viz::{EditIdx, PlModal},
};
use egui::{Color32, Context, Modal, RichText, Sides};

pub fn delete_bundle(
    name: &str,
    pl_modal: &mut PlModal,
    pl_file: &mut PlFile,
    edit_idx: &mut EditIdx,
    need_refresh: &mut bool,
    ctx: &Context,
) {
    Modal::new("delete_bundle".into()).show(ctx, |ui| {
        ui.set_width(300.);
        ui.heading(t!("Delete entry"));
        ui.add_space(10.);
        ui.label(t!("_really_delete", name = name));
        ui.label(t!("_no_undo"));
        ui.add_space(15.);
        ui.separator();

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new(t!("_ok_with_icon")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    if let Err(e) = pl_file.save_with_deleted_bundle(name.to_string()) {
                        println!("TODO 'Delete entry' failed with {e:?}");
                    }
                    *edit_idx = EditIdx::None;
                    *pl_modal = PlModal::None;
                    *need_refresh = true;
                }
                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    *edit_idx = EditIdx::None;
                    *pl_modal = PlModal::None;
                    //
                }
            },
        );
    });
}
