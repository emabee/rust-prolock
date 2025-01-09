pub type VBundles = Vec<VBundle>;

pub struct VBundle {
    pub name: String,
    pub description: String,
    pub named_secrets: Vec<(String, bool, String)>,
    pub save_modal_open: bool,
    pub save_progress: Option<f32>,
}
