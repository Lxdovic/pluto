use crate::search::time::TimeManager;
use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use shakmaty::{CastlingMode, Chess, Position};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::SystemTime;

pub(crate) const MATE_SCORE: i32 = 30_000;
pub(crate) const MAX_DEPTH: u32 = 1024;

pub(crate) struct Search<'a> {
    opt: &'a SearchOptions,
    #[allow(dead_code)]
    stop: Arc<AtomicBool>,
    result: SearchResult,
    start_time: SystemTime,
}

impl<'a> Search<'a> {
    pub(crate) fn from(opt: &'a SearchOptions, stop: Arc<AtomicBool>) -> Self {
        Search {
            opt,
            stop,
            result: SearchResult::new(),
            start_time: SystemTime::now(),
        }
    }
}

impl<'a> Search<'a> {
    fn init(&mut self) {
        self.result = SearchResult::new();
        self.start_time = SystemTime::now();
    }

    pub(crate) fn run(&mut self) -> &SearchResult {
        self.init();

        let max_depth = self.opt.depth.unwrap_or(MAX_DEPTH) + 1;
        let moves = self.opt.position.legal_moves();

        for d in 1..max_depth {
            for m in moves.iter() {
                let child = self.opt.position.clone().play(*m).unwrap();
                let score = -self.negamax(&child, d - 1, -MATE_SCORE, MATE_SCORE);

                if score > self.result.score {
                    self.result.score = score;
                    self.result.best_move = m.to_uci(CastlingMode::Standard);
                }
            }

            self.result.time = SystemTime::now()
                .duration_since(self.start_time)
                .unwrap()
                .as_millis() as u64;

            println!(
                "info depth {} time {} score cp {} nodes {} bestmove {}",
                d, self.result.time, self.result.score, self.result.nodes, self.result.best_move
            );

            if TimeManager::should_stop(self.start_time, self.opt.move_time) {
                break;
            }
        }

        self.result.nps = (self.result.nodes as f64 / self.result.time as f64 * 1000.0) as u64;

        &self.result
    }

    fn negamax(&mut self, pos: &Chess, depth: u32, alpha: i32, beta: i32) -> i32 {
        self.result.nodes += 1;

        if self.result.nodes % 1024 == 0 {
            if TimeManager::should_stop(self.start_time, self.opt.move_time) {
                self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }

        if depth == 0 {
            return self.eval(pos);
        }

        let moves = pos.legal_moves();
        let mut alpha = alpha;
        let mut best_score = -MATE_SCORE;

        for m in moves {
            let child = pos.clone().play(m).unwrap();
            let score = -self.negamax(&child, depth - 1, -beta, -alpha);

            if self.stop.load(std::sync::atomic::Ordering::Relaxed) {
                return -0;
            }

            if score > best_score {
                best_score = score;

                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                return best_score;
            }
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
