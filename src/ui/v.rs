use std::time::Instant;

use crate::data::{Bundle, Secret, Transient};
pub struct V {
    pub bundles: Vec<VBundle>,
    pub search: String,
    pub edit_idx: Option<usize>,
    pub edit_bundle: VEditBundle,
}

#[derive(Default)]
pub struct VBundle {
    pub name: String,
    pub description: String,
    pub v_named_secrets: Vec<VNamedSecret>,
}

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
    pub fn from_bundle(name: &str, bundle: &Bundle, transient: &Transient) -> Self {
        VEditBundle {
            orig_name: name.to_string(),
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
        }
    }

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
}
