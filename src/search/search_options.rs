pub(crate) struct SearchOptions {
    pub(crate) depth: Option<u32>,
    pub(crate) nodes: Option<u64>,
    pub(crate) movetime: Option<u64>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            depth: None,
            nodes: None,
            movetime: None,
        }
    }
}