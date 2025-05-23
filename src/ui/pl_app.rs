use crate::{
    data::{PlFile, Settings},
    ui::{
        main_ui::main_ui,
        controller::Controller,
        modals::{
            change_file, change_language, change_password, create_bundle, create_document,
            delete_bundle, delete_document, show_about, show_log,
        },
        password::ask_for_password,
        top_panel::top_panel,
        viz::{ModalState, V},
    },
};
use anyhow::{Context as _, Result};
use eframe::{App, Frame};
use egui::Context;
use flexi_logger::LoggerHandle;

pub struct PlApp {
    pl_file: PlFile,
    v: V,
    controller: Controller,
    settings: Settings,
    logger_handle: LoggerHandle,
}
impl PlApp {
    pub fn new(logger_handle: LoggerHandle, settings: Settings) -> Result<Self> {
        let mut v = V::default();
        v.file_selection.reset(settings.current_file);
        let pl_file =
            PlFile::read_or_create(settings.current_file()).context("PlFile open error")?;
        log::info!("{} {}", t!("Starting with file"), pl_file.file_path());
        Ok(PlApp {
            pl_file,
            v,
            controller: Controller::default(),
            settings,
            logger_handle,
        })
    }
}

impl App for PlApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // execute actions
        self.controller
            .act(&mut self.pl_file, &mut self.v, &mut self.settings);

        // render the UI
        top_panel(&self.pl_file, &mut self.v, &mut self.controller, ctx);

        // show modal if desired
        match self.v.modal_state {
            ModalState::None => {}

            ModalState::AddBundle {
                v_edit_bundle: ref mut bundle,
                ref error,
            } => {
                create_bundle(bundle, error.as_deref(), &mut self.controller, ctx);
            }
            ModalState::DeleteBundle { ref key, ref error } => {
                delete_bundle(key, error.as_deref(), &mut self.controller, ctx);
            }

            ModalState::AddDocument {
                ref mut v_edit_document,
                ref mut error,
            } => {
                create_document(v_edit_document, error, &mut self.controller, ctx);
            }
            ModalState::DeleteDocument { ref key, ref error } => {
                delete_document(key, error.as_deref(), &mut self.controller, ctx);
            }

            ModalState::About => {
                show_about(&mut self.controller, ctx);
            }
            ModalState::ChangePassword => {
                change_password(&mut self.v.pw, &mut self.controller, ctx);
            }
            ModalState::ChangeFile => {
                change_file(
                    &mut self.settings,
                    &mut self.v.file_selection,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::ChangeLanguage => {
                change_language(&mut self.v.lang, &mut self.controller, ctx);
            } // _ => {}
        }

        // show the log
        if self.v.show_log {
            show_log(
                &self.logger_handle,
                &mut self.v.logger_snapshot,
                &mut self.v.show_log,
                ctx,
            );
        }

        // show the main UI
        if let Some(transient) = self.pl_file.transient() {
            main_ui(
                self.pl_file.bundles(),
                self.pl_file.documents(),
                transient,
                &mut self.v,
                &mut self.controller,
                ctx,
            );
        } else {
            let is_first_start = self.pl_file.update_counter().peek() == Some(0);
            ask_for_password(is_first_start, &mut self.v, &mut self.controller, ctx);
        }
    }
}
