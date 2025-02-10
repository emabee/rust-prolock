use super::{viz::PwFocus, Ui};
use egui::{CentralPanel, Color32, Context, RichText, TextEdit, TopBottomPanel};

impl Ui {
    pub(super) fn ask_for_password(&mut self, ctx: &Context) {
        TopBottomPanel::top("pw error").show(ctx, |ui| {
            if let Some(e) = &self.v.pw.error {
                ui.label(RichText::new(e).color(Color32::RED));
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            if self.pl_file.stored.readable.header.update_counter.peek() == Some(0) {
                // this is the first start, so ask twice
                ui.add_space(15.);
                ui.label(
                    RichText::new("Creating the prolock file")
                        .size(16.)
                        .color(Color32::DARK_BLUE),
                );
                ui.add_space(15.);
                ui.label("Specify the password to secure your prolock file:");
                ui.add_space(15.);
                ui.horizontal(|ui| {
                    ui.add_space(50.);
                    ui.add(TextEdit::singleline(&mut "Password:").desired_width(80.));
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
                        && ui.input(|i| {
                            i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Tab)
                        })
                    {
                        self.v.pw.focus = PwFocus::Pw2;
                    }
                });

                ui.horizontal(|ui| {
                    let mut go_forward = false;
                    ui.add_space(50.);
                    ui.add(TextEdit::singleline(&mut "Repeat:").desired_width(80.));
                    let response = ui.add(
                        TextEdit::singleline(&mut self.v.pw.pw2)
                            .desired_width(120.)
                            .password(true),
                    );
                    if matches!(self.v.pw.focus, PwFocus::Pw2) {
                        response.request_focus();
                        self.v.pw.focus = PwFocus::None;
                    }
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        go_forward = true;
                    }
                    if ui.button("OK").clicked() {
                        go_forward = true;
                    }
                    if go_forward {
                        if self.v.pw.pw1 == self.v.pw.pw2 {
                            self.switch_to_actionable();
                        } else {
                            self.v.pw.error = Some("The passwords don't match".to_string());
                        }
                    }
                });
            } else {
                // ask once
                ui.horizontal(|ui| {
                    let mut go_forward = false;
                    ui.add_space(50.);
                    ui.add(TextEdit::singleline(&mut "Password:").desired_width(80.));
                    let response = ui.add(
                        TextEdit::singleline(&mut self.v.pw.pw1)
                            .desired_width(120.)
                            .password(true),
                    );
                    if matches!(self.v.pw.focus, PwFocus::Pw1) {
                        response.request_focus();
                        self.v.pw.focus = PwFocus::None;
                    }
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        go_forward = true;
                    }
                    if ui.button("OK").clicked() {
                        go_forward = true;
                    }

                    if go_forward {
                        self.switch_to_actionable();
                    }
                });
            }
        });
    }

    fn switch_to_actionable(&mut self) {
        match self.pl_file.set_actionable(self.v.pw.pw1.clone()) {
            Ok(()) => {
                self.v.pw.error = None;
                self.v.reset_bundles(
                    &self.pl_file.stored.readable.bundles,
                    self.pl_file.transient().unwrap(/*should never fail*/),
                );
            }
            Err(e) => {
                self.v.pw.error = Some(format!("{e:?}"));
            }
        }
    }
}
