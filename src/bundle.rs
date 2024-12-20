use std::collections::BTreeMap;

use crate::{s_idx::SIdx, transient::Transient};
use serde::de::Visitor;

// A bundle.
//
// Contains zero or more named secrets.
// Secret has two variants, New and Ref.
// A bundle can only be serialized (i.e., written to the file)
// if each Secret it contains has Variant Ref.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Bundle {
    description: String,
    named_secrets: BTreeMap<String, Secret>,
}
impl Bundle {
    pub fn new<S: AsRef<str>>(description: S) -> Self {
        Self {
            description: description.as_ref().to_string(),
            named_secrets: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.named_secrets.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn update_description(&mut self, description: &str) {
        self.description.clear();
        self.description.insert_str(0, description);
    }
    pub fn add_cred(&mut self, id: String, password: String) {
        self.named_secrets.insert(id, Secret::New(password));
    }

    pub(super) fn convert_to_refs(&mut self, transient: &mut Transient) {
        for pw in self.named_secrets.values_mut() {
            if let Secret::New(s) = pw {
                let ref_value = transient.add_secret_value(s.clone());
                *pw = Secret::Ref(ref_value);
            }
        }
    }

    pub(super) fn is_storable(&self) -> bool {
        for pw in self.named_secrets.values() {
            if let Secret::New(_) = *pw {
                return false;
            }
        }
        true
    }

    pub(super) fn secret(&self, name: &str, transient: &Transient) -> String {
        self.named_secrets[name].resolve(transient)
    }
}

#[derive(Clone, Debug)]
enum Secret {
    New(String),
    Ref(SIdx),
}
impl Secret {
    fn resolve(&self, transient: &Transient) -> String {
        match self {
            Secret::New(s) => s.clone(),
            Secret::Ref(i) => transient
                .get_secret_value(*i)
                .expect("index out of bounds")
                .to_string(),
        }
    }
}
impl Default for Secret {
    fn default() -> Self {
        Secret::New(String::new())
    }
}
impl serde::ser::Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Secret::New(_) => unreachable!("Password::New"),
            Secret::Ref(idx) => serializer.serialize_u64(*idx),
        }
    }
}
impl<'de> serde::de::Deserialize<'de> for Secret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = PwVisitor;
        deserializer.deserialize_u64(visitor)
    }
}

struct PwVisitor;
impl Visitor<'_> for PwVisitor {
    type Value = Secret;

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Secret::Ref(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a u64")
    }
}
