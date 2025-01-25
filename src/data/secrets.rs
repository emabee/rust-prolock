use std::{
    collections::{hash_map::Keys, HashMap},
    fmt::Write,
};

// A map from u64 to String, containing the secret values, keyed by some number.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Secrets(HashMap<u64, String>);

impl Secrets {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn keys(&self) -> Keys<u64, String> {
        self.0.keys()
    }

    pub fn add(&mut self, idx: u64, s: String) -> Option<String> {
        self.0.insert(idx, s)
    }

    pub(crate) fn remove(&mut self, idx: &u64) {
        self.0.remove(idx);
    }

    #[must_use]
    pub fn get(&self, idx: u64) -> Option<&String> {
        self.0.get(&idx)
    }

    pub fn write_keys(&self, w: &mut dyn Write) {
        let mut keys = self.0.keys().map(Clone::clone).collect::<Vec<u64>>();
        keys.sort();
        write!(w, "[").unwrap();
        for key in keys {
            write!(w, "{},", key).unwrap();
        }
        write!(w, "]").unwrap();
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
