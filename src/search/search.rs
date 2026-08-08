use crate::search::eval::{Eval, Score};
use crate::search::move_picker::MovePicker;
use crate::search::search_options::BuiltSearchOptions;
use crate::search::time::TimeManager;
use crate::search::tt::{TTBound, TranspositionTable};
use crate::search::{search_options::SearchOptions, search_result::SearchResult};
use shakmaty::uci::UciMove;
use shakmaty::zobrist::Zobrist64;
use shakmaty::{CastlingMode, Chess, EnPassantMode, Move, Position};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

pub(crate) const MATE_SCORE: i16 = 30_000;
pub(crate) const MAX_DEPTH: u8 = u8::MAX;

pub(crate) struct Search<'a> {
    opt: BuiltSearchOptions,
    #[allow(dead_code)]
    stop: Arc<AtomicBool>,
    result: SearchResult,
    start_time: SystemTime,
    tt: &'a mut TranspositionTable,
}

impl<'a> Search<'a> {
    pub(crate) fn from(
        opt: &SearchOptions,
        stop: Arc<AtomicBool>,
        tt: &'a mut TranspositionTable,
    ) -> Self {
        let built_opt = opt.build();

        Search {
            opt: built_opt,
            stop,
            result: SearchResult::new(),
            start_time: SystemTime::now(),
            tt,
        }
    }
}

impl<'a> Search<'a> {
    fn init(&mut self) {
        self.tt.bump_generation();
        self.result = SearchResult::new();
        self.start_time = SystemTime::now();
    }

    pub(crate) fn run(&mut self) -> &SearchResult {
        self.init();

        let max_depth = match self.opt.depth {
            Some(depth) => depth + 1,
            None => MAX_DEPTH,
        }
        // make sure at least 1 depth is searched
        .max(2);

        for d in 1..max_depth {
            let pos = self.opt.position.clone();
            let (score, best_move) = self.root_negamax(&pos, d, -MATE_SCORE, MATE_SCORE, 0);

            self.result.time = SystemTime::now()
                .duration_since(self.start_time)
                .unwrap()
                .as_millis() as u64;

            self.result.nps =
                (self.result.nodes as f64 / (self.result.time.max(1) as f64 / 1000.0)) as u64;

            if self.stop.load(Ordering::Relaxed) {
                println!(
                    "info depth {} time {} score {} nodes {} nps {} bestmove {}",
                    d,
                    self.result.time,
                    self.result.score,
                    self.result.nodes,
                    self.result.nps,
                    self.result.best_move
                );

                break;
            }

            match MAX_DEPTH as i16 > MATE_SCORE - score.abs() {
                true => {
                    self.result.score = match score > 0 {
                        true => Score::Mate((MATE_SCORE - score) as i8 / 2 + 1),
                        false => Score::Mate((-MATE_SCORE - score) as i8 / 2),
                    }
                }
                false => self.result.score = Score::Cp(score),
            }

            self.result.best_move = best_move;

            println!(
                "info depth {} time {} score {} nodes {} nps {} bestmove {}",
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
        depth: u8,
        alpha: i16,
        beta: i16,
        ply: u32,
    ) -> (i16, UciMove) {
        let mut alpha = alpha;
        let mut best_score = -MATE_SCORE;
        let mut best_move = UciMove::Null;

        let moves = pos.legal_moves();
        let mut mp = MovePicker::new(moves.to_vec(), None);

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

    fn negamax(&mut self, pos: &Chess, depth: u8, alpha: i16, beta: i16, ply: u32) -> i16 {
        self.result.nodes += 1;

        if let Some(search_nodes) = self.opt.nodes {
            if self.result.nodes >= search_nodes {
                self.stop.store(true, Ordering::Relaxed);

                return 0;
            }
        }

        if self.result.nodes % 1024 == 0 {
            if TimeManager::should_stop(self.start_time, &self.opt) {
                self.stop.store(true, Ordering::Relaxed);

                return 0;
            }
        }

        if depth == 0 {
            return self.qsearch(pos, alpha, beta, ply);
        }

        let is_root = ply == 0;
        let position_key: Zobrist64 = pos.zobrist_hash::<Zobrist64>(EnPassantMode::Legal);
        let entry = self.tt.probe(position_key);

        if let Some(entry) = entry
            && !is_root
            && entry.generation == self.tt.generation()
            && entry.depth >= depth
            && (entry.bound == TTBound::Exact
                || (entry.bound == TTBound::Alpha && entry.score <= alpha)
                || (entry.bound == TTBound::Beta && entry.score >= beta))
        {
            return entry.score;
        }

        let moves = pos.legal_moves();

        if moves.is_empty() {
            match pos.is_check() {
                true => return -MATE_SCORE + ply as i16,
                false => return 0,
            }
        }

        let start_alpha = alpha;
        let mut alpha = alpha;
        let mut best_score = -MATE_SCORE;
        let mut best_move: Option<Move> = None;

        let mut mp = MovePicker::new(moves.to_vec(), entry.and_then(|e| e.best_move));
        let mut move_index = 0;

        while let Some(m) = mp.next() {
            move_index += 1;

            let child = pos.clone().play(m).unwrap();
            let mut r = 1;

            if depth >= 3 && move_index >= 2 && !child.is_check() {
                r = (1.5 + (depth as f32).ln() * (move_index as f32).ln() / 3.0) as u8;
                r = r.clamp(1, depth);
            }

            let score = -self.negamax(&child, depth - r, -beta, -alpha, ply + 1);

            if self.stop.load(Ordering::Relaxed) {
                return 0;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(m);

                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                break;
            }
        }

        let bound = match best_score {
            score if score <= start_alpha => TTBound::Alpha,
            score if score >= beta => TTBound::Beta,
            _ => TTBound::Exact,
        };

        self.tt
            .store(position_key, depth, best_score, bound, best_move);

        best_score
    }

    fn qsearch(&mut self, pos: &Chess, alpha: i16, beta: i16, ply: u32) -> i16 {
        self.result.nodes += 1;

        if let Some(search_nodes) = self.opt.nodes {
            if self.result.nodes >= search_nodes {
                self.stop.store(true, Ordering::Relaxed);

                return 0;
            }
        }

        if self.result.nodes % 1024 == 0 {
            if TimeManager::should_stop(self.start_time, &self.opt) {
                self.stop.store(true, Ordering::Relaxed);

                return 0;
            }
        }

        let stand_pat = Eval::simple(pos);

        if stand_pat >= beta {
            return beta;
        }

        let mut alpha = alpha;

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let moves = pos.capture_moves();

        for m in moves {
            let child = pos.clone().play(m).unwrap();
            let score = -self.qsearch(&child, -beta, -alpha, ply + 1);

            if self.stop.load(Ordering::Relaxed) {
                return 0;
            }

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}
