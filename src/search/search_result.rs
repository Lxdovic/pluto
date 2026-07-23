use shakmaty::uci::UciMove;

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub(crate) bestmove: String,
    pub(crate) nodes: u64,
    pub(crate) nps: u64,
}

impl SearchResult {
    pub(crate) fn new() -> Self {
        SearchResult {
            bestmove: UciMove::Null.to_string(),
            nodes: 0,
            nps: 0,
        }
    }
}
