use crate::{
    SUPPORTED_LANGUAGES,
    ui::{
        controller::{Action, Controller},
        sizes::MODAL_WIDTH,
        viz::Lang,
    },
};
use egui::{Color32, ComboBox, Context, FontFamily, FontId, Grid, Modal, RichText, Sides};

pub fn change_language(lang: &mut Lang, controller: &mut Controller, ctx: &Context) {
    let modal_response = Modal::new("change_language".into()).show(ctx, |ui| {
        ui.set_width(MODAL_WIDTH);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(140.);
                ui.set_height(140.);
                ui.add_space(50.);
                ui.label(RichText::new("🌐").font(FontId::new(128., FontFamily::Proportional)));
            });

            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(RichText::new(t!("Change language", locale = lang.selected.0)).size(24.));

                ui.add_space(15.);

                Grid::new("Change Password").num_columns(2).show(ui, |ui| {
                    ui.label(t!("Current language:", locale = lang.selected.0));
                    ui.label(lang.current.1);

                    ui.end_row();

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
            });
        });

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
                    controller.set_action(Action::FinalizeChangeLanguage);
                }
                if ui
                    .button(
                        RichText::new(t!("_cancel_with_icon", locale = lang.selected.0))
                            .color(Color32::DARK_RED),
                    )
                    .clicked()
                {
                    controller.set_action(Action::CloseModal);
                }
            },
        );
    });
    if modal_response.should_close() {
        controller.set_action(Action::CloseModal);
    }
}
