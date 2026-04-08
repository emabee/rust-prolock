use crate::{
    PlFile, Settings,
    data::Key,
    ui::{
        Action,
        viz::{
            BundleState, DocumentState, ImportAction, ImportStartCondition, ImportStep, MainState,
            ModalState, Pw, PwFocus, V, VEditBundle, VEditDocument,
        },
    },
    util::generate_password,
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
        let action_s = format!("{action:?}");

        let done = if v.modal_state.is_none() {
            act_on_no_modal(pl_file, v, settings, action)
        } else {
            act_on_modal(pl_file, v, settings, action)
        };

        if !done {
            log::warn!(
                "Unhandled situation: {:?}, {}, action = {action_s:?}",
                &v.main_state,
                v.modal_state.get_id(),
            );
        }
    }
}

#[allow(clippy::too_many_lines)]
fn act_on_no_modal(
    pl_file: &mut PlFile,
    v: &mut V,
    settings: &mut Settings,
    action: Action,
) -> bool {
    match action {
        Action::None => {}

        Action::CloseModal => {
            v.modal_state.close_modal();
        }

        Action::Cancel => {
            v.modal_state.close_modal();
            v.main_state = match v.main_state {
                MainState::Bundles(_) => MainState::Bundles(BundleState::Default),
                MainState::Documents(DocumentState::ModifyDocument {
                    ref v_edit_document,
                    ..
                }) => {
                    MainState::Documents(DocumentState::Default(Some(v_edit_document.key.clone())))
                }
                MainState::Documents(DocumentState::Default(_)) => {
                    MainState::Documents(DocumentState::Default(None))
                }
            };
        }

        ///////////////////////////////////////////
        Action::FinalizeModifyBundle => {
            if let MainState::Bundles(BundleState::ModifyBundle {
                v_edit_bundle: bundle,
                error,
            }) = &mut v.main_state
            {
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
            } else {
                return false;
            }
        }

        Action::FinalizeModifyDocument => {
            if let MainState::Documents(DocumentState::ModifyDocument {
                v_edit_document,
                error,
            }) = &mut v.main_state
            {
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
            } else {
                return false;
            }
        }

        Action::ShowAbout => {
            v.modal_state = ModalState::About;
        }

        Action::ShowLog => {
            v.show_log = true;
        }

        Action::StartAddBundle => {
            if let MainState::Bundles(BundleState::Default) = &mut v.main_state {
                v.modal_state = ModalState::AddBundle {
                    v_edit_bundle: VEditBundle::new(),
                    generate_pw: false,
                    error: None,
                };
            } else {
                return false;
            }
        }

        Action::StartAddDocument => {
            if let MainState::Documents(DocumentState::Default(_)) = &mut v.main_state {
                v.modal_state = ModalState::AddDocument {
                    v_edit_document: VEditDocument::new(),
                    error: None,
                };
            } else {
                return false;
            }
        }

        Action::StartChangeFile => {
            v.file_selection.reset(settings.current_file);
            v.modal_state = ModalState::ChangeFile;
        }

        Action::StartChangeLanguage => {
            v.lang.init(&settings.language);
            v.modal_state = ModalState::ChangeLanguage;
        }

        Action::StartChangePassword => {
            v.pw = Pw::default();
            v.modal_state = ModalState::ChangePassword;
        }

        Action::StartDeleteBundle(key) => {
            if let MainState::Bundles(BundleState::Default) = &mut v.main_state {
                v.modal_state = ModalState::DeleteBundle {
                    key: key.clone(),
                    error: None,
                };
            } else {
                return false;
            }
        }

        Action::StartDeleteDocument(key) => {
            if let MainState::Documents(DocumentState::Default(_)) = &mut v.main_state {
                v.modal_state = ModalState::DeleteDocument { key, error: None };
            } else {
                return false;
            }
        }

        Action::StartExportData => {
            v.modal_state = ModalState::ExportData;
            v.export_data.reset(pl_file.bundles());
        }

        Action::StartImportData => {
            v.modal_state = ModalState::ImportData {
                step: ImportStep::default(),
                error: None,
            };
        }

        Action::StartFilter => match &mut v.main_state {
            MainState::Bundles(_) => {
                v.apply_filter_to_bundles(pl_file.bundles());
            }
            MainState::Documents(_) => {
                v.apply_filter_to_documents(pl_file.documents());
            }
        },

        Action::StartGeneratePassword(o_cred) => {
            if let MainState::Bundles(BundleState::ModifyBundle { .. }) = &mut v.main_state {
                v.modal_state = ModalState::GeneratePassword;
                v.generate_pw.cred_idx = o_cred;
            } else {
                return false;
            }
        }

        Action::StartModifyBundle(key) => {
            if let MainState::Bundles(BundleState::Default) = &mut v.main_state {
                v.modal_state = ModalState::None;
                v.main_state = MainState::Bundles(BundleState::ModifyBundle {
                    v_edit_bundle: VEditBundle::from_bundle(
                        &key,
                        pl_file.bundles().get(&key).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            } else {
                return false;
            }
        }

        Action::StartModifyDocument(key) => {
            if let MainState::Documents(DocumentState::Default(_)) = &mut v.main_state {
                v.main_state = MainState::Documents(DocumentState::ModifyDocument {
                    v_edit_document: VEditDocument::from_document(
                        &key,
                        pl_file.documents().get(&key).unwrap(/*OK*/),
                        pl_file.transient().unwrap(/*OK*/),
                    ),
                    error: None,
                });
            } else {
                return false;
            }
        }

        Action::SwitchToActionable => {
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

        _ => {
            return false;
        }
    }
    true
}

#[allow(clippy::too_many_lines)]
fn act_on_modal(pl_file: &mut PlFile, v: &mut V, settings: &mut Settings, action: Action) -> bool {
    match (action, &mut v.modal_state, &mut v.main_state) {
        (_, ModalState::None, _) => {
            unreachable!("act_on_modal called with ModalState::None");
        }

        (
            Action::FinalizeAddBundle,
            ModalState::AddBundle {
                v_edit_bundle,
                generate_pw: false,
                error,
            },
            MainState::Bundles(BundleState::Default),
        ) => match pl_file.save_with_added_bundle(v_edit_bundle) {
            Ok(()) => {
                let key = v_edit_bundle.key.clone();
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
            Action::FinalizeAddDocument,
            ModalState::AddDocument {
                v_edit_document,
                error,
            },
            MainState::Documents(DocumentState::Default(_)),
        ) => match pl_file.save_with_added_document(v_edit_document) {
            Ok(()) => {
                let key = v_edit_document.key.clone();
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

        (Action::FinalizeChangeLanguage, ModalState::ChangeLanguage, _) => {
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

        (Action::FinalizeChangePassword { old, new }, ModalState::ChangePassword, _) => {
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
            Action::FinalizeDeleteBundle,
            ModalState::DeleteBundle { key, error },
            MainState::Bundles(BundleState::Default),
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
            Action::FinalizeDeleteDocument,
            ModalState::DeleteDocument { key, error },
            MainState::Documents(DocumentState::Default(_)),
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

        (Action::FinalizeExportData, ModalState::ExportData, _main_state) => {
            match pl_file.export_data(&mut v.export_data) {
                Ok(()) => {
                    v.modal_state.close_modal();
                }
                Err(e) => {
                    v.export_data.pw.error = Some(match e.source() {
                        Some(source) => t!(
                            "%{error}, caused by %{source}",
                            error = e,
                            source = format!("{}", source)
                        )
                        .to_string(),
                        None => t!("Error: %{error}", error = e).to_string(),
                    });
                }
            }
        }

        (Action::FinalizeImportData, ModalState::ImportData { step, error }, _main_state) => {
            match step {
                ImportStep::AskForFileAndPassword { file_path, pw } => {
                    match PlFile::read(&PathBuf::from(&file_path)) {
                        Err(e) => {
                            *error = Some(format!("{file_path}\n{e:?}"));
                            return false;
                        }
                        Ok(mut file) => {
                            if let Err(e) = file.set_actionable(pw.clone()) {
                                *error = Some(format!("{e:?}"));
                                return false;
                            }
                            let start_conditions: Vec<ImportStartCondition> = file
                                .bundles()
                                .iter()
                                .map(|(key, bundle)| match pl_file.bundles().get(key) {
                                    None => ImportStartCondition::New,
                                    Some(old_bundle) => {
                                        if bundle.equals(
                                            old_bundle,
                                            file.transient().unwrap(/*OK*/),
                                            pl_file.transient().unwrap(/*OK*/),
                                        ) {
                                            ImportStartCondition::Identical
                                        } else {
                                            ImportStartCondition::Modified
                                        }
                                    }
                                })
                                .collect();

                            let actions = start_conditions
                                .iter()
                                .map(|cond| match cond {
                                    ImportStartCondition::New => ImportAction::Add,
                                    ImportStartCondition::Identical => ImportAction::Skip,
                                    ImportStartCondition::Modified => ImportAction::Overwrite,
                                })
                                .collect();

                            // switch to next step
                            *step = ImportStep::ChooseImportActions {
                                file: Box::new(file),
                                start_conditions,
                                actions,
                            }
                        }
                    }
                }
                ImportStep::ChooseImportActions {
                    file,
                    start_conditions: _,
                    actions,
                } => {
                    if let Err(e) = execute_import(pl_file, file, actions) {
                        v.import_data.error =
                            Some(format!("Error: {}, caused by {:?}", e, e.source()));
                    } else {
                        v.reset_bundles(pl_file.bundles(), None);
                        v.modal_state.close_modal();
                    }
                }
            }
        }

        (
            Action::FinalizeGeneratePassword,
            ModalState::GeneratePassword,
            MainState::Bundles(BundleState::ModifyBundle { v_edit_bundle, .. }),
        ) => {
            let pw = generate_password(&v.generate_pw);
            if !pw.is_empty() {
                v_edit_bundle.v_edit_creds[v.generate_pw.cred_idx]
                    .secret
                    .clone_from(&pw);
                v.modal_state.close_modal();
            }
        }

        (
            Action::FinalizeGeneratePassword,
            ModalState::AddBundle {
                generate_pw,
                v_edit_bundle,
                ..
            },
            MainState::Bundles(BundleState::Default),
        ) => {
            let pw = generate_password(&v.generate_pw);
            if !pw.is_empty() {
                *generate_pw = false;
                v_edit_bundle.v_edit_creds[v.generate_pw.cred_idx]
                    .secret
                    .clone_from(&pw);
            }
        }

        (
            Action::StartGeneratePassword(o_cred),
            ModalState::AddBundle { generate_pw, .. },
            MainState::Bundles(BundleState::Default),
        ) => {
            *generate_pw = true;
            v.generate_pw.cred_idx = o_cred;
        }

        (Action::SwitchToKnownFile(idx), ModalState::ChangeFile, _) => {
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
        (Action::SwitchToNewFile(path), ModalState::ChangeFile, _) => {
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

        (Action::CloseModal, _, _) => {
            v.modal_state.close_modal();
        }

        (Action::Cancel, _, _) => {
            v.modal_state.close_modal();
            v.main_state = match v.main_state {
                MainState::Bundles(_) => MainState::Bundles(BundleState::Default),
                MainState::Documents(DocumentState::ModifyDocument {
                    ref v_edit_document,
                    ..
                }) => {
                    MainState::Documents(DocumentState::Default(Some(v_edit_document.key.clone())))
                }
                MainState::Documents(DocumentState::Default(_)) => {
                    MainState::Documents(DocumentState::Default(None))
                }
            };
        }

        (action, _, _) => {
            if !matches!(action, Action::None) {
                return false;
            }
        }
    }
    true
}

fn execute_import(
    pl_file: &mut PlFile,
    file: &mut PlFile,
    actions: &mut Vec<ImportAction>,
) -> Result<()> {
    for ((key, bundle), action) in file.bundles().iter().zip(actions) {
        // convert bundle into a VEditBundle, to get rid of reffs (which would not be valid in the new pl_file)
        let v_edit_bundle = VEditBundle::from_bundle(key, bundle, file.transient().unwrap(/*Ok*/));
        match action {
            ImportAction::Skip => {}
            ImportAction::Add => {
                // then update pl_file
                pl_file.save_with_added_bundle(&v_edit_bundle)?;
            }
            ImportAction::Overwrite => {
                pl_file.save_with_updated_bundle(&v_edit_bundle)?;
            }
            ImportAction::ImportAfterRename => {
                for i in 1.. {
                    if pl_file
                        .rename_bundle(key, &Key::new([key.0.clone(), (-i).to_string()].concat()))?
                    {
                        break;
                    }
                }
                pl_file.save_with_added_bundle(&v_edit_bundle)?;
            }
        }
    }
    Ok(())
}

fn switch_to_current_file(pl_file: &mut PlFile, v: &mut V, settings: &mut Settings) -> Result<()> {
    *pl_file = PlFile::read_or_create(settings.current_file()).context("File open error")?;
    log::info!("{} {}", t!("Switch to file"), pl_file.file_path());
    *v = V::default();
    v.file_selection.reset(settings.current_file);
    Ok(())
}
