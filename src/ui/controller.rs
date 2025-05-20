use crate::{
    PlFile, Settings,
    ui::viz::{MainState, ModalState, Pw, V, VEditBundle, VEditDocument},
};
use anyhow::Context;
use std::path::PathBuf;

use super::viz::{BundleState, DocumentState};

// The controller is responsible for managing the state of the application and the UI,
// and is the only place where the application data is modified.
// The UI code calls Controller::set_action() to set the next action to be taken.
// The main loop calls Controller::act() to execute the action.
#[derive(Default)]
pub struct Controller {
    // The next action to be taken by the controller.
    next_action: Action,
}
impl Controller {
    pub fn set_action(&mut self, action: Action) {
        self.next_action = action;
    }

    #[allow(clippy::too_many_lines)]
    pub fn act(&mut self, pl_file: &mut PlFile, v: &mut V, settings: &mut Settings) {
        let action = std::mem::take(&mut self.next_action);
        action.log();
        match (&mut v.main_state, &mut v.modal_state, action) {
            (_, ModalState::None, Action::None) => {}

            (_, ModalState::None, Action::StartChangeFile) => {
                v.file_selection.reset(settings.current_file);
                v.modal_state = ModalState::ChangeFile;
            }
            (_, ModalState::ChangeFile, Action::SwitchToKnownFile(idx)) => {
                match settings.set_current_file(idx) {
                    Ok(()) => {
                        switch_to_current_file(pl_file, v, settings);
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        v.file_selection.error =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    }
                }
            }
            (_, ModalState::ChangeFile, Action::SwitchToNewFile(path)) => {
                match settings.add_and_set_file(&PathBuf::from(path)) {
                    Ok(()) => {
                        switch_to_current_file(pl_file, v, settings);
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        v.file_selection.error =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    }
                }
            }

            (_, ModalState::None, Action::ShowAbout) => {
                v.modal_state = ModalState::About;
            }

            (_, ModalState::None, Action::ShowLog) => {
                v.show_log = true;
            }

            (_, ModalState::None, Action::StartChangeLanguage) => {
                v.lang.init(&settings.language);
                v.modal_state = ModalState::ChangeLanguage;
            }
            (_, ModalState::ChangeLanguage, Action::FinalizeChangeLanguage) => {
                match settings.set_language(v.lang.selected.0) {
                    Ok(()) => {
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        let s = e.to_string();
                        log::error!("{s}");
                        v.lang.error = Some(s);
                    }
                }
            }

            (_, ModalState::None, Action::SwitchToActionable) => {
                match pl_file.set_actionable(v.pw.pw1.clone()) {
                    Ok(()) => {
                        v.pw.error = None;
                        v.reset_bundles(pl_file.bundles(), None);
                        v.reset_documents(pl_file.documents(), None);
                        // FIXME if pl_file.is_empty() {
                        //     v.edit_b.bundle.prepare_for_create();
                        // }
                        v.find.request_focus = true;
                    }
                    Err(e) => {
                        let s = e.to_string();
                        log::error!("{s}");
                        v.pw.error = Some(s);
                    }
                }
            }

            (MainState::Bundles(_), ModalState::None, Action::StartFilter) => {
                v.apply_filter_to_bundles(pl_file.bundles());
            }
            (MainState::Documents(_), ModalState::None, Action::StartFilter) => {
                v.apply_filter_to_documents(pl_file.documents());
            }

            (_, ModalState::None, Action::StartChangePassword) => {
                v.pw = Pw::default();
                v.modal_state = ModalState::ChangePassword;
            }
            (_, ModalState::ChangePassword, Action::FinalizeChangePassword { old, new }) => {
                match pl_file.change_password(&old, new) {
                    Ok(()) => {
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        let s = e.to_string();
                        log::error!("{s}");
                        v.pw.error = Some(s);
                    }
                }
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::None,
                Action::StartAddBundle,
            ) => {
                v.modal_state = ModalState::AddBundle {
                    v_edit_bundle: VEditBundle::new(),
                    error: None,
                };
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::AddBundle {
                    v_edit_bundle,
                    error,
                },
                Action::FinalizeAddBundle,
            ) => match pl_file.save_with_added_bundle(v_edit_bundle) {
                Ok(()) => {
                    let name = v_edit_bundle.name.clone();
                    v.reset_bundles(pl_file.bundles(), Some(&name));
                    v.modal_state.close_modal();
                }
                Err(e) => {
                    let s = e.to_string();
                    log::error!("{s}");
                    *error = Some(s);
                }
            },

            (
                MainState::Bundles(BundleState::Default),
                ModalState::None,
                Action::StartModifyBundle(index, name),
            ) => {
                v.modal_state = ModalState::None;
                v.main_state = MainState::Bundles(BundleState::ModifyBundle {
                    idx: index,
                    v_edit_bundle: VEditBundle::from_bundle(
                        &name,
                        pl_file.bundles().get(&name).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            }
            (
                MainState::Bundles(BundleState::ModifyBundle {
                    idx: _,
                    v_edit_bundle: bundle,
                    error,
                }),
                ModalState::None,
                Action::FinalizeModifyBundle,
            ) => {
                match pl_file.save_with_updated_bundle(bundle) {
                    Ok(()) => {
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        let s = e.to_string();
                        log::error!("{s}");
                        *error = Some(s);
                    }
                }

                v.reset_bundles(pl_file.bundles(), None);
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::None,
                Action::StartDeleteBundle(name),
            ) => {
                v.modal_state = ModalState::DeleteBundle {
                    name: name.clone(),
                    error: None,
                };
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::DeleteBundle { name, error },
                Action::FinalizeDeleteBundle,
            ) => match pl_file.save_with_deleted_bundle((*name).clone()) {
                Ok(()) => {
                    v.reset_bundles(pl_file.bundles(), None);
                    v.modal_state.close_modal();
                }
                Err(e) => {
                    let s = e.to_string();
                    log::error!("{s}");
                    *error = Some(s);
                }
            },

            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::None,
                Action::StartAddDocument,
            ) => {
                v.modal_state = ModalState::AddDocument {
                    v_edit_document: VEditDocument::new(),
                    error: None,
                };
            }

            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::AddDocument {
                    v_edit_document,
                    error,
                },
                Action::FinalizeAddDocument(name),
            ) => match pl_file.save_with_added_document(v_edit_document) {
                Ok(()) => {
                    v.modal_state.close_modal();
                    v.reset_documents(pl_file.documents(), Some(&name));
                }
                Err(e) => {
                    let s = e.to_string();
                    log::error!("{s}");
                    *error = Some(s);
                }
            },

            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::None,
                Action::StartModifyDocument(index, name),
            ) => {
                v.main_state = MainState::Documents(DocumentState::ModifyDocument {
                    idx: index,
                    v_edit_document: VEditDocument::from_document(
                        &name,
                        pl_file.documents().get(&name).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            }

            (
                MainState::Documents(DocumentState::ModifyDocument {
                    idx,
                    v_edit_document,
                    error,
                }),
                ModalState::None,
                Action::FinalizeModifyDocument,
            ) => {
                match pl_file.save_with_updated_document(v_edit_document) {
                    Ok(()) => {
                        v.modal_state.close_modal();
                    }
                    Err(e) => {
                        let s = e.to_string();
                        log::error!("{s}");
                        *error = Some(s);
                    }
                }

                let name = v_edit_document.name.clone();
                v.main_state = MainState::Documents(DocumentState::Default(Some((*idx, name))));
                v.reset_documents(pl_file.documents(), None);
            }

            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::None,
                Action::StartDeleteDocument(name),
            ) => {
                v.modal_state = ModalState::DeleteDocument {
                    name: name.clone(),
                    error: None,
                };
            }
            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::DeleteDocument { name, error },
                Action::FinalizeDeleteDocument,
            ) => match pl_file.save_with_deleted_document(name.clone()) {
                Ok(()) => {
                    v.reset_documents(pl_file.documents(), None);
                    v.modal_state.close_modal();
                }
                Err(e) => {
                    let s = e.to_string();
                    log::error!("{s}");
                    *error = Some(s);
                }
            },

            (_, _, Action::Cancel | Action::SilentCancel) => {
                v.modal_state.close_modal();
                v.main_state = match v.main_state {
                    MainState::Bundles(_) => MainState::Bundles(BundleState::Default),
                    MainState::Documents(_) => MainState::Documents(DocumentState::Default(None)),
                };
            }

            (_main_state, modal_state, action) => {
                if !matches!(modal_state, ModalState::None) && !matches!(action, Action::None) {
                    log::warn!(
                        "Unexpected situation: {}, action = {action:?}",
                        modal_state.get_id()
                    );
                }
            }
        }
    }
}

fn switch_to_current_file(pl_file: &mut PlFile, v: &mut V, settings: &mut Settings) {
    *pl_file = PlFile::read_or_create(settings.current_file())
        .context("File open error")
        .unwrap(/* FIXME */);
    log::info!("{} {}", t!("Switch to file"), pl_file.file_path());
    *v = V::default();
    v.file_selection.reset(settings.current_file);
}

#[derive(Default, Debug)]
pub(crate) enum Action {
    #[default]
    None,

