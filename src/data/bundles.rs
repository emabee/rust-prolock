use crate::data::{Bundle, Transient};
use anyhow::{Result, anyhow};
use std::collections::{
    BTreeMap,
    btree_map::{Entry, Iter},
};

// All bundles in the file.
//
// When doing the desired modifications to `bundles`, we
// - always make sure that `bundles` only contain Passwords in variant Ref and that the
//   referenced value in Secret exists and is up-to-date
// - when converting a Password::New to Password::Ref, the ref-value is taken from the sequence
// - before storing the content: garbage-collect the Secret:
//     - retrieve a list of ref-values from `bundles`
//     - remove all bundles from secret that do not appear in the list

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Bundles(BTreeMap<String, Bundle>);

impl Bundles {
    pub fn new() -> Self {
        Bundles(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, std::string::String, Bundle> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Bundle> {
        self.0.get(key)
    }

    pub fn count_secrets(&self) -> usize {
        self.0.values().map(|bundle| bundle.creds().len()).sum()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn refs(&self) -> Vec<u64> {
        let mut left_refs: Vec<u64> = self.0.values().flat_map(Bundle::refs).collect();
        left_refs.sort_unstable();
        left_refs
    }

    pub fn add<S>(&mut self, key: S, bundle: Bundle) -> Result<()>
    where
        S: AsRef<str>,
    {
        let key = key.as_ref().to_string();
        if let Entry::Vacant(e) = self.0.entry(key.clone()) {
            e.insert(bundle);
            Ok(())
        } else {
            Err(anyhow!("bundle {key} exists already"))
        }
    }

    pub fn modify<S>(&mut self, key: S, modified_bundle: Bundle) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.get_mut(key.as_ref()) {
            None => Err(anyhow!("bundle {} does not exist", key.as_ref())),
            Some(bundle) => {
                *bundle = modified_bundle;
                Ok(())
            }
        }
    }

    pub fn remove_bundle_with_refs<S>(&mut self, key: S, transient: &mut Transient) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.remove_entry(key.as_ref()) {
            None => Err(anyhow!("bundle {} does not exist", key.as_ref())),
            Some((_key, bundle)) => {
                for cred in bundle.creds() {
                    transient.remove_secret(cred.name.0);
                    transient.remove_secret(cred.secret.0);
                }
                Ok(())
            }
        }
    }
}

impl<'a> IntoIterator for &'a Bundles {
    type Item = (&'a String, &'a Bundle);

    type IntoIter = std::collections::btree_map::Iter<'a, String, Bundle>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
