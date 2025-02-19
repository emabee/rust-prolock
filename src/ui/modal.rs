use super::viz::PlModal;
use egui::{Context, Id, Modal};

mod change_password;
mod create_bundle;
mod show_about;

impl super::Ui {
    pub(super) fn show_modal(&mut self, ctx: &Context) {
        let id = Id::new(match self.v.pl_modal {
            PlModal::None => return, // happens frequently!!
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
            PlModal::ChangeLanguage => todo!("FIXME"),
            PlModal::ShowPrintable => todo!("FIXME"),
            PlModal::CreateBundle => create_bundle::create_bundle(
                &mut self.v.edit_bundle,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                &mut self.v.need_refresh,
                &self.colors,
                ui,
            ),
        });

        if modal_response.should_close()
            && matches!(self.v.pl_modal, PlModal::About | PlModal::ShowPrintable)
        {
            self.v.pl_modal = PlModal::None;
        }
    }
}