    ShowAbout,
    ShowLog,

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

    StartAddBundle,
    FinalizeAddBundle,

    StartModifyBundle(usize, String),
    FinalizeModifyBundle,

    StartDeleteBundle(String),
    FinalizeDeleteBundle,

    StartAddDocument,
    FinalizeAddDocument(String),

    StartModifyDocument(usize, String),
    FinalizeModifyDocument,

    StartDeleteDocument(String),
    FinalizeDeleteDocument,

    Cancel,
    SilentCancel,
}
impl Action {
    fn log(&self) {
        match self {
            Action::None | Action::StartFilter | Action::ShowLog | Action::SilentCancel => {}

            Action::ShowAbout
            | Action::StartChangeFile
            | Action::SwitchToKnownFile(_)
            | Action::SwitchToNewFile(_)
            | Action::StartChangePassword
            | Action::SwitchToActionable
            | Action::StartChangeLanguage
            | Action::FinalizeChangeLanguage
            | Action::StartAddBundle
            | Action::FinalizeAddBundle
            | Action::StartModifyBundle(_, _)
            | Action::FinalizeModifyBundle
            | Action::StartDeleteBundle(_)
            | Action::FinalizeDeleteBundle
            | Action::StartAddDocument
            | Action::FinalizeAddDocument(_)
            | Action::StartModifyDocument(_, _)
            | Action::FinalizeModifyDocument
            | Action::StartDeleteDocument(_)
            | Action::FinalizeDeleteDocument
            | Action::Cancel
            | Action::FinalizeChangePassword { old: _, new: _ } => {
                log::info!("[Action::{self:?}]");
            }
        }
    }
}
