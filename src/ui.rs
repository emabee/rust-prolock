mod actionable;
mod password;
pub mod sizes;
mod viz;

use super::PlFile;
use viz::V;

use eframe::{App, Frame};
use egui::{include_image, Color32, Context, FontFamily, Image, RichText, Theme, TopBottomPanel};

/* TODOs *************************************

Backlog:
- !! cred-Liste editierbar machen (limitieren auf vier?)
- !! Mehrsprachigkeit

- Header visuell abheben
- Suchfeld, um Einträge schnell finden zu können
    - Konkretere Beispiele erzeugen
    - sehr viele Beispiele erzeugen

- Drei-Punkt Menu rechts oben
    - Passwort ändern…
    - Über prolock…
    - Druck-Option: alles als druckbaren Text serialisieren und anzeigen

- About + Hilfe

******************************************* */

pub struct Ui {
    v: V,
    pl_file: PlFile,
    colors: Colors,
}
pub struct Colors {
    pub user: Color32,
    pub secret: Color32,
}
impl Ui {
    pub fn new(pl_file: PlFile) -> Self {
        Ui {
            v: V::new(),
            pl_file,
            colors: Colors {
                user: Color32::DARK_BLUE,
                secret: Color32::DARK_RED,
            },
        }
    }
}

impl App for Ui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        (self.colors.user, self.colors.secret) = match ctx.theme() {
            Theme::Dark => (Color32::LIGHT_BLUE, Color32::LIGHT_RED),
            Theme::Light => (Color32::DARK_BLUE, Color32::DARK_RED),
        };
        if self.v.need_refresh {
            self.v.reset_bundles(
                &self.pl_file.stored.readable.bundles,
                self.pl_file.transient().unwrap(/*should never fail*/),
            );
            self.v.need_refresh = false;
        }

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

                ui.label(format!(
                    "{} entries with {} secrets",
                    self.pl_file.stored.readable.bundles.len(),
                    self.pl_file.stored.readable.bundles.count_secrets(),
                ));

                ui.add_space(ui.available_width() - 80.);

                // TODO: Drei-Punkt Menu rechts oben
                ui.add(Image::new(include_image!("ui/assets/burger.png")));
            });

            ui.add_space(10.);
        });

        if self.pl_file.is_actionable() {
            self.panels_for_actionable_ui(ctx);
        } else {
            self.ask_for_password(ctx);
        }
    }
}
