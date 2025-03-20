use crate::data::{Bundle, Transient};
use anyhow::{Result, anyhow};
use std::collections::{BTreeMap, btree_map::Entry};

use super::BundleKey;

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
pub(crate) struct Bundles(BTreeMap<BundleKey, Bundle>);

impl Bundles {
    pub fn new() -> Self {
        Bundles(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Bundle)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Bundle> {
        self.0.get(&BundleKey::new(key))
    }

    pub fn count_secrets(&self) -> usize {
        self.0.values().map(|bundle| bundle.creds().len()).sum()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(&BundleKey::new(key))
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
        match self.0.entry(BundleKey::new(key.as_ref())) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(bundle);
                Ok(())
            }
            Entry::Occupied(occupied_entry) => {
                Err(anyhow!("bundle {} exists already", occupied_entry.key()))
            }
        }
    }

    pub fn modify<S>(&mut self, key: S, modified_bundle: Bundle) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.get_mut(&BundleKey::new(key.as_ref())) {
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
        match self.0.remove_entry(&BundleKey::new(key.as_ref())) {
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
