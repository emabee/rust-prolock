use egui::{
    include_image, Color32, Context, Hyperlink, Id, Image, Key, Modal, RichText, Sides, TextEdit,
    Ui, Vec2,
};

use super::viz::{Burger, PwFocus};

impl super::Ui {
    pub(crate) fn show_modal(&mut self, ctx: &Context) {
        if matches!(self.v.burger, Burger::None) {
            return;
        }

        let modal = Modal::new(Id::new("ChangePassword")).show(ctx, |ui| match self.v.burger {
            Burger::None => return,
            Burger::ChangePassword => self.change_password(ui),
            Burger::About => self.show_about(ui),
            Burger::ChangeLanguage => todo!(),
            Burger::ShowPrintable => todo!(),
        });
        if modal.should_close() {
            // self.v.burger = Burger::None;
        }
    }

    fn change_password(&mut self, ui: &mut Ui) {
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
                TextEdit::singleline(&mut self.v.pw.old)
                    .desired_width(120.)
                    .password(true),
            );
            if matches!(self.v.pw.focus, PwFocus::PwOld) {
                response.request_focus();
                self.v.pw.focus = PwFocus::None;
            }
            if response.lost_focus()
                && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
            {
                self.v.pw.focus = PwFocus::Pw1;
            }
        });

        ui.add_space(15.);
        ui.horizontal(|ui| {
            ui.add_space(8.);
            ui.add(
                TextEdit::singleline(&mut t!("New password:"))
                    .background_color(Color32::TRANSPARENT)
                    .desired_width(170.),
            );
            let response = ui.add(
                TextEdit::singleline(&mut self.v.pw.pw1)
                    .desired_width(120.)
                    .password(true),
            );
            if matches!(self.v.pw.focus, PwFocus::Pw1) {
                response.request_focus();
                self.v.pw.focus = PwFocus::None;
            }
            if response.lost_focus()
                && ui.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::Tab))
            {
                self.v.pw.focus = PwFocus::Pw2;
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
                TextEdit::singleline(&mut self.v.pw.pw2)
                    .desired_width(120.)
                    .password(true),
            );
            if matches!(self.v.pw.focus, PwFocus::Pw2) {
                response.request_focus();
                self.v.pw.focus = PwFocus::None;
            }
            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                go_for_it = true;
            }
        });

        ui.add_space(15.);

        if let Some(e) = &self.v.pw.error {
            ui.label(RichText::new(e).color(Color32::RED));
        }

        ui.add_space(15.);

        ui.separator();

        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new(t!("Ok")).color(Color32::DARK_GREEN))
                    .clicked()
                {
                    go_for_it = true;
                }
                if ui
                    .button(RichText::new(t!("Cancel")).color(Color32::DARK_RED))
                    .clicked()
                {
                    self.v.burger = Burger::None;
                }

                if go_for_it {
                    match self.pl_file.check_password(&self.v.pw.old) {
                        Err(e) => self.v.pw.error = Some(e.to_string()),
                        Ok(()) => {
                            if self.v.pw.pw1 == self.v.pw.pw2 {
                                match self
                                    .pl_file
                                    .change_password(&self.v.pw.old, self.v.pw.pw1.clone())
                                {
                                    Ok(()) => {
                                        self.v.burger = Burger::None;
                                    }
                                    Err(e) => {
                                        self.v.pw.error = Some(e.to_string());
                                    }
                                }
                            } else {
                                self.v.pw.error = Some(t!("The passwords don't match").to_string());
                            }
                        }
                    }
                }
            },
        );
    }

    fn show_about(&mut self, ui: &mut Ui) {
        ui.set_width(400.0);
        ui.add_space(10.);
        ui.heading("ProLock");
        ui.add_space(5.);
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.set_width(150.);
                ui.set_height(150.);
                ui.add(Image::new(include_image!("assets/logo.png")));
            });
            ui.vertical(|ui| {
                ui.add(
                    TextEdit::multiline(&mut format!(
                    "{}\n\n{}\n\nVersion: {}",
                    "ProLock is a tool for securely storing secrets in a password-protected file.",
                    "ProLock is written in rust.",
                    env!("CARGO_PKG_VERSION")
                ))
                    .background_color(Color32::TRANSPARENT),
                );

                ui.add_space(15.);

                ui.horizontal(|ui| {
                    ui.label("Repository and README:");
                    ui.hyperlink("https://github.com/emabee/rust-prolock");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2 { x: 3., y: 0. };
                    ui.label(RichText::new(
                        "Please provide your suggestions, proposals, wishes, complaints:",
                    ));
                });
                ui.add(Hyperlink::from_label_and_url(
                    ".../rust-prolock/issues",
                    "https://github.com/emabee/rust-prolock/issues",
                ));

                ui.add_space(15.);
            });
        });
        Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button(RichText::new("✅").color(Color32::DARK_GREEN))
                    .clicked()
                {
                    self.v.burger = Burger::None;
                }
            },
        );
    }
}
