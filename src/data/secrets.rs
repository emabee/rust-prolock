use rand::{distr::StandardUniform, rng, Rng};
use std::collections::{hash_map::Keys, HashMap};

// A map from u64 to String, containing the secret values, keyed by some number.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Secrets {
    random_string: String,
    content: HashMap<u64, String>,
}
impl Secrets {
    pub fn prepare(&mut self) {
        let len: usize = rng().random_range(20..80);
        self.random_string = rng()
            .sample_iter::<char, _>(StandardUniform)
            .take(len)
            .collect();
    }

    #[must_use]
    pub fn keys(&self) -> Keys<u64, String> {
        self.content.keys()
    }

    pub fn add(&mut self, idx: u64, s: String) -> Option<String> {
        self.content.insert(idx, s)
    }

    pub fn remove(&mut self, idx: u64) {
        self.content.remove(&idx);
    }

    #[must_use]
    pub fn get(&self, idx: u64) -> Option<&String> {
        self.content.get(&idx)
    }
}
