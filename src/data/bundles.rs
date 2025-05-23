use crate::data::{Bundle, Key, Transient};
use anyhow::{Result, anyhow};
use std::collections::{BTreeMap, btree_map::Entry};

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
pub(crate) struct Bundles(BTreeMap<Key, Bundle>);

impl Bundles {
    pub fn new() -> Self {
        Bundles(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Bundle)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &Key) -> Option<&Bundle> {
        self.0.get(key)
    }

    pub fn count_secrets(&self) -> usize {
        self.0.values().map(|bundle| bundle.creds().len()).sum()
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.contains_key(key)
    }

    pub fn refs(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(self.0.values().flat_map(Bundle::refs))
    }

    pub fn add(&mut self, key: Key, bundle: Bundle) -> Result<()> {
        match self.0.entry(key) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(bundle);
                Ok(())
            }
            Entry::Occupied(occupied_entry) => {
                Err(anyhow!("bundle {} exists already", occupied_entry.key()))
            }
        }
    }

    pub fn modify(&mut self, key: &Key, modified_bundle: Bundle) -> Result<()> {
        match self.0.get_mut(key) {
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
        match self.0.remove_entry(&Key::new(key.as_ref())) {
            None => Err(anyhow!("bundle {} does not exist", key.as_ref())),
            Some((_key, bundle)) => {
                for cred in bundle.creds() {
                    transient.remove_secret(cred.name.reff());
                    transient.remove_secret(cred.secret.reff());
                }
                Ok(())
            }
        }
    }
}
