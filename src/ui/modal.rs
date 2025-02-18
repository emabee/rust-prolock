use super::viz::PlModal;
use egui::{include_image, Context, Id, ImageSource, Modal};

mod change_password;
mod create_bundle;
mod show_about;
const IMG_SAVE: ImageSource = include_image!("./modal/assets/save.png");
const IMG_CANCEL: ImageSource = include_image!("./modal/assets/cancel.png");

impl super::Ui {
    pub(super) fn show_modal(&mut self, ctx: &Context) {
        let id = Id::new(match self.v.pl_modal {
            PlModal::None => return,
            PlModal::ChangePassword => "ChangePassword",
            PlModal::About => "About",
            PlModal::ChangeLanguage => "ChangeLanguage",
            PlModal::ShowPrintable => "ShowPrintable",
            PlModal::CreateBundle => "CreateBundle",
        });

        let modal_response = Modal::new(id).show(ctx, |ui| match self.v.pl_modal {
            PlModal::None => unreachable!(),
            PlModal::ChangePassword => change_password::change_password(
                &mut self.v.pw,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                ui,
            ),
            PlModal::About => show_about::show_about(&mut self.v.pl_modal, ui),
            PlModal::ChangeLanguage => todo!(),
            PlModal::ShowPrintable => todo!(),
            PlModal::CreateBundle => create_bundle::create_bundle(
                &mut self.v.edit_bundle,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                &mut self.v.need_refresh,
                &self.colors,
                ui,
            ),
        });

        if modal_response.should_close() && matches!(self.v.pl_modal, PlModal::About) {
            self.v.pl_modal = PlModal::None;
        }
    }
}
