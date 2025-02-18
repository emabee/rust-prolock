use crate::ui::viz::Burger;

use super::{
    viz::{Pw, PwFocus},
    Ui,
};
use egui::{
    include_image, menu::menu_custom_button, Button, Color32, Context, FontFamily, Image, RichText,
    TopBottomPanel,
};

impl Ui {
    pub fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("file").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::LIGHT_GRAY,
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
                    Button::image(Image::new(include_image!("assets/burger.png")))
                        .fill(Color32::TRANSPARENT),
                    |ui| {
                        if ui.button(t!("❓About ProLock")).clicked() {
                            self.v.burger = Burger::About;
                            // FIXME
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(
                                self.pl_file.is_actionable(),
                                Button::new(t!("🔐 Change password")),
                            )
                            .clicked()
                        {
                            self.v.burger = Burger::ChangePassword;
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
                            // FIXME
                            self.v.burger = Burger::ChangeLanguage;
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(
                                self.pl_file.is_actionable(),
                                Button::new(t!("📄 Show content as printable document")),
                            )
                            .clicked()
                        {
                            // FIXME
                            self.v.burger = Burger::ShowPrintable;
                            ui.close_menu();
                        }
                    },
                );
            });

            ui.add_space(10.);
        });
    }
}
