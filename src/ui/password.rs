use crate::{
    PlFile,
    ui::viz::{PwFocus, V},
};
use egui::{CentralPanel, Color32, Context, RichText, TextEdit, TopBottomPanel};

pub(super) fn ask_for_password(pl_file: &mut PlFile, v: &mut V, ctx: &Context) {
    TopBottomPanel::top("pw error").show(ctx, |ui| {
        if let Some(e) = &v.pw.error {
            ui.label(RichText::new(e).color(Color32::RED));
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        if pl_file.update_counter().peek() == Some(0) {
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
                        switch_to_actionable(pl_file, v);
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
                    switch_to_actionable(pl_file, v);
                }
            });
        }
    });
}

fn switch_to_actionable(pl_file: &mut PlFile, v: &mut V) {
    match pl_file.set_actionable(v.pw.pw1.clone()) {
        Ok(()) => {
            v.pw.error = None;
            v.reset_bundles(
                &pl_file.bundles(),
                pl_file.transient().unwrap(/*should never fail*/),
            );
            if pl_file.is_empty() {
                v.edit_bundle.prepare_for_create();
            }
        }
        Err(e) => {
            v.pw.error = Some(format!("{e}"));
        }
    }
}
