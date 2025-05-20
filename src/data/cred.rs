use crate::data::Transient;
use crate::data::secret::Secret;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Cred {
    pub(crate) name: Secret,
    pub(crate) secret: Secret,
}

impl Cred {
    pub(crate) fn new(name: String, password: String, transient: &mut Transient) -> Self {
        Self {
            name: Secret::new(name, transient),
            secret: Secret::new(password, transient),
        }
    }
    pub(crate) fn name<'t>(&self, transient: &'t Transient) -> &'t str {
        self.name.disclose(transient)
    }
    pub(crate) fn secret<'t>(&self, transient: &'t Transient) -> &'t str {
        self.secret.disclose(transient)
    }
}
