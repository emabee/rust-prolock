use crate::ui::{
    controller::{Action, Controller},
    show_error,
    viz::{PwFocus, V},
};
use egui::{CentralPanel, Color32, Context, Grid, Key, RichText, TextEdit};

pub fn ask_for_password_to_open(
    is_first_start: bool,
    v: &mut V,
    controller: &mut Controller,
    ctx: &Context,
) {
    CentralPanel::default().show(ctx, |ui| {
        if is_first_start {
            ask_twice(v, controller, ui);
        } else {
            ask_once(v, controller, ui);
        }

        if let Some(e) = &v.pw.error {
            show_error(e, ui);
        }
    });
}

fn ask_once(v: &mut V, controller: &mut Controller, ui: &mut egui::Ui) {
    ui.add_space(15.);
    ui.label(
        RichText::new(t!("_opening_the_prolock_file"))
            .size(16.)
            .color(Color32::DARK_BLUE),
    );
    ui.add_space(15.);
    ui.label(t!("_specify_pw_to_open"));
    ui.add_space(15.);

    ui.horizontal(|ui| {
        ui.add_space(50.);
        Grid::new("Password once").num_columns(2).show(ui, |ui| {
            ui.label(t!("Password:"));
            let response = ui.add(
                TextEdit::singleline(&mut v.pw.pw1)
                    .desired_width(120.)
                    .password(true),
            );

            let mut go_forward = false;
            if matches!(v.pw.focus, PwFocus::Pw1) {
                response.request_focus();
                v.pw.focus = PwFocus::None;
            }
            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                go_forward = true;
            }
            if ui.button("OK").clicked() {
                go_forward = true;
            }

            if go_forward {
                controller.set_action(Action::SwitchToActionable);
            }
            ui.end_row();
        });
    });
}

fn ask_twice(v: &mut V, controller: &mut Controller, ui: &mut egui::Ui) {
    // this is the first start, so ask twice
    ui.add_space(15.);
    ui.label(
        RichText::new(t!("Creating the prolock file"))
            .size(16.)
            .color(Color32::DARK_BLUE),
    );
    ui.add_space(15.);
    ui.label(t!("_specify_pw_to_secure"));
    ui.add_space(15.);

    ui.horizontal(|ui| {
        ui.add_space(50.);
        Grid::new("Password twice").num_columns(2).show(ui, |ui| {
            ui.label(t!("Password:"));
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
                && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
            {
                v.pw.focus = PwFocus::Pw2;
            }
            ui.end_row();
            let mut go_forward = false;
            ui.label(t!("Repeat:"));
            let response = ui.add(
                TextEdit::singleline(&mut v.pw.pw2)
                    .desired_width(120.)
                    .password(true),
            );
            if matches!(v.pw.focus, PwFocus::Pw2) {
                response.request_focus();
                v.pw.focus = PwFocus::None;
            }
            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
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
            ui.end_row();
        });
    });
}
