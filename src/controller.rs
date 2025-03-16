use crate::{
    PlFile, Settings,
    ui::viz::{EditIdx, Pw, V, VEditBundle, VEditCred},
};
use anyhow::{Context, anyhow};
use std::path::PathBuf;

// Implements the controller for the application, i.e., the logic that decides what to do next.
// The controller is a state machine that reacts to user input and decides what to do next.
// The controller is the only place where the application data (FIXME state?) is modified.
#[derive(Default)]
pub struct Controller {
    // The next action to be taken by the controller.
    next_action: Action,
    // UI to be shown to the user.
    current_modal: PlModal,
}
impl Controller {
    pub fn start_add(&mut self, v: &mut V) {
        self.current_modal = PlModal::CreateBundle;
        v.edit_bundle.prepare_for_create();
    }
    pub fn start_change_password(&mut self) {
        self.next_action = Action::StartChangePassword;
    }
    pub fn start_change_file(&mut self) {
        self.next_action = Action::StartChangeFile;
    }
    pub fn show_about(&mut self) {
        self.current_modal = PlModal::About;
    }
    pub fn show_change_language(&mut self) {
        self.current_modal = PlModal::ChangeLanguage;
        self.next_action = Action::StartChangeLanguage;
    }
    pub fn finalize_change_language(&mut self) {
        self.next_action = Action::FinalizeChangeLanguage;
    }
    pub fn finalize_change_password(&mut self, old: String, new: String) {
        self.next_action = Action::FinalizeChangePassword { old, new };
    }
    pub fn switch_to_known_file(&mut self, idx: usize) {
        self.next_action = Action::SwitchToKnownFile(idx);
    }
    pub fn switch_to_new_file(&mut self, path: String) {
        self.next_action = Action::SwitchToNewFile(path);
    }
    pub fn finalize_add(&mut self) {
        self.next_action = Action::FinalizeAdd;
    }
    pub fn start_modify(&mut self, index: usize, name: String) {
        self.next_action = Action::StartModify(index, name);
    }
    pub fn finalize_modify(&mut self) {
        self.next_action = Action::FinalizeModify;
    }
    pub fn start_delete(&mut self, name: String) {
        self.next_action = Action::StartDelete(name);
    }
    pub fn finalize_delete(&mut self, name: String) {
        self.next_action = Action::FinalizeDelete(name);
    }
    pub fn cancel(&mut self) {
        self.next_action = Action::Cancel;
    }

