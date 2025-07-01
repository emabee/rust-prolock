use crate::ui::{
    controller::{Action, Controller},
    show_error,
    sizes::MODAL_WIDTH,
    viz::{Pw, PwFocus},
};
use egui::{Color32, Context, FontFamily, FontId, Grid, Key, Modal, RichText, Sides, TextEdit};

#[allow(clippy::too_many_lines)]
pub fn change_password(pw: &mut Pw, controller: &mut Controller, ctx: &Context) {
    let modal_response = Modal::new("change_password".into()).show(ctx, |ui| {
        let mut go_for_it = false;

        ui.set_width(MODAL_WIDTH);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(140.);
                ui.set_height(140.);
                ui.add_space(50.);
                ui.label(RichText::new("🔐").font(FontId::new(128., FontFamily::Proportional)));
            });
            ui.vertical(|ui| {
                ui.add_space(50.);
                ui.label(RichText::new(t!("Change password")).size(24.));

                ui.add_space(30.);

                Grid::new("Change Password").num_columns(2).show(ui, |ui| {
                    ui.label(t!("Current password:"));
                    let response = ui.add(
                        TextEdit::singleline(&mut pw.pw1)
                            .desired_width(120.)
                            .password(true),
                    );
                    if matches!(pw.focus, PwFocus::Pw1) {
                        response.request_focus();
                        pw.focus = PwFocus::None;
                    }
                    if response.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
                    {
                        pw.focus = PwFocus::Pw2;
                    }
                    ui.end_row();
                    ui.end_row();

                    ui.label(t!("New password:"));
                    let response = ui.add(
                        TextEdit::singleline(&mut pw.pw2)
                            .desired_width(120.)
                            .password(true),
                    );
                    if matches!(pw.focus, PwFocus::Pw2) {
                        response.request_focus();
                        pw.focus = PwFocus::None;
                    }
                    if response.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
                    {
                        pw.focus = PwFocus::Pw3;
                    }
                    ui.end_row();

                    ui.label(t!("Repeat new password:"));
                    let response = ui.add(
                        TextEdit::singleline(&mut pw.pw3)
                            .desired_width(120.)
                            .password(true),
                    );
                    if matches!(pw.focus, PwFocus::Pw3) {
                        response.request_focus();
                        pw.focus = PwFocus::None;
                    }
                    if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                        go_for_it = true;
                    }
                    ui.end_row();
                });

                if let Some(e) = &pw.error {
                    show_error(e, ui);
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
                    .button(RichText::new(t!("_ok_with_icon")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    go_for_it = true;
                }
                if ui
                    .button(RichText::new(t!("_cancel_with_icon")).color(Color32::DARK_RED))
                    .clicked()
                {
                    controller.set_action(Action::CloseModal);
                }
            },
        );

        if go_for_it {
            if pw.pw2 == pw.pw3 {
                controller.set_action(Action::FinalizeChangePassword {
                    old: pw.pw1.clone(),
                    new: pw.pw2.clone(),
                });
            } else {
                pw.error = Some(t!("_passwords_dont_match").to_string());
            }
        }
    });
    if modal_response.should_close() {
        controller.set_action(Action::CloseModal);
    }
}
