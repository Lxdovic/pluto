use shakmaty::{CastlingMode, Position};

use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub(crate) struct Search<'a> {
    opt: &'a SearchOptions,
    #[allow(dead_code)]
    stop: Arc<AtomicBool>,
    result: SearchResult,
}

impl<'a> Search<'a> {
    pub(crate) fn from(opt: &'a SearchOptions, stop: Arc<AtomicBool>) -> Self {
        Search {
            opt,
            stop,
            result: SearchResult::new(),
        }
    }
}

impl<'a> Search<'a> {
    pub(crate) fn run(&mut self) -> &SearchResult {
        let moves = self.opt.position.legal_moves();

        let rng = rand::random_range(0..moves.len());
        let bestmove = moves[rng].to_uci(CastlingMode::Standard).to_string();

        self.result.bestmove = bestmove;
        self.result.nodes = 1;
        self.result.nps = 1;

        &self.result
    }
}
