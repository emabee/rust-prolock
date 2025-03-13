use crate::data::{Cred, Secret, Transient};
use jiff::Zoned;

// A bundle.
//
// Contains zero or more named secrets.
// Secret has two variants, New and Ref.
// A bundle can only be serialized (i.e., written to the file)
// if each Secret it contains has Variant Ref.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Bundle {
    description: String,
    creds: Vec<Cred>,
    last_changed_at: Zoned,
}
impl Bundle {
    pub(crate) fn new(description: String, creds: Vec<Cred>) -> Self {
        Self {
            description,
            creds,
            last_changed_at: Zoned::now(),
        }
    }
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
    pub(crate) fn creds(&self) -> &[Cred] {
        &self.creds
    }
    pub(crate) fn last_changed_at(&self) -> &Zoned {
        &self.last_changed_at
    }
    pub(super) fn convert_new_secrets_to_refs(&mut self, transient: &mut Transient) {
        for cred in &mut self.creds {
            if let Secret::New(s) = &cred.name {
                let ref_value = transient.add_secret(s.clone());
                cred.name = Secret::Ref(ref_value);
            }
            if let Secret::New(s) = &cred.secret {
                let ref_value = transient.add_secret(s.clone());
                cred.secret = Secret::Ref(ref_value);
            }
        }
    }

    pub(super) fn refs(&self) -> (Vec<u64>, bool) {
        let mut found_non_reffed_secrets = false;
        (
            self.creds
                .iter()
                .flat_map(|t| [&t.name, &t.secret].into_iter())
                .filter_map(|secret| {
                    if let Secret::Ref(n) = secret {
                        Some(*n)
                    } else {
                        found_non_reffed_secrets = true;
                        None
                    }
                })
                .collect::<Vec<u64>>(),
            found_non_reffed_secrets,
        )
    }

    pub(super) fn is_storable(&self) -> bool {
        for cred in &self.creds {
            if !(cred.name.is_ref() && cred.secret.is_ref()) {
                return false;
            }
        }
        true
    }
}
