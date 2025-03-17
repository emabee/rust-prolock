use crate::{
    DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES,
    data::{Bundle, Bundles, Cred, Transient},
};
use std::time::Instant;

#[derive(Default)]
pub struct V {
    pub pw: Pw,
    pub file_selection: FileSelection,
    pub name_for_delete: String,
    pub search: String,
    pub bundles: Vec<VBundle>,
    pub edit_idx: Option<usize>,
    pub edit_bundle: VEditBundle,
    pub lang: Lang,
}
impl V {
    pub fn reset_bundles(&mut self, bundles: &Bundles) {
        self.bundles = bundles
            .iter()
            .map(|(_name, bundle)| VBundle {
                v_creds: vec![VCred::default(); bundle.creds().len()],
            })
            .collect();
    }
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
    pub err: Option<String>,
    pub current: usize,
    pub new: String,
}
impl FileSelection {
    pub fn reset(&mut self, current: usize) {
        self.err = None;
        self.current = current;
        self.new.clear();
    }
}

#[derive(Default, Clone)]
pub struct VBundle {
    pub v_creds: Vec<VCred>,
}

#[derive(Default, Clone)]
pub struct VCred {
    pub show_secret: bool,
    pub copied_at: Option<Instant>,
}

#[derive(Default)]
pub struct VEditBundle {
    pub orig_name: String,
    pub name: String,
    pub description: String,
    pub v_edit_creds: Vec<VEditCred>,
    pub err: Option<String>,
}

#[derive(Clone, Default)]
pub struct VEditCred {
    pub name: String,
    pub secret: String,
}

impl VEditBundle {
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

    pub fn prepare_for_create(&mut self) {
        *self = Self::default();
        self.v_edit_creds.push(VEditCred::default());
        self.v_edit_creds.push(VEditCred::default());
        self.v_edit_creds.push(VEditCred::default());
        self.v_edit_creds.push(VEditCred::default());
    }
}
