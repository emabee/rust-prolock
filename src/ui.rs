mod actionable;
mod assets;
mod modals;
mod password;
pub mod sizes;
mod top_panel;
mod viz;

use super::PlFile;
use eframe::{App, Frame};
use egui::{Color32, Context, Theme};
use viz::V;

use assets::{
    IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_BURGER, IMG_CANCEL, IMG_DELETE, IMG_DELETE_INACTIVE,
    IMG_EDIT, IMG_EDIT_INACTIVE, IMG_LOGO, IMG_OK, IMG_SAVE, IMG_SEARCH,
};

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

        self.show_modal(ctx);

        if self.pl_file.is_actionable() {
            self.panels_for_actionable_ui(ctx);
        } else {
            self.ask_for_password(ctx);
        }
    }
}
