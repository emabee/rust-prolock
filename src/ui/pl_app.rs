use crate::{
    Controller,
    data::{PlFile, Settings},
    ui::{
        main_ui::{ask_for_password_to_open, main_ui, show_log},
        modals::{
            change_file, change_language, change_password, configure_password_generation,
            create_bundle, create_document, delete_bundle, delete_document, import_data,
            modal_export_data, show_about,
        },
        top_panel::top_panel,
        viz::{ModalState, V},
    },
};
use anyhow::{Context as _, Result};
use eframe::App;
use egui::{
    FontFamily::Proportional,
    FontId,
    TextStyle::{Body, Button, Heading, Monospace, Name, Small},
    Ui,
};
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

    fn show_modal_if_required(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();
        match self.v.modal_state {
            ModalState::None => {}

            ModalState::AddBundle {
                v_edit_bundle: ref mut bundle,
                generate_pw,
                ref error,
            } => {
                create_bundle(bundle, error.as_deref(), &mut self.controller, ctx);
                if generate_pw {
                    configure_password_generation(
                        &mut self.v.generate_pw,
                        &mut self.controller,
                        ui,
                    );
                }
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
            ModalState::ChangeFile {
                ref mut file_dialog,
            } => {
                change_file(
                    file_dialog,
                    &mut self.settings,
                    &mut self.v.file_selection,
                    &mut self.controller,
                    ui,
                );
            }
            ModalState::ChangeLanguage => {
                change_language(&mut self.v.lang, &mut self.controller, ctx);
            }
            ModalState::GeneratePassword => {
                configure_password_generation(&mut self.v.generate_pw, &mut self.controller, ctx);
            }
            ModalState::ExportData {
                ref mut export_tab,
                ref mut export_data,
                ref mut file_dialog,
            } => {
                modal_export_data(
                    export_data,
                    export_tab,
                    file_dialog,
                    &mut self.controller,
                    ctx,
                );
            }
            ModalState::ImportData {
                ref mut step,
                ref mut error,
            } => {
                import_data(step, error, &mut self.controller, ctx);
            }
        }
    }
}

impl App for PlApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        set_basic_styles(ui);

        // execute action, if any
        self.controller
            .act(&mut self.pl_file, &mut self.v, &mut self.settings);

        // render the UI
        top_panel(&self.pl_file, &mut self.v, &mut self.controller, ui);

        if let Some(transient) = self.pl_file.transient() {
            main_ui(
                self.pl_file.bundles(),
                self.pl_file.documents(),
                transient,
                &mut self.v,
                &mut self.controller,
                ui,
            );
        } else {
            let is_first_start = self.pl_file.update_counter().peek() == Some(0);
            ask_for_password_to_open(is_first_start, &mut self.v, &mut self.controller, ui);
        }

        // show the log
        if self.v.show_log {
            show_log(
                &self.logger_handle,
                &mut self.v.logger_snapshot,
                &mut self.v.show_log,
                ui.ctx(),
            );
        }

        // show modal if desired
        self.show_modal_if_required(ui);
    }
}

fn set_basic_styles(ui: &mut Ui) {
    ui.ctx().all_styles_mut(move |style| {
        style.text_styles = [
            (Heading, FontId::new(20.0, Proportional)),
            (Name("Heading2".into()), FontId::new(18.0, Proportional)),
            (Name("Context".into()), FontId::new(16.0, Proportional)),
            (Body, FontId::new(14.0, Proportional)),
            (Monospace, FontId::new(12.0, Proportional)),
            (Button, FontId::new(12.0, Proportional)),
            (Small, FontId::new(9.0, Proportional)),
        ]
        .into();
    });
    ui.visuals_mut().button_frame = true;
}
