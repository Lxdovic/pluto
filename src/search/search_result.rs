#[derive(Debug)]
pub(crate) struct SearchResult {
    pub(crate) bestmove: String,
    pub(crate) nodes: u64,
    pub(crate) nps: u64,
}
