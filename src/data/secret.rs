use crate::data::Transient;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

#[derive(Clone, Debug)]
pub enum Secret {
    New(String),
    Ref(u64),
}
impl Secret {
    pub fn is_ref(&self) -> bool {
        matches!(self, Secret::Ref(_))
    }

    pub fn disclose(&self, transient: &Transient) -> String {
        match self {
            Secret::New(s) => s.clone(),
            Secret::Ref(i) => transient
                .get_secret(*i)
                .expect("wrong secret ref")
                .to_string(),
        }
    }
}
impl Default for Secret {
    fn default() -> Self {
        Secret::New(String::new())
    }
}
impl Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Secret::New(_) => unreachable!("Secret::New"),
            Secret::Ref(idx) => serializer.serialize_u64(*idx),
        }
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
        Ok(Secret::Ref(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "expecting a u64")
    }
}
