use super::{secret::Secret, Transient};
use std::collections::BTreeMap;

// A bundle.
//
// Contains zero or more named secrets.
// Secret has two variants, New and Ref.
// A bundle can only be serialized (i.e., written to the file)
// if each Secret it contains has Variant Ref.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Bundle {
    pub description: String,
    pub named_secrets: BTreeMap<String, Secret>,
}
impl Bundle {
    #[cfg(test)]
    pub fn new<S: AsRef<str>>(description: S) -> Self {
        Self {
            description: description.as_ref().to_string(),
            named_secrets: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub fn new_with_creds<S: AsRef<str> + Ord>(description: &S, creds: &[(S, S)]) -> Self {
        let mut named_secrets: BTreeMap<String, Secret> = BTreeMap::new();
        for (name, secret) in creds {
            named_secrets.insert(
                name.as_ref().to_string(),
                Secret::New(secret.as_ref().to_string()),
            );
        }
        Self {
            description: description.as_ref().to_string(),
            named_secrets,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.named_secrets.len()
    }

    // #[cfg(test)]
    // #[must_use]
    // pub fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }

    // #[cfg(test)]
    // pub fn update_description(&mut self, description: &str) {
    //     self.description.clear();
    //     self.description.insert_str(0, description);
    // }
    #[cfg(test)]
    pub fn add_cred(&mut self, id: String, password: String) {
        self.named_secrets.insert(id, Secret::New(password));
    }

    pub(super) fn convert_new_secrets_to_refs(&mut self, transient: &mut Transient) {
        for pw in self.named_secrets.values_mut() {
            if let Secret::New(s) = pw {
                let ref_value = transient.add_secret_value(s.clone());
                *pw = Secret::Ref(ref_value);
            }
        }
    }

    pub fn refs(&self) -> (Vec<u64>, bool) {
        let mut found_non_reffed_secrets = false;
        (
            self.named_secrets
                .values()
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
        for pw in self.named_secrets.values() {
            if let Secret::New(_) = *pw {
                return false;
            }
        }
        true
    }

    pub(super) fn secret_value(&self, name: &str, transient: &Transient) -> String {
        self.named_secrets[name].disclose(transient)
    }
}
