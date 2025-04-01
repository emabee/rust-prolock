use crate::{
    controller::{Action, Controller},
    ui::sizes::MODAL_WIDTH,
};
use egui::{Color32, Context, FontFamily, FontId, Modal, RichText, ScrollArea, Sides, TextEdit};
use egui_extras::{Size, StripBuilder};

pub fn show_log(text: &str, controller: &mut Controller, ctx: &Context) {
    let modal_response = Modal::new("action_log".into()).show(ctx, |ui| {
        ui.set_width(MODAL_WIDTH);
        let mut text1 = text;
        let height = 200.;
        ui.heading(t!("Action log"));
        ui.add_space(5.);
        ui.separator();
        ui.add_space(5.);

        StripBuilder::new(ui)
            .size(Size::exact(height))
            .vertical(|mut log_strip| {
                log_strip.cell(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [MODAL_WIDTH, height],
                            TextEdit::multiline(&mut text1).interactive(true),
                        );
                    });
                });
            });

        ui.add_space(15.);
        ui.separator();
        ui.add_space(5.);

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(
                        RichText::new("✅")
                            .color(Color32::DARK_GREEN)
                            .font(FontId::new(20., FontFamily::Proportional)),
                    )
                    .clicked()
                {
                    controller.set_action(Action::SilentCancel);
                }
            },
        );
    });
    if modal_response.should_close() {
        controller.set_action(Action::SilentCancel);
    }
}
