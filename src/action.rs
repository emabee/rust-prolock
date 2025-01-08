pub(crate) enum Action {
    Modified(String),
    Rename(String, String),
    Delete(String),
    None,
}
