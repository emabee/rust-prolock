mod actionable;
mod assets;
mod modals;
mod password;
pub mod sizes;
mod top_panel;
mod viz;

use crate::{
    data::{PlFile, Settings},
    ui::{
        actionable::panels_for_actionable_ui,
        assets::{
            IMG_ADD_ENTRY, IMG_ADD_ENTRY_INACTIVE, IMG_BURGER, IMG_CANCEL, IMG_DELETE,
            IMG_DELETE_INACTIVE, IMG_EDIT, IMG_EDIT_INACTIVE, IMG_LOGO, IMG_OK, IMG_RUST_LOGO,
            IMG_SAVE, IMG_SEARCH,
        },
        modals::{
            change_file, change_language, change_password, create_bundle, delete_bundle, show_about,
        },
        password::ask_for_password,
        top_panel::top_panel,
        viz::{FileAction, PlModal, V},
    },
};
use anyhow::{Context as _, Result};
use eframe::{App, Frame};
use egui::{Color32, Context, Theme};
use std::path::PathBuf;

pub const VERY_LIGHT_GRAY: Color32 = Color32::from_rgb(235, 235, 235);

pub struct Ui {
    v: V,
    settings: Settings,
    o_plfile: Option<PlFile>,
    colors: Colors,
}
pub struct Colors {
    pub user: Color32,
    pub secret: Color32,
}
impl Ui {
    pub fn new(settings: Settings) -> Result<Self> {
        let colors = Colors {
            user: Color32::DARK_BLUE,
            secret: Color32::DARK_RED,
        };
        let mut v = V::new();
        v.file_selection.reset(settings.current_file);
        Ok(Ui {
            v,
            o_plfile: Some(
                PlFile::read_or_create(settings.current_file()).context("File open error")?,
            ),
            settings,
            colors,
        })
    }
}

impl App for Ui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        (self.colors.user, self.colors.secret) = match ctx.theme() {
            Theme::Dark => (Color32::LIGHT_BLUE, Color32::LIGHT_RED),
            Theme::Light => (Color32::DARK_BLUE, Color32::DARK_RED),
        };

        if let Some(file_action) = &self.v.file_selection.o_action {
            match file_action {
                FileAction::SwitchToKnown(idx) => {
                    self.settings.set_current_file(*idx).unwrap();
                }
                FileAction::SwitchToNew(path) => {
                    self.settings
                        .add_and_set_file(&PathBuf::from(path))
                        .unwrap();
                }
            }
            let pl_file = PlFile::read_or_create(self.settings.current_file())
                .context("File open error")
                .unwrap();
            self.o_plfile = Some(pl_file);
            self.v = V::new();
            self.v.file_selection.reset(self.settings.current_file);
        }

        if let Some(pl_file) = &mut self.o_plfile {
            if self.v.need_refresh {
                self.v.reset_bundles(
                    pl_file.bundles(),
                    pl_file.transient().unwrap(/*should never fail*/),
                );
                self.v.need_refresh = false;
            }

            // UI
            top_panel(&self.settings, pl_file, &mut self.v, ctx);

            match self.v.pl_modal.clone() {
                PlModal::CreateBundle => {
                    create_bundle(
                        &mut self.v.edit_bundle,
                        &mut self.v.pl_modal,
                        pl_file,
                        &mut self.v.need_refresh,
                        &self.colors,
                        ctx,
                    );
                }
                PlModal::DeleteBundle(ref name) => {
                    delete_bundle(
                        name,
                        &mut self.v.pl_modal,
                        pl_file,
                        &mut self.v.edit_idx,
                        &mut self.v.need_refresh,
                        ctx,
                    );
                }
                PlModal::About => {
                    show_about(&mut self.v.pl_modal, ctx);
                }
                PlModal::ChangePassword => {
                    change_password(&mut self.v.pw, &mut self.v.pl_modal, pl_file, ctx);
                }
                PlModal::ChangeFile => {
                    change_file(
                        &mut self.v.pl_modal,
                        &mut self.v.file_selection,
                        &mut self.settings,
                        ctx,
                    );
                }
                PlModal::ChangeLanguage => {
                    change_language(
                        &mut self.v.lang,
                        &mut self.v.pl_modal,
                        &mut self.settings,
                        ctx,
                    );
                    return;
                }
                PlModal::None | PlModal::ShowPrintable => {
                    // TODO ShowPrintable
                }
            }

            if pl_file.is_actionable() {
                panels_for_actionable_ui(pl_file, &mut self.v, &self.colors, ctx);
            } else {
                ask_for_password(pl_file, &mut self.v, ctx);
            }
        } else {
            change_file(
                &mut self.v.pl_modal,
                &mut self.v.file_selection,
                &mut self.settings,
                ctx,
            );
        }
    }
}

impl Ui {}
