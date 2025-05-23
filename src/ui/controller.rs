use crate::{
    PlFile, Settings,
    data::Key,
    ui::viz::{
        BundleState, DocumentState, MainState, ModalState, Pw, PwFocus, V, VEditBundle,
        VEditDocument,
    },
};
use anyhow::{Context, Result};
use std::path::PathBuf;

// The controller is responsible for managing the state of the application and the UI,
// and is the only place where the application data is modified.
// The UI code calls Controller::set_action() to set the next action to be taken.
// The main loop calls Controller::act() to execute the action.
#[derive(Default)]
pub struct Controller {
    next_action: Action,
}
impl Controller {
    // Set the next action to be taken by the controller.
    pub fn set_action(&mut self, action: Action) {
        self.next_action = action;
    }

    // Executes the action set by the UI code.
    #[allow(clippy::too_many_lines)]
    pub fn act(&mut self, pl_file: &mut PlFile, v: &mut V, settings: &mut Settings) {
        let action = std::mem::take(&mut self.next_action);
        action.log(&v.main_state, &v.modal_state);

        match (&mut v.main_state, &mut v.modal_state, action) {
            (_, ModalState::None, Action::None) => {}

            (_, ModalState::None, Action::StartChangeFile) => {
                v.file_selection.reset(settings.current_file);
                v.modal_state = ModalState::ChangeFile;
            }
            (_, ModalState::ChangeFile, Action::SwitchToKnownFile(idx)) => {
                match settings.set_current_file(idx) {
                    Ok(()) => match switch_to_current_file(pl_file, v, settings) {
                        Ok(()) => {
                            v.modal_state.close_modal();
                        }
                        Err(e) => {
                            v.file_selection.error =
                                Some(format!("Error: {}, caused by {:?}", e, e.source()));
                        }
                    },
                    Err(e) => {
                        v.file_selection.error =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    }
                }
            }
            (_, ModalState::ChangeFile, Action::SwitchToNewFile(path)) => {
                match settings.add_and_set_file(&PathBuf::from(path)) {
                    Ok(()) => match switch_to_current_file(pl_file, v, settings) {
                        Ok(()) => {
                            v.modal_state.close_modal();
                        }
                        Err(e) => {
                            v.file_selection.error =
                                Some(format!("Error: {}, caused by {:?}", e, e.source()));
                        }
                    },
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
                        // TODO if pl_file.is_empty() {
                        //     v.edit_b.bundle.prepare_for_create();
                        // }
                        v.find.request_focus = true;
                    }
                    Err(e) => {
                        // TODO mark all entered text to facilitate repetition
                        v.pw.focus = PwFocus::Pw1;
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
                Action::FinalizeAddBundle(key),
            ) => match pl_file.save_with_added_bundle(v_edit_bundle) {
                Ok(()) => {
                    v.modal_state.close_modal();
                    v.reset_bundles(pl_file.bundles(), Some(&key));
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
                Action::StartModifyBundle(key),
            ) => {
                v.modal_state = ModalState::None;
                v.main_state = MainState::Bundles(BundleState::ModifyBundle {
                    v_edit_bundle: VEditBundle::from_bundle(
                        &key,
                        pl_file.bundles().get(&key).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            }
            (
                MainState::Bundles(BundleState::ModifyBundle {
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
                v.main_state = MainState::Bundles(BundleState::Default);
                v.modal_state = ModalState::None;
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::None,
                Action::StartDeleteBundle(key),
            ) => {
                v.modal_state = ModalState::DeleteBundle {
                    key: key.clone(),
                    error: None,
                };
            }

            (
                MainState::Bundles(BundleState::Default),
                ModalState::DeleteBundle { key, error },
                Action::FinalizeDeleteBundle,
            ) => match pl_file.save_with_deleted_bundle(key.clone()) {
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
                Action::FinalizeAddDocument(key),
            ) => match pl_file.save_with_added_document(v_edit_document) {
                Ok(()) => {
                    v.modal_state.close_modal();
                    v.reset_documents(pl_file.documents(), Some(&key));
                    v.main_state = MainState::Documents(DocumentState::Default(Some(key)));
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
                Action::StartModifyDocument(key),
            ) => {
                v.main_state = MainState::Documents(DocumentState::ModifyDocument {
                    v_edit_document: VEditDocument::from_document(
                        &key,
                        pl_file.documents().get(&key).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            }

            (
                MainState::Documents(DocumentState::ModifyDocument {
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

                v.main_state =
                    MainState::Documents(DocumentState::Default(Some(v_edit_document.key.clone())));
                v.reset_documents(pl_file.documents(), None);
            }

            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::None,
                Action::StartDeleteDocument(key),
            ) => {
                v.modal_state = ModalState::DeleteDocument { key, error: None };
            }
            (
                MainState::Documents(DocumentState::Default(_)),
                ModalState::DeleteDocument { key, error },
                Action::FinalizeDeleteDocument,
            ) => match pl_file.save_with_deleted_document(key) {
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
                    MainState::Documents(DocumentState::ModifyDocument {
                        ref v_edit_document,
                        ..
                    }) => MainState::Documents(DocumentState::Default(Some(
                        v_edit_document.key.clone(),
                    ))),
                    MainState::Documents(DocumentState::Default(_)) => {
                        MainState::Documents(DocumentState::Default(None))
                    }
                };
            }

            (main_state, modal_state, action) => {
                if !matches!(modal_state, ModalState::None) && !matches!(action, Action::None) {
                    log::warn!(
                        "Unhandled situation: {main_state:?}, {}, action = {action:?}",
                        modal_state.get_id()
                    );
                }
            }
        }
    }
}

fn switch_to_current_file(pl_file: &mut PlFile, v: &mut V, settings: &mut Settings) -> Result<()> {
    *pl_file = PlFile::read_or_create(settings.current_file()).context("File open error")?;
    log::info!("{} {}", t!("Switch to file"), pl_file.file_path());
    *v = V::default();
    v.file_selection.reset(settings.current_file);
    Ok(())
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
    FinalizeAddBundle(Key),

    StartModifyBundle(Key),
    FinalizeModifyBundle,

    StartDeleteBundle(Key),
    FinalizeDeleteBundle,

    StartAddDocument,
    FinalizeAddDocument(Key),

    StartModifyDocument(Key),
    FinalizeModifyDocument,

    StartDeleteDocument(Key),
    FinalizeDeleteDocument,

    Cancel,
    SilentCancel,
}
impl Action {
    fn log(&self, main_state: &MainState, modal_state: &ModalState) {
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
            | Action::FinalizeAddBundle(_)
            | Action::StartModifyBundle(_)
            | Action::FinalizeModifyBundle
            | Action::StartDeleteBundle(_)
            | Action::FinalizeDeleteBundle
            | Action::StartAddDocument
            | Action::FinalizeAddDocument(_)
            | Action::StartModifyDocument(_)
            | Action::FinalizeModifyDocument
            | Action::StartDeleteDocument(_)
            | Action::FinalizeDeleteDocument
            | Action::Cancel
            | Action::FinalizeChangePassword { .. } => {
                log::info!("[Action::{self:?}] [{main_state:?}] [{modal_state:?}]");
            }
        }
    }
}
