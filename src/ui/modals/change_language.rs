use crate::{
    ui::viz::{Lang, PlModal},
    PlFile, SUPPORTED_LANGUAGES,
};
use egui::{Color32, ComboBox, RichText, Sides, Ui};

pub(super) fn change_language(
    lang: &mut Lang,
    pl_modal: &mut PlModal,
    pl_file: &mut PlFile,
    ui: &mut Ui,
) {
    ui.horizontal(|ui| {
        ui.label(t!("Current language:", locale = lang.selected.0));
        ui.label(lang.current.1);
    });
    ui.horizontal(|ui| {
        ui.label(t!("New language:", locale = lang.selected.0));
        ComboBox::new("new language", "")
            .selected_text(lang.selected.1.to_string())
            .show_ui(ui, |ui| {
                for supported_language in &SUPPORTED_LANGUAGES {
                    ui.selectable_value(
                        &mut lang.selected,
                        supported_language,
                        supported_language.1,
                    );
                }
            });
    });

    ui.add_space(15.);
    ui.separator();

    Sides::new().show(
        ui,
        |_ui| {},
        |ui| {
            if ui
                .button(
                    RichText::new(t!("_ok_with_icon", locale = lang.selected.0))
                        .color(Color32::DARK_GREEN),
                )
                .clicked()
            {
                match pl_file.set_language(lang.selected.0) {
                    Ok(()) => {
                        *pl_modal = PlModal::None;
                    }
                    Err(e) => {
                        lang.err = Some(e.to_string());
                    }
                }
            }
            if ui
                .button(
                    RichText::new(t!("_cancel_with_icon", locale = lang.selected.0))
                        .color(Color32::DARK_RED),
                )
                .clicked()
            {
                *pl_modal = PlModal::None;
            }
        },
    );
}
