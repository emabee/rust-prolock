use crate::ui::{
    viz::{PlModal, Pw, PwFocus},
    Ui, IMG_BURGER,
};
use egui::{
    menu::menu_custom_button, Button, Color32, Context, FontFamily, Image, RichText, TopBottomPanel,
};

impl Ui {
    pub fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("file").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::GRAY,
                    RichText::new(self.pl_file.file_path.display().to_string())
                        .family(FontFamily::Monospace),
                );
                ui.add_space(10.);
                ui.label("  –—  ");
                ui.add_space(10.);

                ui.label(t!(
                    "entries_with_secrets %{n1} %{n2}",
                    n1 = self.pl_file.stored.readable.bundles.len(),
                    n2 = self.pl_file.stored.readable.bundles.count_secrets()
                ));

                ui.add_space(ui.available_width() - 80.);

                menu_custom_button(
                    ui,
                    Button::image(Image::new(IMG_BURGER)).fill(Color32::TRANSPARENT),
                    |ui| {
                        if ui.button(t!("❓About ProLock")).clicked() {
                            self.v.pl_modal = PlModal::About;
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(
                                self.pl_file.is_actionable(),
                                Button::new(t!("🔐 Change password")),
                            )
                            .clicked()
                        {
                            self.v.pl_modal = PlModal::ChangePassword;
                            self.v.pw = Pw::default();
                            self.v.pw.focus = PwFocus::PwOld;
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(
                                self.pl_file.is_actionable(),
                                Button::new(t!("🌍 Change language")),
                            )
                            .clicked()
                        {
                            self.v.lang.init(self.pl_file.language());
                            self.v.pl_modal = PlModal::ChangeLanguage;
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(
                                false, //self.pl_file.is_actionable(),
                                Button::new(t!("📄 Show content as printable document")),
                            )
                            .clicked()
                        {
                            self.v.pl_modal = PlModal::ShowPrintable;
                            ui.close_menu();
                        }
                    },
                );
            });

            ui.add_space(10.);
        });
    }
}
