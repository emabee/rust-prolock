use crate::s_idx::SIdx;
use std::collections::{hash_map::Keys, HashMap};

// A map from SIdx to String
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Secrets(HashMap<SIdx, String>);

impl Secrets {
    #[must_use]
    pub fn keys(&self) -> Keys<u64, String> {
        self.0.keys()
    }

    pub fn add(&mut self, idx: u64, s: String) -> Option<String> {
        self.0.insert(idx, s)
    }

    #[must_use]
    pub fn get(&self, idx: SIdx) -> Option<&String> {
        self.0.get(&idx)
    }
}

// impl serde::ser::Serialize for Secret {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(self.0.len()))?;
//         for (k, v) in &self.0 {
//             map.serialize_bundle(k, v)?;
//         }
//         map.end()
//     }
// }

// impl<'de> serde::de::Deserialize<'de> for Secret {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let visitor = SecretVisitor;
//         deserializer.deserialize_map(visitor)
//     }
// }
// struct SecretVisitor;
// impl<'de> serde::de::Visitor<'de> for SecretVisitor {
//     type Value = Secret;
//     fn visit_map<E>(self, value: u128) -> Result<Self::Value, E>
//     where
//         E: serde::de::Error,
//     {
//         Ok(Secret::from(value?))
//     }

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(formatter, "expecting a u64")
//     }
// }
