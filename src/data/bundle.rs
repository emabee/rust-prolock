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

    pub(super) fn refs(&self) -> Vec<u64> {
        self.creds
            .iter()
            .flat_map(|t| [&t.name, &t.secret].into_iter())
            .map(Secret::reff)
            .collect::<Vec<u64>>()
    }

    pub fn equals(
        &self,
        other: &Bundle,
        own_transient: &Transient,
        other_transient: &Transient,
    ) -> bool {
        let mut result =
            self.description == other.description && self.creds().len() == other.creds().len();
        if result {
            for (cred, other_cred) in self.creds().iter().zip(other.creds().iter()) {
                result = cred.name(own_transient) == other_cred.name(other_transient);
                if !result {
                    break;
                }
            }
        }
        result
    }
}
