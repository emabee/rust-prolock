mod actionable;
mod password;
pub mod sizes;
mod top_panel;
mod viz;

use super::PlFile;
use viz::V;

use eframe::{App, Frame};
use egui::{Color32, Context, Theme};

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

        self.top_panel(ctx);

        match self.v.burger {
            viz::Burger::About => unimplemented!(""), //self.panel_about(ctx),
            viz::Burger::ChangePassword => self.change_password(ctx),
            viz::Burger::ChangeLanguage => unimplemented!(""), //self.change_language(ctx),
            viz::Burger::ShowPrintable => unimplemented!(""),  //self.show_printable(),
            viz::Burger::None => {
                if self.pl_file.is_actionable() {
                    self.panels_for_actionable_ui(ctx);
                } else {
                    self.ask_for_password(ctx);
                }
            }
        }
    }
}
