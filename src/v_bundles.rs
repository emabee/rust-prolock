pub type VBundles = Vec<VBundle>;

pub struct VBundle {
    pub name: String,
    pub description: String,
    pub named_secrets: Vec<(String, bool, String)>,
}
