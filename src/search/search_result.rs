use shakmaty::uci::UciMove;

use crate::search::eval::Score;

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub(crate) best_move: UciMove,
    pub(crate) score: Score,
    pub(crate) nodes: u64,
    pub(crate) nps: u64,
    pub(crate) time: u64,
}

impl SearchResult {
    pub(crate) fn new() -> Self {
        SearchResult {
            best_move: UciMove::Null,
            score: Score::Cp(0),
            nodes: 0,
            nps: 0,
            time: 0,
        }
    }
}
