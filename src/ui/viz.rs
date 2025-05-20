use crate::{
    DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES,
    data::{Bundle, Bundles, Cred, Document, Documents, Secret, Transient},
};
use flexi_logger::Snapshot;
use fuzzy_matcher::clangd::fuzzy_match;
use std::time::Instant;

#[derive(Default)]
pub struct V {
    pub main_state: MainState,
    pub modal_state: ModalState,
    pub show_log: bool,

    pub v_bundles: Vec<VBundle>,
    pub v_documents: Vec<VDocument>,

    pub file_selection: FileSelection,
    pub pw: Pw,
    pub find: Find,
    pub lang: Lang,

    pub logger_snapshot: Snapshot,
}
impl V {
    pub fn reset_bundles(&mut self, bundles: &Bundles, o_scroll_to: Option<&str>) {
        self.v_bundles = bundles
            .iter()
            .map(|(name, bundle)| VBundle {
                suppressed: false,
                scroll_to: if let Some(s) = o_scroll_to {
                    s == name
                } else {
                    false
                },
                v_creds: vec![VCred::default(); bundle.creds().len()],
            })
            .collect();
        self.apply_filter_to_bundles(bundles);
    }

    pub fn reset_documents(&mut self, documents: &Documents, o_scroll_to: Option<&str>) {
        self.v_documents = documents
            .iter()
            .map(|(name, _document)| VDocument {
                suppressed: false,
                scroll_to: if let Some(s) = o_scroll_to {
                    s == name
                } else {
                    false
                },
            })
            .collect();
        self.apply_filter_to_documents(documents);
    }

    pub fn visible_bundles(&self) -> usize {
        self.v_bundles
            .iter()
            .filter(|bundle| !bundle.suppressed)
            .count()
    }

    pub fn apply_filter_to_bundles(&mut self, bundles: &Bundles) {
        for (vbundle, (name, bundle)) in self.v_bundles.iter_mut().zip(bundles.iter()) {
            vbundle.apply_filter(name, bundle, &self.find.pattern);
        }
    }

    pub fn apply_filter_to_documents(&mut self, documents: &Documents) {
        for (vdoc, name) in self.v_documents.iter_mut().zip(documents.iter_keys()) {
            vdoc.apply_filter(name, &self.find.pattern);
        }
    }
}

pub enum MainState {
    Bundles(BundleState),
    Documents(DocumentState),
}
impl MainState {
    pub fn is_bundles(&self) -> bool {
        matches!(self, Self::Bundles(_))
    }
    pub fn is_documents(&self) -> bool {
        matches!(self, Self::Documents(_))
    }
}

#[derive(Default)]
pub enum BundleState {
    #[default]
    Default,
    ModifyBundle {
        idx: usize,
        v_edit_bundle: VEditBundle,
        error: Option<String>,
    },
}

pub type OSelected = Option<(usize, String)>;
pub enum DocumentState {
    Default(OSelected),
    ModifyDocument {
        idx: usize,
        v_edit_document: VEditDocument,
        error: Option<String>,
    },
}
impl Default for DocumentState {
    fn default() -> Self {
        Self::Default(None)
    }
}

#[derive(Default)]
pub enum ModalState {
    #[default]
    None,
    AddBundle {
        v_edit_bundle: VEditBundle,
        error: Option<String>,
    },
    DeleteBundle {
        name: String,
        error: Option<String>,
    },
    AddDocument {
        v_edit_document: VEditDocument,
        error: Option<String>,
    },
    DeleteDocument {
        name: String,
        error: Option<String>,
    },
    About,
    ChangePassword,
    ChangeFile,
    ChangeLanguage,
}
impl Default for MainState {
    fn default() -> Self {
        Self::Bundles(BundleState::Default)
    }
}
impl ModalState {
    pub fn close_modal(&mut self) {
        *self = Self::None;
    }
    pub fn no_modal_is_open(&self) -> bool {
        matches!(self, Self::None)
    }
    pub fn is_ready_for_modal(&self) -> bool {
        matches!(self, ModalState::None)
    }
    pub fn get_id(&self) -> String {
        match self {
            Self::None => "ModalState::None".to_string(),
            Self::AddBundle { .. } => "ModalState::AddBundle".to_string(),
            Self::DeleteBundle { .. } => "ModalState::DeleteBundle".to_string(),
            Self::AddDocument { .. } => "ModalState::AddDocument".to_string(),
            Self::DeleteDocument { .. } => "ModalState::DeleteDocument".to_string(),
            Self::About => "ModalState::About".to_string(),
            Self::ChangePassword => "ModalState::ChangePassword".to_string(),
            Self::ChangeFile => "ModalState::ChangeFile".to_string(),
            Self::ChangeLanguage => "ModalState::ChangeLanguage".to_string(),
        }
    }
}

#[derive(Default)]
pub struct Find {
    pub pattern: String,
    pub request_focus: bool,
}

