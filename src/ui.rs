mod actionable;
mod assets;
pub mod colors;
mod modals;
mod password;
pub mod sizes;
mod top_panel;
pub mod viz;

use crate::{
    controller::{Controller, PlModal},
    data::{PlFile, Settings},
    ui::{
        actionable::panels_for_actionable_ui,
        assets::{
            IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_BURGER, IMG_CANCEL, IMG_DELETE,
            IMG_DELETE_INACTIVE, IMG_EDIT, IMG_EDIT_INACTIVE, IMG_LOGO, IMG_OK, IMG_RUST_LOGO,
            IMG_SAVE,
        },
        modals::{
            change_file, change_language, change_password, create_bundle, delete_bundle, show_about,
        },
        password::ask_for_password,
        top_panel::top_panel,
        viz::V,
    },
};
use anyhow::{Context as _, Result};
use eframe::{App, Frame};
use egui::{Color32, Context};

pub const LIGHT_GRAY: Color32 = Color32::from_rgb(230, 230, 230);
pub const VERY_LIGHT_GRAY: Color32 = Color32::from_rgb(235, 235, 235);

pub struct Ui {
    o_plfile: Option<PlFile>,
    v: V,
    controller: Controller,
    settings: Settings,
}
impl Ui {
    pub fn new(settings: Settings) -> Result<Self> {
        let mut v = V::default();
        v.file_selection.reset(settings.current_file);
        Ok(Ui {
            o_plfile: Some(
                PlFile::read_or_create(settings.current_file()).context("File open error")?,
            ),
            v,
            controller: Controller::default(),
            settings,
        })
    }
}

impl App for Ui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // execute actions
        self.controller
            .act(&mut self.o_plfile, &mut self.v, &mut self.settings);

        // update the UI
        if let Some(pl_file) = &mut self.o_plfile {
            top_panel(pl_file, &mut self.v, &mut self.controller, ctx);

            match self.controller.current_modal() {
                PlModal::None => {}

                PlModal::CreateBundle => {
                    create_bundle(&mut self.v.edit_bundle, &mut self.controller, ctx);
                }
                PlModal::DeleteBundle => {
                    delete_bundle(&self.v.name_for_delete, &mut self.controller, ctx);
                }
                PlModal::About => {
                    show_about(&mut self.controller, ctx);
                }
                PlModal::ChangePassword => {
                    change_password(&mut self.v.pw, &mut self.controller, ctx);
                }
                PlModal::ChangeFile => {
                    change_file(
                        &mut self.settings,
                        &mut self.v.file_selection,
                        &mut self.controller,
                        ctx,
                    );
                }
                PlModal::ChangeLanguage => {
                    change_language(&mut self.v.lang, &mut self.controller, ctx);
                }
            }

            if let Some(transient) = pl_file.transient() {
                panels_for_actionable_ui(
                    pl_file.bundles(),
                    transient,
                    &mut self.v,
                    &mut self.controller,
                    ctx,
                );
            } else {
                let is_first_start = pl_file.update_counter().peek() == Some(0);
                ask_for_password(is_first_start, &mut self.v, &mut self.controller, ctx);
            }
        } else {
            change_file(
                &mut self.settings,
                &mut self.v.file_selection,
                &mut self.controller,
                ctx,
            );
        }
    }
}

impl Ui {}
