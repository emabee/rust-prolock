use super::viz::PlModal;
use egui::{Context, Id, Modal};

mod change_language;
mod change_password;
mod create_bundle;
mod delete_bundle;
mod show_about;

impl super::Ui {
    pub(super) fn show_modal(&mut self, ctx: &Context) {
        let id = Id::new(match self.v.pl_modal {
            PlModal::None => return, // happens frequently!!
            PlModal::CreateBundle => "CreateBundle",
            PlModal::DeleteBundle(_) => "DeleteBundle",
            PlModal::ChangePassword => "ChangePassword",
            PlModal::About => "About",
            PlModal::ChangeLanguage => "ChangeLanguage",
            PlModal::ShowPrintable => "ShowPrintable",
        });

        let modal_response = Modal::new(id).show(ctx, |ui| match self.v.pl_modal.clone() {
            PlModal::None => unreachable!(), // because of 'return' above
            PlModal::CreateBundle => create_bundle::create_bundle(
                &mut self.v.edit_bundle,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                &mut self.v.need_refresh,
                &self.colors,
                ui,
            ),
            PlModal::DeleteBundle(name) => delete_bundle::delete_bundle(
                &name,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                &mut self.v.edit_idx,
                &mut self.v.need_refresh,
                ui,
            ),
            PlModal::ChangePassword => change_password::change_password(
                &mut self.v.pw,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                ui,
            ),
            PlModal::About => show_about::show_about(&mut self.v.pl_modal, ui),
            PlModal::ChangeLanguage => change_language::change_language(
                &mut self.v.lang,
                &mut self.v.pl_modal,
                &mut self.pl_file,
                ui,
            ),
            PlModal::ShowPrintable => todo!("TODO"),
        });

        if modal_response.should_close()
            && matches!(self.v.pl_modal, PlModal::About | PlModal::ShowPrintable)
        {
            self.v.pl_modal = PlModal::None;
        }
    }
}
