use super::{secret::Secret, Transient};

// A bundle.
//
// Contains zero or more named secrets.
// Secret has two variants, New and Ref.
// A bundle can only be serialized (i.e., written to the file)
// if each Secret it contains has Variant Ref.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Bundle {
    pub description: String,
    pub creds: Vec<Cred>,
}
impl Bundle {
    pub(super) fn convert_new_secrets_to_refs(&mut self, transient: &mut Transient) {
        for cred in &mut self.creds {
            if let Secret::New(s) = &cred.name {
                let ref_value = transient.add_secret_value(s.clone());
                cred.name = Secret::Ref(ref_value);
            }
            if let Secret::New(s) = &cred.secret {
                let ref_value = transient.add_secret_value(s.clone());
                cred.secret = Secret::Ref(ref_value);
            }
        }
    }

    pub fn refs(&self) -> (Vec<u64>, bool) {
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

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Cred {
    pub name: Secret,
    pub secret: Secret,
}
impl Cred {
    pub(crate) fn new(name: String, password: String) -> Self {
        Self {
            name: Secret::New(name),
            secret: Secret::New(password),
        }
    }
    // pub(crate) fn is_storable(&self) -> bool {
    //     self.name.is_ref() && self.secret.is_ref()
    // }
    pub(crate) fn name(&self, transient: &Transient) -> String {
        self.name.disclose(transient)
    }
    pub(crate) fn secret(&self, transient: &Transient) -> String {
        self.secret.disclose(transient)
    }
}
