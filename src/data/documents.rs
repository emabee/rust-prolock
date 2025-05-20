use crate::data::{Document, Key, Transient};
use anyhow::{Result, anyhow};
use std::collections::{BTreeMap, btree_map::Entry};

// Documents in the file.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub(crate) struct Documents(BTreeMap<Key, Document>);

impl Documents {
    pub fn new() -> Self {
        Documents(BTreeMap::new())
    }

    pub fn _len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Document)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(Key::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Document> {
        self.0.get(&Key::new(key))
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(&Key::new(key))
    }

    pub fn refs(&self) -> Vec<u64> {
        let mut refs: Vec<u64> = self.0.values().map(Document::reff).collect();
        refs.sort_unstable();
        refs
    }

    pub fn add<S>(&mut self, key: S, document: Document) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.entry(Key::new(key.as_ref())) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(document);
                Ok(())
            }
            Entry::Occupied(occupied_entry) => {
                Err(anyhow!("secret {} exists already", occupied_entry.key()))
            }
        }
    }

    pub fn modify<S>(&mut self, key: S, modified_document: Document) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.get_mut(&Key::new(key.as_ref())) {
            None => Err(anyhow!("document {} does not exist", key.as_ref())),
            Some(occupied_entry) => {
                *occupied_entry = modified_document;
                Ok(())
            }
        }
    }

    pub fn remove_document_with_ref<S>(&mut self, key: S, transient: &mut Transient) -> Result<()>
    where
        S: AsRef<str>,
    {
        match self.0.remove_entry(&Key::new(key.as_ref())) {
            None => Err(anyhow!("key {} does not exist", key.as_ref())),
            Some((_key, secret)) => {
                transient.remove_secret(secret.reff());
                Ok(())
            }
        }
    }
}
