use std::time::Instant;

pub type VBundles = Vec<VBundle>;

pub struct VBundle {
    pub name: String,
    pub description: String,
    pub named_secrets: Vec<NamedSecret>,
}
pub struct NamedSecret {
    pub name: String,
    pub secret: String,
    pub show_secret: bool,
    pub copied_at: Option<Instant>,
}
