use std::fmt::{Display, Formatter};

// We use a newtype pattern to ensure that we only compare bundle keys in a case-insensitive way.
// This is important because the bundle keys are used as keys in a BTreeMap.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct BundleKey(String);
impl BundleKey {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        BundleKey(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl From<String> for BundleKey {
    fn from(s: String) -> Self {
        BundleKey(s)
    }
}
impl From<&str> for BundleKey {
    fn from(s: &str) -> Self {
        BundleKey(s.to_string())
    }
}
impl AsRef<str> for BundleKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl PartialEq<str> for BundleKey {
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<String> for BundleKey {
    fn eq(&self, other: &String) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<BundleKey> for String {
    fn eq(&self, other: &BundleKey) -> bool {
        self.eq_ignore_ascii_case(&other.0)
    }
}
impl PartialEq<BundleKey> for &str {
    fn eq(&self, other: &BundleKey) -> bool {
        self.eq_ignore_ascii_case(&other.0)
    }
}
impl Ord for BundleKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .to_ascii_lowercase()
            .cmp(&other.0.to_ascii_lowercase())
    }
}
impl PartialOrd for BundleKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Display for BundleKey {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
