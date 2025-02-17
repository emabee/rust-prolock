use crate::data::{Bundle, Bundles, Secret, Transient};
use std::time::Instant;

#[derive(Default)]
pub struct V {
    pub pw: Pw,
    pub search: String,
    pub burger: Burger,
    pub bundles: Vec<VBundle>,
    pub edit_idx: EditIdx,
    pub edit_bundle: VEditBundle,
    pub need_refresh: bool,
}
impl V {
    pub(crate) fn new() -> Self {
        V::default()
    }

    pub(crate) fn reset_bundles(&mut self, bundles: &Bundles, transient: &Transient) {
        self.bundles = bundles
            .into_iter()
            .map(|(name, bundle)| VBundle {
                name: name.to_string(),
                description: bundle.description.clone(),
                v_named_secrets: bundle
                    .named_secrets
                    .iter()
                    .map(|(name, secret)| VNamedSecret {
                        name: name.clone(),
                        secret: secret.disclose(transient),
                        show_secret: false,
                        copied_at: None,
                    })
                    .collect(),
            })
            .collect();
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Burger {
    #[default]
    None,
    About,
    ChangePassword,
    ChangeLanguage,
    ShowPrintable,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditIdx {
    #[default]
    None,
    Mod(usize),
    New(usize),
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
    pub fn is_new(&self) -> bool {
        matches!(self, Self::New(_))
    }
    pub fn is_new_with(&self, idx: usize) -> bool {
        if let Self::New(i) = self {
            *i == idx
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
pub struct VBundle {
    pub name: String,
    pub description: String,
    pub v_named_secrets: Vec<VNamedSecret>,
}

#[derive(Clone, Default)]
pub struct VNamedSecret {
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
    pub v_named_secrets: Vec<VNamedSecret>,
}

impl VEditBundle {
    // pub fn from_bundle(name: &str, bundle: &Bundle, transient: &Transient) -> Self {
    //     VEditBundle {
    //         orig_name: name.to_string(),
    //         name: name.to_string(),
    //         description: bundle.description.clone(),
    //         v_named_secrets: bundle
    //             .named_secrets
    //             .iter()
    //             .map(|(name, secret)| VNamedSecret {
    //                 name: name.clone(),
    //                 secret: secret.disclose(transient),
    //                 show_secret: false,
    //                 copied_at: None,
    //             })
    //             .collect(),
    //     }
    // }

    pub fn as_bundle(&self) -> (String, String, Bundle) {
        (
            self.orig_name.to_string(),
            self.name.to_string(),
            Bundle {
                description: self.description.clone(),
                named_secrets: self
                    .v_named_secrets
                    .iter()
                    .map(|vns| (vns.name.clone(), Secret::New(vns.secret.clone())))
                    .collect(),
            },
        )
    }

    pub fn clear(&mut self) {
        *self = Self::default();
        self.v_named_secrets.push(VNamedSecret::default());
    }
}
