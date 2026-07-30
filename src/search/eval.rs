use std::fmt::Display;

use shakmaty::{Chess, Position};

pub(crate) struct Eval;

#[cfg_attr(any(), rustfmt::skip)]
const PAWN_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

#[cfg_attr(any(), rustfmt::skip)]
const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[cfg_attr(any(), rustfmt::skip)]
const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[cfg_attr(any(), rustfmt::skip)]
const ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

#[cfg_attr(any(), rustfmt::skip)]
const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[cfg_attr(any(), rustfmt::skip)]
const KING_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

#[derive(Debug)]
pub(crate) enum Score {
    Cp(i32),
    Mate(i32),
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Cp(score) => write!(f, "cp {}", score),
            Score::Mate(score) => write!(f, "mate {}", score),
        }
    }
}

impl Eval {
    pub(crate) fn simple(pos: &Chess) -> i32 {
        let mut score = 0;

        for (sq, piece) in pos.board() {
            let index = match piece.color {
                shakmaty::Color::White => 63 - sq as usize,
                shakmaty::Color::Black => sq as usize,
            };

            let value = match piece.role {
                shakmaty::Role::Pawn => 100 + PAWN_TABLE[index],
                shakmaty::Role::Knight => 300 + KNIGHT_TABLE[index],
                shakmaty::Role::Bishop => 300 + BISHOP_TABLE[index],
                shakmaty::Role::Rook => 500 + ROOK_TABLE[index],
                shakmaty::Role::Queen => 900 + QUEEN_TABLE[index],
                shakmaty::Role::King => 0 + KING_TABLE[index],
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