pub struct Lang {
    pub current: &'static (&'static str, &'static str),
    pub selected: &'static (&'static str, &'static str),
    pub error: Option<String>,
}
impl Default for Lang {
    fn default() -> Self {
        Self {
            current: DEFAULT_LANGUAGE,
            selected: DEFAULT_LANGUAGE,
            error: None,
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
        self.error = None;
    }
}

// Is used for
// - plain PW entry
// - initial PW entry (twice)
// - changing the PW (old, and twice new)
#[derive(Default)]
pub struct Pw {
    pub pw1: String,
    pub pw2: String,
    pub pw3: String,
    pub error: Option<String>,
    pub focus: PwFocus,
}

#[derive(Default)]
pub enum PwFocus {
    None,
    #[default]
    Pw1,
    Pw2,
    Pw3,
}

#[derive(Default)]
pub struct FileSelection {
    pub error: Option<String>,
    pub current: usize,
    pub new: String,
}
impl FileSelection {
    pub fn reset(&mut self, current: usize) {
        self.error = None;
        self.current = current;
        self.new.clear();
    }
}

#[derive(Default, Clone)]
pub struct VBundle {
    pub suppressed: bool,
    pub scroll_to: bool,
    pub v_creds: Vec<VCred>,
}
impl VBundle {
    pub fn apply_filter(&mut self, name: &str, bundle: &Bundle, pattern: &str) {
        self.suppressed = fuzzy_match(name, pattern).is_none()
            && fuzzy_match(bundle.description(), pattern).is_none();
    }
}

#[derive(Default, Debug, Clone)]
pub struct VDocument {
    pub suppressed: bool,
    pub scroll_to: bool,
}
impl VDocument {
    pub fn apply_filter(&mut self, name: &str, pattern: &str) {
        self.suppressed = fuzzy_match(name, pattern).is_none();
    }
}

#[derive(Default, Clone)]
pub struct VCred {
    pub show_secret: bool,
    pub copied_at: Option<Instant>,
}

pub struct VEditBundle {
    pub orig_name: String,
    pub name: String,
    pub description: String,
    pub v_edit_creds: Vec<VEditCred>,
    pub request_focus: bool,
}
impl VEditBundle {
    pub fn new() -> Self {
        let mut instance = Self {
            request_focus: true,
            orig_name: String::new(),
            name: String::new(),
            description: String::new(),
            v_edit_creds: Vec::new(),
        };
        instance.v_edit_creds.push(VEditCred::default());
        instance.v_edit_creds.push(VEditCred::default());
        instance.v_edit_creds.push(VEditCred::default());
        instance.v_edit_creds.push(VEditCred::default());
        instance
    }

    pub fn from_bundle(name: &str, bundle: &Bundle, transient: &Transient) -> Self {
        let mut result = VEditBundle {
            orig_name: name.to_string(),
            name: name.to_string(),
            description: bundle.description().to_string(),
            v_edit_creds: bundle
                .creds()
                .iter()
                .map(|cred| VEditCred {
                    name: cred.name.disclose(transient).to_string(),
                    secret: cred.secret.disclose(transient).to_string(),
                })
                .collect(),
            request_focus: true,
        };
        while result.v_edit_creds.len() < 4 {
            result.v_edit_creds.push(VEditCred::default());
        }
        result
    }

    pub fn as_oldname_newname_bundle(&self, transient: &mut Transient) -> (String, String, Bundle) {
        (
            self.orig_name.to_string(),
            self.name.to_string(),
            Bundle::new(
                self.description.clone(),
                self.v_edit_creds
                    .iter()
                    .filter_map(|vns| {
                        if vns.name.trim().is_empty() && vns.secret.trim().is_empty() {
                            None
                        } else {
                            Some(Cred::new(vns.name.clone(), vns.secret.clone(), transient))
                        }
                    })
                    .collect(),
            ),
        )
    }
}

#[derive(Default)]
pub struct VEditDocument {
    pub orig_name: String,
    pub name: String,
    pub text: String,
    pub request_focus: bool,
}
impl VEditDocument {
    pub fn new() -> Self {
        Self {
            request_focus: true,
            ..Default::default()
        }
    }

    pub fn from_document(name: &str, document: &Document, transient: &Transient) -> Self {
        VEditDocument {
            orig_name: name.to_string(),
            name: name.to_string(),
            text: document.secret().disclose(transient).to_string(),
            request_focus: true,
        }
    }

    pub fn as_oldname_newname_document(
        &self,
        transient: &mut Transient,
    ) -> (String, String, Document) {
        (
            self.orig_name.to_string(),
            self.name.to_string(),
            Document::new(Secret::new(self.text.clone(), transient)),
        )
    }
}

#[derive(Clone, Default)]
pub struct VEditCred {
    pub name: String,
    pub secret: String,
}
