use crate::data::{Transient, secret::Secret};
use jiff::Zoned;

// A document.
//
// Contains a single Secret
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Document {
    secret: Secret,
    last_changed_at: Zoned,
}
impl Document {
    pub(crate) fn new(secret: Secret) -> Self {
        Self {
            secret,
            last_changed_at: Zoned::now(),
        }
    }
    pub(crate) fn secret(&self) -> &Secret {
        &self.secret
    }

    pub(crate) fn text<'t>(&self, transient: &'t Transient) -> &'t str {
        self.secret.disclose(transient)
    }

    pub(crate) fn last_changed_at(&self) -> &Zoned {
        &self.last_changed_at
    }

    pub(super) fn reff(&self) -> u64 {
        self.secret.reff()
    }
}
