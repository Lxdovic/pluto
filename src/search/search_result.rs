use shakmaty::uci::UciMove;

use crate::search::search::MATE_SCORE;

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub(crate) best_move: UciMove,
    pub(crate) score: i32,
    pub(crate) nodes: u64,
    pub(crate) nps: u64,
    pub(crate) time: u64,
}

impl SearchResult {
    pub(crate) fn new() -> Self {
        SearchResult {
            best_move: UciMove::Null,
            score: -MATE_SCORE,
            nodes: 0,
            nps: 0,
            time: 0,
        }
    }
}
