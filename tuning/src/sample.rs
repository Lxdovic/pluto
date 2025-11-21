use shakmaty::Chess;

use crate::outcome::Outcome;

pub struct Sample {
    pub pos: Chess,
    pub outcome: Outcome,
    pub phase: i32,
    pub base_index: usize,
    pub coeffs_count: usize,
}
