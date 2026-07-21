use shakmaty::{CastlingMode, Position};

use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub(crate) struct Search;

impl Search {
    pub(crate) fn run(options: &SearchOptions, _stop: Arc<AtomicBool>) -> SearchResult {
        let moves = options.position.legal_moves();

        let rng = rand::random_range(0..moves.len());
        let bestmove = moves[rng].to_uci(CastlingMode::Standard).to_string();

        SearchResult { bestmove }
    }
}
