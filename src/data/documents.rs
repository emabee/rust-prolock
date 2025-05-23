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

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Document)> {
        self.0.iter()
    }

    pub fn iter_keys(&self) -> impl Iterator<Item = &Key> {
        self.0.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &Key) -> Option<&Document> {
        self.0.get(key)
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.contains_key(key)
    }

    pub fn refs(&self) -> Vec<u64> {
        let mut refs: Vec<u64> = self.0.values().map(Document::reff).collect();
        refs.sort_unstable();
        refs
    }

    pub fn add(&mut self, key: Key, document: Document) -> Result<()> {
        match self.0.entry(key) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(document);
                Ok(())
            }
            Entry::Occupied(occupied_entry) => {
                Err(anyhow!("secret {} exists already", occupied_entry.key()))
            }
        }
    }

    pub fn modify(&mut self, key: &Key, modified_document: Document) -> Result<()> {
        match self.0.get_mut(key) {
            None => Err(anyhow!("document {} does not exist", key.as_ref())),
            Some(occupied_entry) => {
                *occupied_entry = modified_document;
                Ok(())
            }
        }
    }

    pub fn remove_document_with_ref(&mut self, key: &Key, transient: &mut Transient) -> Result<()> {
        match self.0.remove_entry(key) {
            None => Err(anyhow!("key {} does not exist", key.as_ref())),
            Some((_key, secret)) => {
                transient.remove_secret(secret.reff());
                Ok(())
            }
        }
    }
}
