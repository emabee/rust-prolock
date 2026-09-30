use crate::{
    Controller,
    data::Settings,
    file_dialog::FileDialog,
    ui::{
        Action, assets::IMG_CHANGE_FILE, modals::buttons, show_error, sizes::MODAL_WIDTH,
        viz::FileSelection,
    },
};
use egui::{Context, FontFamily, FontId, Image, RichText, TextEdit};
use egui_modal_with_titlebar::ModalWithTitlebar;

pub fn change_file(
    file_dialog: &mut FileDialog,
    settings: &mut Settings,
    file_selection: &mut FileSelection,
    controller: &mut Controller,
    ctx: &Context,
) {
    let modal_response =
        ModalWithTitlebar::new("change_file", t!("switch_to_another_prolock_file"), true).show(
            ctx,
            |ui| {
                ui.set_width(MODAL_WIDTH);

                ui.horizontal(|ui| {
                    ui.add_space(20.);
                    ui.vertical(|ui| {
                        ui.set_width(120.);
                        ui.set_height(140.);
                        ui.add_space(20.);
                        ui.add(
                            Image::new(IMG_CHANGE_FILE)
                                .maintain_aspect_ratio(true)
                                .fit_to_original_size(1.25),
                        );
                    });
                    ui.vertical(|ui| {
                        ui.add_space(35.);
                        if let Some(e) = &file_selection.error {
                            show_error(e, ui);
                        }

                        for (i, s) in settings.files.iter().enumerate() {
                            ui.radio_value(
                                &mut file_selection.current,
                                i,
                                RichText::new(s.display().to_string())
                                    .font(FontId::new(12., FontFamily::Monospace)),
                            );
                        }

                        ui.horizontal(|ui| {
                            ui.radio_value(&mut file_selection.current, settings.files.len(), "");
                            if ui
                                .add(
                                    TextEdit::singleline(&mut file_selection.new)
                                        .hint_text(t!("File path"))
                                        .font(FontId::new(12., FontFamily::Monospace)),
                                )
                                .gained_focus()
                            {
                                file_selection.current = settings.files.len();
                            }
                            if ui.button(format!("📂 {}...", t!("_select"))).clicked() {
                                file_selection.current = settings.files.len();
                                file_dialog.pick_file();
                            }

                            file_dialog.update(ui.ctx());

                            if let Some(path) = file_dialog.take_picked() {
                                file_selection.new = path.display().to_string();
                            }
                        });
                        ui.add_space(20.);
                    });
                });

                ui.separator();

                buttons::cancel_and_action(
                    ui,
                    controller,
                    None,
                    if file_selection.current < settings.files.len() {
                        Action::SwitchToKnownFile(file_selection.current)
                    } else {
                        Action::SwitchToNewFile(file_selection.new.clone())
                    },
                );
            },
        );

    if modal_response.should_close() || modal_response.inner.1 {
        controller.set_action(Action::CloseModal);
    }
}
