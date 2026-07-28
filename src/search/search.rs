use crate::search::eval::Eval;
use crate::search::move_picker::MovePicker;
use crate::search::search_options::BuiltSearchOptions;
use crate::search::time::TimeManager;
use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Position};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

pub(crate) const MATE_SCORE: i32 = 30_000;
pub(crate) const MAX_DEPTH: u32 = 1024;

pub(crate) struct Search {
    opt: BuiltSearchOptions,
    #[allow(dead_code)]
    stop: Arc<AtomicBool>,
    result: SearchResult,
    start_time: SystemTime,
}

impl Search {
    pub(crate) fn from(opt: &SearchOptions, stop: Arc<AtomicBool>) -> Self {
        Search {
            opt: opt.build(),
            stop,
            result: SearchResult::new(),
            start_time: SystemTime::now(),
        }
    }
}

impl Search {
    fn init(&mut self) {
        self.result = SearchResult::new();
        self.start_time = SystemTime::now();
    }

    pub(crate) fn run(&mut self) -> &SearchResult {
        self.init();

        let max_depth = self.opt.depth.unwrap_or(MAX_DEPTH) + 1;

        for d in 1..max_depth {
            let pos = self.opt.position.clone();
            let (score, best_move) = self.root_negamax(&pos, d, -MATE_SCORE, MATE_SCORE, 0);

            if self.stop.load(Ordering::Relaxed) {
                break;
            }

            self.result.score = score;
            self.result.best_move = best_move;

            self.result.time = SystemTime::now()
                .duration_since(self.start_time)
                .unwrap()
                .as_millis() as u64;

            self.result.nps =
                (self.result.nodes as f64 / (self.result.time.max(1) as f64 / 1000.0)) as u64;

            println!(
                "info depth {} time {} score cp {} nodes {} nps {} bestmove {}",
                d,
                self.result.time,
                self.result.score,
                self.result.nodes,
                self.result.nps,
                self.result.best_move
            );
        }

        self.result.nps = (self.result.nodes as f64 / self.result.time as f64 * 1000.0) as u64;

        &self.result
    }

    fn root_negamax(
        &mut self,
        pos: &Chess,
        depth: u32,
        alpha: i32,
        beta: i32,
        ply: u32,
    ) -> (i32, UciMove) {
        let mut alpha = alpha;
        let mut best_score = -MATE_SCORE;
        let mut best_move = UciMove::Null;

        let moves = pos.legal_moves();
        let mut mp = MovePicker::new(moves.to_vec());

        while let Some(m) = mp.next() {
            let child = pos.clone().play(m).unwrap();
            let score = -self.negamax(&child, depth - 1, -beta, -alpha, ply + 1);

            if self.stop.load(Ordering::Relaxed) {
                break;
            }

            if score > best_score {
                best_score = score;
                best_move = m.to_uci(CastlingMode::Standard);

                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                break;
            }
        }

        (best_score, best_move)
    }

    fn negamax(&mut self, pos: &Chess, depth: u32, alpha: i32, beta: i32, ply: u32) -> i32 {
        self.result.nodes += 1;

        if self.result.nodes % 1024 == 0 {
            if TimeManager::should_stop(self.start_time, &self.opt) {
                self.stop.store(true, Ordering::Relaxed);

                return 0;
            }
        }

        if depth == 0 {
            return Eval::simple(pos);
        }

        let moves = pos.legal_moves();

        if moves.is_empty() {
            match pos.is_check() {
                true => return -MATE_SCORE + ply as i32,
                false => return 0,
             }
        }

        let mut alpha = alpha;
        let mut best_score = -MATE_SCORE;

        let mut mp = MovePicker::new(moves.to_vec());

        while let Some(m) = mp.next() {
            let child = pos.clone().play(m).unwrap();
            let score = -self.negamax(&child, depth - 1, -beta, -alpha, ply + 1);

            if self.stop.load(Ordering::Relaxed) {
                return 0;
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
}
