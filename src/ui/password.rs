use crate::{
    controller::{Action, Controller},
    ui::viz::{PwFocus, V},
};
use egui::{CentralPanel, Color32, Context, RichText, TextEdit, TopBottomPanel};

pub(super) fn ask_for_password(
    is_first_start: bool,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    TopBottomPanel::top("pw error").show(ctx, |ui| {
        if let Some(e) = &v.pw.error {
            ui.label(RichText::new(e).color(Color32::RED));
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        if is_first_start {
            // this is the first start, so ask twice
            ui.add_space(15.);
            ui.label(
                RichText::new(t!("Creating the prolock file"))
                    .size(16.)
                    .color(Color32::DARK_BLUE),
            );
            ui.add_space(15.);
            ui.label(t!("Specify the password to secure your prolock file:"));
            ui.add_space(15.);
            ui.horizontal(|ui| {
                ui.add_space(50.);
                ui.add(TextEdit::singleline(&mut t!("Password:")).desired_width(80.));
                let response = ui.add(
                    TextEdit::singleline(&mut v.pw.pw1)
                        .desired_width(120.)
                        .password(true),
                );
                if matches!(v.pw.focus, PwFocus::Pw1) {
                    response.request_focus();
                    v.pw.focus = PwFocus::None;
                }
                if response.lost_focus()
                    && ui
                        .input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Tab))
                {
                    v.pw.focus = PwFocus::Pw2;
                }
            });

            ui.horizontal(|ui| {
                let mut go_forward = false;
                ui.add_space(50.);
                ui.add(TextEdit::singleline(&mut t!("Repeat:")).desired_width(80.));
                let response = ui.add(
                    TextEdit::singleline(&mut v.pw.pw2)
                        .desired_width(120.)
                        .password(true),
                );
                if matches!(v.pw.focus, PwFocus::Pw2) {
                    response.request_focus();
                    v.pw.focus = PwFocus::None;
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    go_forward = true;
                }
                if ui.button("OK").clicked() {
                    go_forward = true;
                }
                if go_forward {
                    if v.pw.pw1 == v.pw.pw2 {
                        controller.set_action(Action::SwitchToActionable);
                    } else {
                        v.pw.error = Some(t!("The passwords don't match").to_string());
                    }
                }
            });
        } else {
            // ask once
            ui.horizontal(|ui| {
                let mut go_forward = false;
                ui.add_space(50.);
                ui.add(TextEdit::singleline(&mut t!("Password:")).desired_width(80.));
                let response = ui.add(
                    TextEdit::singleline(&mut v.pw.pw1)
                        .desired_width(120.)
                        .password(true),
                );
                if matches!(v.pw.focus, PwFocus::Pw1) {
                    response.request_focus();
                    v.pw.focus = PwFocus::None;
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    go_forward = true;
                }
                if ui.button("OK").clicked() {
                    go_forward = true;
                }

                if go_forward {
                    controller.set_action(Action::SwitchToActionable);
                }
            });
        }
    });
}
