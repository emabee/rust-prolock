use crate::{
    DEFAULT_LANGUAGE, SUPPORTED_LANGUAGES,
    data::{Bundle, Bundles, Cred, Transient},
};
use fuzzy_matcher::clangd::fuzzy_match;
use std::time::Instant;

#[derive(Default)]
pub struct V {
    pub bundles: Vec<VBundle>,
    pub pw: Pw,
    pub file_selection: FileSelection,
    pub delete: Delete,
    pub find: Find,
    pub edit: Edit,
    pub lang: Lang,
}
impl V {
    pub fn reset_bundles(&mut self, bundles: &Bundles, o_scroll_to: Option<&str>) {
        self.bundles = bundles
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
        self.apply_filter(bundles);
    }

    pub fn visible_len(&self) -> usize {
        self.bundles
            .iter()
            .filter(|bundle| !bundle.suppressed)
            .count()
    }

    pub fn apply_filter(&mut self, bundles: &Bundles) {
        for (vbundle, (name, bundle)) in self.bundles.iter_mut().zip(bundles.iter()) {
            vbundle.apply_filter(name, bundle, &self.find.pattern);
        }
    }
}

#[derive(Default)]
pub struct Edit {
    pub idx: Option<usize>,
    pub bundle: VEditBundle,
    pub error: Option<String>,
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

#[derive(Default)]
pub struct Delete {
    pub name: String,
    pub error: Option<String>,
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
    pub request_focus: bool,
}
impl VEditBundle {
    pub fn from_bundle(name: &str, bundle: &Bundle, transient: &Transient) -> Self {
        let mut result = VEditBundle {
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
            request_focus: true,
        };
        while result.v_edit_creds.len() < 4 {
            result.v_edit_creds.push(VEditCred::default());
        }
        result
    }
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
        self.request_focus = true;
    }
}
