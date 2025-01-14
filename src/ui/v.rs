use std::time::Instant;

pub struct V {
    pub bundles: Vec<VBundle>,
    pub search: String,
    pub edit_idx: Option<usize>,
}

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
