use crate::{
    data::Key,
    ui::viz::{MainState, ModalState},
};

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

    StartModifyBundle(Key),
    FinalizeModifyBundle,

    StartDeleteBundle(Key),
    FinalizeDeleteBundle,

    StartAddDocument,
    FinalizeAddDocument,

    StartModifyDocument(Key),
    FinalizeModifyDocument,

    StartDeleteDocument(Key),
    FinalizeDeleteDocument,

    StartGeneratePassword(usize),
    FinalizeGeneratePassword,

    Cancel,
    CloseModal,
}

impl Action {
    pub(crate) fn log(&self, main_state: &MainState, modal_state: &ModalState) {
        match self {
            Action::None | Action::StartFilter | Action::ShowLog | Action::CloseModal => {}

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
            | Action::StartModifyBundle(_)
            | Action::FinalizeModifyBundle
            | Action::StartDeleteBundle(_)
            | Action::FinalizeDeleteBundle
            | Action::StartAddDocument
            | Action::FinalizeAddDocument
            | Action::StartModifyDocument(_)
            | Action::FinalizeModifyDocument
            | Action::StartDeleteDocument(_)
            | Action::FinalizeDeleteDocument
            | Action::StartGeneratePassword(_)
            | Action::FinalizeGeneratePassword
            | Action::Cancel
            | Action::FinalizeChangePassword { .. } => {
                log::info!("[Action::{self:?}] [{main_state:?}] [{modal_state:?}]");
            }
        }
    }
}
