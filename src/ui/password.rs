use crate::{
    controller::{Action, Controller},
    ui::viz::{PwFocus, V},
};
use egui::{CentralPanel, Color32, Context, Grid, Key, RichText, TextEdit, TopBottomPanel};

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
        } else {
            // ask once
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
    });
}
