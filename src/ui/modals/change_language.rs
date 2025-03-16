use crate::{
    SUPPORTED_LANGUAGES,
    controller::{Action, Controller},
    ui::viz::Lang,
};
use egui::{Color32, ComboBox, Context, FontFamily, FontId, Modal, RichText, Sides};

pub fn change_language(lang: &mut Lang, controller: &mut Controller, ctx: &Context) {
    let modal_response = Modal::new("change_language".into()).show(ctx, |ui| {
        ui.set_width(420.0);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(140.);
                ui.set_height(140.);
                ui.add_space(50.);
                ui.label(RichText::new("🌐").font(FontId::new(128., FontFamily::Proportional)));
            });
            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(RichText::new(t!("Change language")).size(24.));

                ui.add_space(15.);

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
                    controller.set_action(Action::FinalizeChangeLanguage);
                }
                if ui
                    .button(
                        RichText::new(t!("_cancel_with_icon", locale = lang.selected.0))
                            .color(Color32::DARK_RED),
                    )
                    .clicked()
                {
                    controller.set_action(Action::Cancel);
                }
            },
        );
    });
    if modal_response.should_close() {
        controller.set_action(Action::Cancel);
    }
}
