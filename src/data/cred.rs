use crate::data::Transient;
use crate::data::secret::Secret;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Cred {
    pub(crate) name: Secret,
    pub(crate) secret: Secret,
}

impl Cred {
    pub(crate) fn new(name: String, password: String) -> Self {
        Self {
            name: Secret::New(name),
            secret: Secret::New(password),
        }
    }
    pub(crate) fn name(&self, transient: &Transient) -> String {
        self.name.disclose(transient)
    }
    pub(crate) fn secret(&self, transient: &Transient) -> String {
        self.secret.disclose(transient)
    }
}
