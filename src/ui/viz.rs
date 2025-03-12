use crate::{
    DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES,
    data::{Bundle, Bundles, Cred, Transient},
};
use jiff::Zoned;
use std::time::Instant;

pub struct V {
    pub pw: Pw,
    pub file_selection: FileSelection,
    pub search: String,
    pub pl_modal: PlModal,
    pub bundles: Vec<VBundle>,
    pub edit_idx: EditIdx,
    pub edit_bundle: VEditBundle,
    pub need_refresh: bool,
    pub lang: Lang,
}
impl V {
    pub fn new() -> Self {
        Self {
            pw: Pw::default(),
            file_selection: FileSelection::default(),
            search: String::default(),
            pl_modal: PlModal::default(),
            bundles: Vec::<VBundle>::default(),
            edit_idx: EditIdx::default(),
            edit_bundle: VEditBundle::default(),
            need_refresh: bool::default(),
            lang: Lang::default(),
        }
    }

    pub fn reset_bundles(&mut self, bundles: &Bundles, transient: &Transient) {
        self.bundles = bundles
            .into_iter()
            .map(|(name, bundle)| VBundle {
                name: name.to_string(),
                description: bundle.description().to_string(),
                v_creds: bundle
                    .creds()
                    .iter()
                    .map(|cred| VCred {
                        name: cred.name(transient),
                        secret: cred.secret(transient),
                        show_secret: false,
                        copied_at: None,
                    })
                    .collect(),
                last_changed: bundle.last_changed_at().clone(),
            })
            .collect();
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum PlModal {
    #[default]
    None,
    CreateBundle,
    DeleteBundle(String),
    About,
    ChangePassword,
    ChangeFile,
    ChangeLanguage,
    ShowPrintable,
}

pub struct Lang {
    pub current: &'static (&'static str, &'static str),
    pub selected: &'static (&'static str, &'static str),
    pub err: Option<String>,
}
impl Default for Lang {
    fn default() -> Self {
        Self {
            current: DEFAULT_LANGUAGE,
            selected: DEFAULT_LANGUAGE,
            err: None,
        }
    }
}
impl Lang {
    pub fn init(&mut self, lang_short: &str) {
        self.current = SUPPORTED_LANGUAGES
            .iter()
            .find(|(short, _long)| lang_short == *short)
            .unwrap_or(DEFAULT_LANGUAGE);
        self.selected = self.current;
        self.err = None;
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditIdx {
    #[default]
    None,
    Mod(usize),
}
impl EditIdx {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
    pub fn is_mod(&self) -> bool {
        matches!(self, Self::Mod(_))
    }
    pub fn is_mod_with(&self, idx: usize) -> bool {
        if let Self::Mod(i) = self {
            *i == idx
        } else {
            false
        }
    }
    pub fn is_mod_not_with(&self, idx: usize) -> bool {
        if let Self::Mod(i) = self {
            *i != idx
        } else {
            false
        }
    }
}
#[derive(Default)]
pub struct Pw {
    pub pw1: String,
    pub pw2: String,
    pub old: String,
    pub error: Option<String>,
    pub focus: PwFocus,
}

#[derive(Default)]
pub enum PwFocus {
    None,
    #[default]
    Pw1,
    Pw2,
    PwOld,
}

#[derive(Default)]
pub struct FileSelection {
    pub err: Option<String>,
    pub current: usize,
    pub new: String,
    pub o_action: Option<FileAction>,
}
impl FileSelection {
    pub fn reset(&mut self, current: usize) {
        self.err = None;
        self.current = current;
        self.new.clear();
        self.o_action = None;
    }
}

pub enum FileAction {
    SwitchToKnown(usize),
    SwitchToNew(String),
}

#[derive(Default)]
pub struct VBundle {
    pub name: String,
    pub description: String,
    pub v_creds: Vec<VCred>,
    pub last_changed: Zoned,
}

#[derive(Clone, Default)]
pub struct VCred {
    pub name: String,
    pub secret: String,
    pub show_secret: bool,
    pub copied_at: Option<Instant>,
}

#[derive(Default)]
pub struct VEditBundle {
    pub orig_name: String,
    pub name: String,
    pub description: String,
    pub v_creds: Vec<VCred>,
    pub err: Option<String>,
}

impl VEditBundle {
    pub fn as_oldname_newname_bundle(&self) -> (String, String, Bundle) {
        (
            self.orig_name.to_string(),
            self.name.to_string(),
            Bundle::new(
                self.description.clone(),
                self.v_creds
                    .iter()
                    .filter_map(|vns| {
                        if vns.name.trim().is_empty() && vns.secret.trim().is_empty() {
                            None
                        } else {
                            Some(Cred::new(vns.name.clone(), vns.secret.clone()))
                        }
                    })
                    .collect(),
            ),
        )
    }

    pub fn prepare_for_create(&mut self) {
        *self = Self::default();
        self.v_creds.push(VCred::default());
        self.v_creds.push(VCred::default());
        self.v_creds.push(VCred::default());
        self.v_creds.push(VCred::default());
    }
}
