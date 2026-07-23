use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Move, Position};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{SystemTime, UNIX_EPOCH};

const MATE_SCORE: i32 = 30_000;

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
        let mut bestmove: Option<Move> = None;
        let mut bestscore = i32::MIN;
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        for m in moves {
            let child = self.opt.position.clone().play(m).unwrap();
            let score = -self.negamax(&child, 3);

            if score > bestscore {
                bestscore = score;
                bestmove = Some(m);
            }
        }

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let bestmove: UciMove =
            bestmove.map_or(UciMove::Null, |m| m.to_uci(CastlingMode::Standard));

        self.result.bestmove = bestmove.to_string();
        self.result.nps = (self.result.nodes as f64 / (end - start).as_secs_f64()) as u64;

        &self.result
    }

    fn negamax(&mut self, pos: &Chess, depth: u32) -> i32 {
        self.result.nodes += 1;

        if depth == 0 {
            return self.eval(pos);
        }

        let moves = pos.legal_moves();
        let mut best_score = -MATE_SCORE;

        for m in moves {
            let child = pos.clone().play(m).unwrap();
            let score = -self.negamax(&child, depth - 1);

            best_score = best_score.max(score);
        }

        best_score
    }

    fn eval(&self, pos: &Chess) -> i32 {
        let mut score = 0;

        for (_sq, piece) in pos.board() {
            let value = match piece.role {
                shakmaty::Role::Pawn => 1,
                shakmaty::Role::Knight => 3,
                shakmaty::Role::Bishop => 3,
                shakmaty::Role::Rook => 5,
                shakmaty::Role::Queen => 9,
                shakmaty::Role::King => 0,
            };

            match piece.color {
                shakmaty::Color::White => score += value,
                shakmaty::Color::Black => score -= value,
            }
        }

        match pos.turn() {
            shakmaty::Color::White => score,
            shakmaty::Color::Black => -score,
        }
    }
}