    #[allow(clippy::too_many_lines)]
    pub fn act(&mut self, o_plfile: &mut Option<PlFile>, v: &mut V, settings: &mut Settings) {
        // println!("{:?}", self.next_action);
        match std::mem::take(&mut self.next_action) {
            Action::None => {}

            Action::StartChangePassword => {
                self.current_modal = PlModal::ChangePassword;
                v.pw = Pw::default();
            }
            Action::FinalizeChangePassword { old, new } => {
                match o_plfile.as_mut().unwrap(/*OK*/).change_password(&old, new) {
                    Ok(()) => {
                        self.current_modal = PlModal::None;
                    }
                    Err(e) => {
                        v.pw.error = Some(e.to_string());
                    }
                }
            }

            Action::FinalizeAdd => {
                let pl_file = o_plfile.as_mut().unwrap(/*OK*/);
                let transient = pl_file.transient_mut().unwrap(/*OK*/);
                let (_orig_name, name, bundle) = v.edit_bundle.as_oldname_newname_bundle(transient);
                match pl_file.save_with_added_bundle(name, bundle) {
                    Ok(()) => {
                        self.current_modal = PlModal::None;
                    }
                    Err(e) => {
                        v.edit_bundle.err = Some(e.to_string());
                    }
                }
                v.reset_bundles(pl_file.bundles());
            }

            Action::StartChangeFile => {
                self.current_modal = PlModal::ChangeFile;
                v.file_selection.reset(settings.current_file);
            }
            Action::SwitchToKnownFile(idx) => match settings.set_current_file(idx) {
                Ok(()) => {
                    self.current_modal = PlModal::None;
                    switch_to_current_file(o_plfile, v, settings);
                }
                Err(e) => {
                    v.file_selection.err =
                        Some(format!("Error: {}, caused by {:?}", e, e.source()));
                }
            },
            Action::SwitchToNewFile(path) => {
                match settings.add_and_set_file(&PathBuf::from(path)) {
                    Ok(()) => {
                        self.current_modal = PlModal::None;
                        switch_to_current_file(o_plfile, v, settings);
                    }
                    Err(e) => {
                        v.file_selection.err =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    }
                }
            }

            Action::StartChangeLanguage => {
                v.lang.init(&settings.language);
                self.current_modal = PlModal::ChangeLanguage;
            }
            Action::FinalizeChangeLanguage => match settings.set_language(v.lang.selected.0) {
                Ok(()) => {
                    self.current_modal = PlModal::None;
                }
                Err(e) => {
                    v.lang.err = Some(e.to_string());
                }
            },

            Action::StartModify(index, name) => {
                let pl_file = o_plfile.as_mut().unwrap(/*OK*/);
                let transient = pl_file.transient().unwrap(/*OK*/);
                let bundle = pl_file.bundles().get(&name).unwrap(/*OK*/);
                v.edit_idx = EditIdx::Mod(index);
                v.edit_bundle = VEditBundle {
                    orig_name: name.to_string(),
                    name: name.to_string(),
                    description: bundle.description().to_string(),
                    v_edit_creds: bundle
                        .creds()
                        .iter()
                        .map(|c| VEditCred {
                            name: c.name.disclose(transient).to_string(),
                            secret: c.secret.disclose(transient).to_string(),
                        })
                        .collect(),
                    err: None,
                };
                while v.edit_bundle.v_edit_creds.len() < 4 {
                    v.edit_bundle.v_edit_creds.push(VEditCred::default());
                }
            }
            Action::FinalizeModify => {
                let pl_file = o_plfile.as_mut().unwrap(/*OK*/);
                let (orig_name, name, bundle) = v
                    .edit_bundle
                    .as_oldname_newname_bundle(pl_file.transient_mut().unwrap(/*OK*/));
                if let Err(e) = if v.edit_idx.is_mod() {
                    pl_file.save_with_updated_bundle(&orig_name, name, &bundle)
                } else {
                    Err(anyhow!("save: only mod is expected"))
                } {
                    println!("TODO 'FinalizeModify' failed with {e:?}");
                }
                v.edit_idx = EditIdx::None;
            }

            Action::StartDelete(name) => {
                self.current_modal = PlModal::DeleteBundle;
                v.name_for_delete = name;
            }
            Action::FinalizeDelete(name) => {
                if let Err(e) = o_plfile.as_mut().unwrap(/*OK*/).save_with_deleted_bundle(name) {
                    println!("TODO 'FinalizeDelete' failed with {e:?}");
                }
            }

            Action::Cancel => {
                self.current_modal = PlModal::None;
                v.edit_idx = EditIdx::None;
            }
        }
    }

    pub fn current_modal(&self) -> &PlModal {
        &self.current_modal
    }
}

fn switch_to_current_file(o_plfile: &mut Option<PlFile>, v: &mut V, settings: &mut Settings) {
    let pl_file = PlFile::read_or_create(settings.current_file())
        .context("File open error")
        .unwrap();
    *o_plfile = Some(pl_file);
    *v = V::default();
    v.file_selection.reset(settings.current_file);
}

#[derive(Default, Debug)]
enum Action {
    #[default]
    None,

    StartChangeFile,
    SwitchToKnownFile(usize),
    SwitchToNewFile(String),

    StartChangePassword,
    FinalizeChangePassword {
        old: String,
        new: String,
    },

    StartChangeLanguage,
    FinalizeChangeLanguage,

    FinalizeAdd,

    StartModify(usize, String),
    FinalizeModify,

    StartDelete(String),
    FinalizeDelete(String),
    Cancel,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum PlModal {
    #[default]
    None,
    CreateBundle,
    DeleteBundle,
    About,
    ChangePassword,
    ChangeFile,
    ChangeLanguage,
}
