mod change_file;
mod change_language;
mod change_password;
mod create_bundle;
mod delete_bundle;
mod show_about;

pub use change_file::change_file;
pub use change_language::change_language;
pub use change_password::change_password;
pub use create_bundle::create_bundle;
pub use delete_bundle::delete_bundle;
use egui::{Color32, Image, RichText};
pub use show_about::show_about;

fn title_of_modal(o_icon: Option<Image>, title: &str, ui: &mut egui::Ui) {
    ui.set_width(400.0);
    ui.add_space(15.);
    ui.horizontal(|ui| {
        if let Some(icon) = o_icon {
            ui.add(icon);
            ui.add_space(5.);
        }
        ui.label(RichText::new(title).size(16.).color(Color32::DARK_BLUE));
    });
    ui.add_space(15.);
}
