use crate::{
    PlFile, Settings,
    ui::viz::{Pw, V, VEditBundle, VEditCred},
};
use anyhow::Context;
use std::path::PathBuf;

// Implements the controller for the application, i.e., the logic that decides what to do next.
// The controller is the only place where the application data is modified.
#[derive(Default)]
pub struct Controller {
    // The next action to be taken by the controller.
    next_action: Action,
    // UI to be shown to the user.
    current_modal: PlModal,
}
impl Controller {
    // this is called in many places of the UI code
    pub fn set_action(&mut self, action: Action) {
        self.next_action = action;
    }

    // this is called in the main loop
    #[allow(clippy::too_many_lines)]
    pub fn act(&mut self, o_plfile: &mut Option<PlFile>, v: &mut V, settings: &mut Settings) {
        match (std::mem::take(&mut self.next_action), o_plfile) {
            (Action::None, _) => {}

            (Action::StartChangeFile, _) => {
                v.file_selection.reset(settings.current_file);
                self.current_modal = PlModal::ChangeFile;
            }
            (Action::SwitchToKnownFile(idx), o_plfile) => match settings.set_current_file(idx) {
                Ok(()) => {
                    switch_to_current_file(o_plfile, v, settings);
                    self.current_modal = PlModal::None;
                }
                Err(e) => {
                    v.file_selection.err =
                        Some(format!("Error: {}, caused by {:?}", e, e.source()));
                }
            },
            (Action::SwitchToNewFile(path), o_plfile) => {
                match settings.add_and_set_file(&PathBuf::from(path)) {
                    Ok(()) => {
                        switch_to_current_file(o_plfile, v, settings);
                        self.current_modal = PlModal::None;
                    }
                    Err(e) => {
                        v.file_selection.err =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    }
                }
            }

            (Action::ShowAbout, _) => {
                self.current_modal = PlModal::About;
            }

            (Action::StartChangeLanguage, _) => {
                v.lang.init(&settings.language);
                self.current_modal = PlModal::ChangeLanguage;
            }
            (Action::FinalizeChangeLanguage, _) => match settings.set_language(v.lang.selected.0) {
                Ok(()) => {
                    self.current_modal = PlModal::None;
                }
                Err(e) => {
                    v.lang.err = Some(e.to_string());
                }
            },

            (Action::SwitchToActionable, Some(pl_file)) => {
                match pl_file.set_actionable(v.pw.pw1.clone()) {
                    Ok(()) => {
                        v.pw.error = None;
                        v.reset_bundles(pl_file.bundles());
                        if pl_file.is_empty() {
                            v.edit_bundle.prepare_for_create();
                        }
                        v.find_request_focus = true;
                    }
                    Err(e) => {
                        v.pw.error = Some(format!("{e}"));
                    }
                }
            }

            (Action::StartFilter, Some(pl_file)) => {
                v.apply_filter(pl_file.bundles());
            }

            (Action::StartChangePassword, _) => {
                v.pw = Pw::default();
                self.current_modal = PlModal::ChangePassword;
            }
            (Action::FinalizeChangePassword { old, new }, Some(pl_file)) => {
                match pl_file.change_password(&old, new) {
                    Ok(()) => {
                        self.current_modal = PlModal::None;
                    }
                    Err(e) => {
                        v.pw.error = Some(e.to_string());
                    }
                }
            }

            (Action::StartAdd, _) => {
                v.edit_bundle.prepare_for_create();
                self.current_modal = PlModal::CreateBundle;
            }
            (Action::FinalizeAdd, Some(pl_file)) => {
                let (_orig_name, name, bundle) = v
                    .edit_bundle
                    .as_oldname_newname_bundle(pl_file.transient_mut().unwrap(/*OK*/));
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

            (Action::StartModify(index, name), Some(pl_file)) => {
                let transient = pl_file.transient().unwrap(/*OK*/);
                let bundle = pl_file.bundles().get(&name).unwrap(/*OK*/);
                v.edit_idx = Some(index);
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
            (Action::FinalizeModify, Some(pl_file)) => {
                let (orig_name, name, bundle) = v
                    .edit_bundle
                    .as_oldname_newname_bundle(pl_file.transient_mut().unwrap(/*OK*/));
                if let Err(e) = pl_file.save_with_updated_bundle(&orig_name, name, &bundle) {
                    println!("TODO 'FinalizeModify' failed with {e:?}");
                }
                v.edit_idx = None;
                // TODO replace with a minimal action:
                v.reset_bundles(pl_file.bundles());
            }

            (Action::StartDelete(name), _) => {
                self.current_modal = PlModal::DeleteBundle;
                v.name_for_delete = name;
            }
            (Action::FinalizeDelete(name), Some(pl_file)) => {
                if let Err(e) = pl_file.save_with_deleted_bundle(name) {
                    println!("TODO 'FinalizeDelete' failed with {e:?}");
                }
                self.current_modal = PlModal::None;
            }

            (Action::Cancel, _) => {
                self.current_modal = PlModal::None;
                v.edit_idx = None;
            }

            (action, &mut None) => {
                unreachable!("Unexpected action {action:?} with no file open");
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
pub(crate) enum Action {
    #[default]
    None,

    ShowAbout,

    StartChangeFile,
    SwitchToKnownFile(usize),
    SwitchToNewFile(String),

    StartChangePassword,
    FinalizeChangePassword {
        old: String,
        new: String,
    },

    SwitchToActionable,

    StartFilter,

    StartChangeLanguage,
    FinalizeChangeLanguage,

    StartAdd,
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
