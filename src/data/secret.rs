use crate::data::Transient;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

#[derive(Clone, Debug)]
pub struct Secret(pub u64);
impl Secret {
    pub fn disclose<'t>(&self, transient: &'t Transient) -> &'t str {
        transient.get_secret(self.0).expect("wrong secret ref")
    }
}
impl Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}
impl<'de> Deserialize<'de> for Secret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
        Ok(Secret(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a u64")
    }
}
