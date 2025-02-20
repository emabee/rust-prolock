use crate::{
    ui::viz::{PlModal, Pw, PwFocus},
    PlFile,
};
use egui::{Color32, Key, RichText, Sides, TextEdit, Ui};

#[allow(clippy::too_many_lines)]
pub(super) fn change_password(
    pw: &mut Pw,
    pl_modal: &mut PlModal,
    pl_file: &mut PlFile,
    ui: &mut Ui,
) {
    let mut go_for_it = false;

    ui.set_width(400.0);
    ui.add_space(15.);
    ui.label(
        RichText::new(t!("Change the password for the prolock file"))
            .size(16.)
            .color(Color32::DARK_BLUE),
    );
    ui.add_space(15.);

    // ask for old PW
    ui.add_space(15.);
    ui.horizontal(|ui| {
        ui.add_space(8.);
        ui.add(
            TextEdit::singleline(&mut t!("Current password:"))
                .background_color(Color32::TRANSPARENT)
                .desired_width(170.),
        );
        let response = ui.add(
            TextEdit::singleline(&mut pw.old)
                .desired_width(120.)
                .password(true),
        );
        if matches!(pw.focus, PwFocus::PwOld) {
            response.request_focus();
            pw.focus = PwFocus::None;
        }
        if response.lost_focus()
            && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
        {
            pw.focus = PwFocus::Pw1;
        }
    });

    // ask twice for new PW
    ui.add_space(15.);
    ui.horizontal(|ui| {
        ui.add_space(8.);
        ui.add(
            TextEdit::singleline(&mut t!("New password:"))
                .background_color(Color32::TRANSPARENT)
                .desired_width(170.),
        );
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
    });

    ui.horizontal(|ui| {
        ui.add_space(8.);
        ui.add(
            TextEdit::singleline(&mut t!("Repeat new password:"))
                .background_color(Color32::TRANSPARENT)
                .desired_width(170.),
        );
        let response = ui.add(
            TextEdit::singleline(&mut pw.pw2)
                .desired_width(120.)
                .password(true),
        );
        if matches!(pw.focus, PwFocus::Pw2) {
            response.request_focus();
            pw.focus = PwFocus::None;
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            go_for_it = true;
        }
    });

    if let Some(e) = &pw.error {
        ui.add_space(15.);
        ui.label(RichText::new(e).color(Color32::RED));
    }

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
                *pl_modal = PlModal::None;
            }

            if go_for_it {
                match pl_file.check_password(&pw.old) {
                    Err(e) => pw.error = Some(e.to_string()),
                    Ok(()) => {
                        if pw.pw1 == pw.pw2 {
                            match pl_file.change_password(&pw.old, pw.pw1.clone()) {
                                Ok(()) => {
                                    *pl_modal = PlModal::None;
                                }
                                Err(e) => {
                                    pw.error = Some(e.to_string());
                                }
                            }
                        } else {
                            pw.error = Some(t!("The passwords don't match").to_string());
                        }
                    }
                }
            }
        },
    );
}
