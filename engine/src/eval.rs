/* Pluto, UCI chess engine
   Copyright (C) 2025 Ludovic Debever

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

/// Position evaluation module containing piece-square tables and evaluation functions.
#[cfg(not(feature = "classical"))]
use crate::nnue::{NNUEState, NNUE};
#[cfg(feature = "classical")]
use crate::packing::s;
#[cfg(feature = "classical")]
use crate::packing::{extract_eg, extract_mg};
#[cfg(feature = "classical")]
use shakmaty::{attacks, Bitboard, Piece, Role, Square};
use shakmaty::{Chess, Color, Position};

#[cfg(feature = "classical")]
#[derive(Default)]
pub struct EvalState {
    phase: i32,
    eval: [i32; 2],
}

#[cfg(feature = "classical")]
type EvalRoleFn = fn(&Chess, Square, Piece) -> i32;

pub struct Eval {}

impl Eval {
    pub fn has_pieces(pos: &Chess) -> bool {
        let material = pos.board().material_side(pos.turn());

        if material.knight > 0 || material.bishop > 0 || material.rook > 0 || material.queen > 0 {
            return true;
        }

        false
    }

    #[cfg(feature = "classical")]
    fn eval_pawn(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let doubled = Self::doubled(pos, sq, piece);
        let isolated = Self::isolated(pos, sq, piece);
        let passed = Self::passed(pos, sq, piece);

        doubled + isolated + passed
    }

    #[cfg(feature = "classical")]
    fn eval_knight(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        Self::mobility(pos, sq, piece)
    }

    #[cfg(feature = "classical")]
    fn eval_bishop(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        Self::mobility(pos, sq, piece)
    }

    #[cfg(feature = "classical")]
    fn eval_rook(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let mobility = Self::mobility(pos, sq, piece);
        let files = Self::rook_files(pos, sq);

        mobility + files
    }

    #[cfg(feature = "classical")]
    fn eval_queen(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        Self::mobility(pos, sq, piece)
    }

    #[cfg(feature = "classical")]
    fn eval_king(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        Self::king_shield(pos, sq, piece)
    }

    #[cfg(feature = "classical")]
    fn mobility(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let attacks = attacks::attacks(sq, piece, pos.board().occupied());

        MOBILITY[attacks.count()]
    }

    #[cfg(feature = "classical")]
    fn doubled(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let pawns = pos.board().by_piece(piece);
        let file = FILES_TABLE[sq.file() as usize];
        let count = pawns.intersect(file).count();

        DOUBLED[count]
    }

    #[cfg(feature = "classical")]
    fn isolated(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let isolation = ADJACENT_FILES_TABLE[sq.file() as usize];
        let our_pawns = pos.board().by_piece(piece);

        if isolation.intersect(our_pawns).count() == 0 {
            return ISOLATED;
        }

        0
    }

    #[cfg(feature = "classical")]
    fn passed(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let isolation = ADJACENT_AND_FILE_TABLE[sq.file() as usize];
        let their_pawns = pos.board().by_piece(Piece {
            role: piece.role,
            color: piece.color.other(),
        });

        if isolation.intersect(their_pawns).count() == 0 {
            return PASSED;
        }

        0
    }

    #[cfg(feature = "classical")]
    fn king_shield(pos: &Chess, sq: Square, piece: Piece) -> i32 {
        let our_pawns = pos.board().by_piece(Piece {
            role: Role::Pawn,
            color: piece.color,
        });

        let index = attacks::attacks(sq, piece, Bitboard(0))
            .intersect(our_pawns)
            .count();

        KING_SHIELD[index]
    }

    #[cfg(feature = "classical")]
    fn bishop_pair(pos: &Chess, state: &mut EvalState) {
        let bishops = pos.board().bishops();
        let white_bishops = pos.board().white().intersect(bishops);
        let black_bishops = pos.board().black().intersect(bishops);

        if white_bishops.count() >= 2 {
            state.eval[Color::White as usize] += BISHOP_PAIR
        }

        if black_bishops.count() >= 2 {
            state.eval[Color::Black as usize] += BISHOP_PAIR
        }
    }

    #[cfg(feature = "classical")]
    fn rook_files(pos: &Chess, sq: Square) -> i32 {
        let board = pos.board();
        let us = pos.turn();
        let file = FILES_TABLE[sq.file() as usize];
        let all_pawns = board.pawns();

        if file.intersect(all_pawns).count() == 0 {
            return ROOK_FILES[0];
        }

        let our_pawns = board.by_piece(Piece {
            role: Role::Pawn,
            color: us,
        });

        if file.intersect(our_pawns).count() == 0 {
            return ROOK_FILES[1];
        }

        0
    }
    #[cfg(feature = "classical")]
    fn eval_piece(pos: &Chess, sq: Square, piece: Piece, state: &mut EvalState) {
        let role_index = piece.role as usize - 1;
        let piece_index = role_index * 2 + (!piece.color as usize);
        let square_index = sq as usize;
        let piece_score = EVAL_ROLES[role_index](pos, sq, piece);

        state.eval[piece.color as usize] += TABLE[piece_index][square_index] + piece_score;
        state.phase += GAME_PHASES[piece_index];
    }

    #[cfg(feature = "classical")]
    fn tempo(pos: &Chess, state: &mut EvalState) {
        state.eval[pos.turn() as usize] += TEMPO;
    }

    #[cfg(feature = "classical")]
    pub fn eval(pos: &Chess) -> i32 {
        let mut state = EvalState::default();

        for (sq, piece) in pos.board() {
            Self::eval_piece(pos, sq, piece, &mut state);
        }

        Self::tempo(pos, &mut state);
        Self::bishop_pair(pos, &mut state);

        let score = state.eval[Color::White as usize] - state.eval[Color::Black as usize];
        let mg = extract_mg(score);
        let eg = extract_eg(score);

        (mg * state.phase + eg * (24 - state.phase)) / 24
            * if pos.turn() == Color::White { 1 } else { -1 }
    }

    #[cfg(feature = "classical")]
    pub const fn init_piece_table() -> [[i32; 64]; 12] {
        let mut eg_table = [[0; 64]; 12];

        let mut p = 0;
        let mut pc = 0;
        let mut sq = 0;

        while p < 6 {
            while sq < 64 {
                eg_table[pc][sq] = PIECE_VALUES[p] + PESTO_TABLE[p][sq ^ 56];
                eg_table[pc + 1][sq] = PIECE_VALUES[p] + PESTO_TABLE[p][sq];

                sq += 1;
            }

            sq = 0;
            p += 1;
            pc += 2;
        }

        eg_table
    }

    #[cfg(not(feature = "classical"))]
    pub fn nnue_eval(state: &NNUEState, pos: &Chess) -> i32 {
        #[rustfmt::skip]
        let (us, them) = match pos.turn() {
            Color::White => (state.stack[state.current].white, state.stack[state.current].black),
            Color::Black => (state.stack[state.current].black, state.stack[state.current].white),
        };

        NNUE.evaluate(&us, &them)
    }
}

#[cfg(feature = "classical")]
const TABLE: [[i32; 64]; 12] = Eval::init_piece_table();
#[cfg(feature = "classical")]
const PESTO_TABLE: [[i32; 64]; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];
#[cfg(feature = "classical")]
#[rustfmt::skip]
pub const PIECE_VALUES: [i32; 6] = [s(82, 94), s(337, 281), s(365, 297), s(447, 512), s(1025, 936), s(0, 0)];
#[cfg(feature = "classical")]
const TEMPO: i32 = s(28, 0);
#[cfg(feature = "classical")]
const GAME_PHASES: [i32; 12] = [0, 0, 1, 1, 1, 1, 2, 2, 4, 4, 0, 0];
#[cfg(feature = "classical")]
const EVAL_ROLES: [EvalRoleFn; 6] = [
    Eval::eval_pawn,
    Eval::eval_knight,
    Eval::eval_bishop,
    Eval::eval_rook,
    Eval::eval_queen,
    Eval::eval_king,
];

#[cfg(feature = "classical")]
const PASSED: i32 = s(10, 10);
#[cfg(feature = "classical")]
const ISOLATED: i32 = s(-10, -10);
#[cfg(feature = "classical")]
#[rustfmt::skip]
const KING_SHIELD: [i32; 9] = [s(0, 0), s(1, 1), s(2, 2), s(3, 3), s(4, 4), s(5, 5), s(6, 6), s(7, 7), s(8, 8)];
#[cfg(feature = "classical")]
const ROOK_FILES: [i32; 2] = [s(20, 0), s(10, 0)];
#[cfg(feature = "classical")]
const BISHOP_PAIR: i32 = s(10, 8);

#[cfg(feature = "classical")]
const DOUBLED: [i32; 9] = [
    s(5, 5),
    s(0, 0),
    s(-5, -5),
    s(-10, -10),
    s(-15, -15),
    s(-20, -20),
    s(-25, -25),
    s(-30, -30),
    s(-35, -35),
];
#[cfg(feature = "classical")]
const MOBILITY: [i32; 28] = [
    s(0, 0),
    s(1, 1),
    s(2, 2),
    s(3, 3),
    s(4, 4),
    s(5, 5),
    s(6, 6),
    s(7, 7),
    s(8, 8),
    s(9, 9),
    s(10, 10),
    s(11, 11),
    s(12, 12),
    s(13, 13),
    s(14, 14),
    s(15, 15),
    s(16, 16),
    s(17, 17),
    s(18, 18),
    s(19, 19),
    s(20, 20),
    s(21, 21),
    s(22, 22),
    s(23, 23),
    s(24, 24),
    s(25, 25),
    s(26, 26),
    s(27, 27),
];

/* gives adjacent files for index i
* if i = 2:
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
* 0 1 0 1 0 0 0 0
*/
#[cfg(feature = "classical")]
const ADJACENT_FILES_TABLE: [Bitboard; 8] = [
    Bitboard(0x202020202020202),
    Bitboard(0x505050505050505),
    Bitboard(0xa0a0a0a0a0a0a0a),
    Bitboard(0x1414141414141414),
    Bitboard(0x2828282828282828),
    Bitboard(0x5050505050505050),
    Bitboard(0xa0a0a0a0a0a0a0a0),
    Bitboard(0x4040404040404040),
];

#[cfg(feature = "classical")]
const FILES_TABLE: [Bitboard; 8] = [
    Bitboard(0x101010101010101),
    Bitboard(0x202020202020202),
    Bitboard(0x404040404040404),
    Bitboard(0x808080808080808),
    Bitboard(0x1010101010101010),
    Bitboard(0x2020202020202020),
    Bitboard(0x4040404040404040),
    Bitboard(0x8080808080808080),
];

/* gives adjacent and current files for index i
* if i = 2:
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
* 0 1 1 1 0 0 0 0
*/
#[cfg(feature = "classical")]
const ADJACENT_AND_FILE_TABLE: [Bitboard; 8] = [
    Bitboard(0x101010101010101 & 0x202020202020202),
    Bitboard(0x202020202020202 & 0x505050505050505),
    Bitboard(0x404040404040404 & 0xa0a0a0a0a0a0a0a),
    Bitboard(0x808080808080808 & 0x1414141414141414),
    Bitboard(0x1010101010101010 & 0x2828282828282828),
    Bitboard(0x2020202020202020 & 0x5050505050505050),
    Bitboard(0x4040404040404040 & 0xa0a0a0a0a0a0a0a0),
    Bitboard(0x8080808080808080 & 0x4040404040404040),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const PAWN: [i32; 64] = [
    s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s( 0,   0), s(  0,   0),
    s( 98, 178), s(134, 173), s( 61, 158), s( 95, 134), s( 68, 147), s(126, 132), s(34, 165), s(-11, 187),
    s( -6,  94), s(  7, 100), s( 26,  85), s( 31,  67), s( 65,  56), s( 56,  53), s(25,  82), s(-20,  84),
    s(-14,  32), s( 13,  24), s(  6,  13), s( 21,   5), s( 23,  -2), s( 12,   4), s(17,  17), s(-23,  17),
    s(-27,  13), s( -2,   9), s( -5,  -3), s( 12,  -7), s( 17,  -7), s(  6,  -8), s(10,   3), s(-25,  -1),
    s(-26,   4), s( -4,   7), s( -4,  -6), s(-10,   1), s(  3,   0), s(  3,  -5), s(33,  -1), s(-12,  -8),
    s(-35,  13), s( -1,   8), s(-20,   8), s(-23,  10), s(-15,  13), s( 24,   0), s(38,   2), s(-22,  -7),
    s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s(  0,   0), s( 0,   0), s(  0,   0),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const KNIGHT: [i32; 64] = [
    s(-167, -58), s(-89, -38), s(-34, -13), s(-49, -28), s( 61, -31), s(-97, -27), s(-15, -63), s(-107, -99),
    s( -73, -25), s(-41,  -8), s( 72, -25), s( 36,  -2), s( 23,  -9), s( 62, -25), s(  7, -24), s( -17, -52),
    s( -47, -24), s( 60, -20), s( 37,  10), s( 65,   9), s( 84,  -1), s(129,  -9), s( 73, -19), s(  44, -41),
    s(  -9, -17), s( 17,   3), s( 19,  22), s( 53,  22), s( 37,  22), s( 69,  11), s( 18,   8), s(  22, -18),
    s( -13, -18), s(  4,  -6), s( 16,  16), s( 13,  25), s( 28,  16), s( 19,  17), s( 21,   4), s(  -8, -18),
    s( -23, -23), s( -9,  -3), s( 12,  -1), s( 10,  15), s( 19,  10), s( 17,  -3), s( 25, -20), s( -16, -22),
    s( -29, -42), s(-53, -20), s(-12, -10), s( -3,  -5), s( -1,  -2), s( 18, -20), s(-14, -23), s( -19, -44),
    s(-105, -29), s(-21, -51), s(-58, -23), s(-33, -15), s(-17, -22), s(-28, -18), s(-19, -50), s( -23, -64),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const BISHOP: [i32; 64] = [
    s(-29, -14), s( 4, -21), s(-82, -11), s(-37,  -8), s(-25, -7), s(-42,  -9), s(  7, -17), s( -8, -24),
    s(-26,  -8), s(16,  -4), s(-18,   7), s(-13, -12), s( 30, -3), s( 59, -13), s( 18,  -4), s(-47, -14),
    s(-16,   2), s(37,  -8), s( 43,   0), s( 40,  -1), s( 35, -2), s( 50,   6), s( 37,   0), s( -2,   4),
    s( -4,  -3), s( 5,   9), s( 19,  12), s( 50,   9), s( 37, 14), s( 37,  10), s(  7,   3), s( -2,   2),
    s( -6,  -6), s(13,   3), s( 13,  13), s( 26,  19), s( 34,  7), s( 12,  10), s( 10,  -3), s(  4,  -9),
    s(  0, -12), s(15,  -3), s( 15,   8), s( 15,  10), s( 14, 13), s( 27,   3), s( 18,  -7), s( 10, -15),
    s(  4, -14), s(15, -18), s( 16,  -7), s(  0,  -1), s(  7,  4), s( 21,  -9), s( 33, -15), s(  1, -27),
    s(-33, -23), s(-3,  -9), s(-14, -23), s(-21,  -5), s(-13, -9), s(-12, -16), s(-39,  -5), s(-21, -17),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const ROOK: [i32; 64] = [
    s( 32, 13), s( 42, 10), s( 32, 18), s( 51, 15), s(63, 12), s( 9,  12), s( 31,   8), s( 43,   5),
    s( 27, 11), s( 32, 13), s( 58, 13), s( 62, 11), s(80, -3), s(67,   3), s( 26,   8), s( 44,   3),
    s( -5,  7), s( 19,  7), s( 26,  7), s( 36,  5), s(17,  4), s(45,  -3), s( 61,  -5), s( 16,  -3),
    s(-24,  4), s(-11,  3), s(  7, 13), s( 26,  1), s(24,  2), s(35,   1), s( -8,  -1), s(-20,   2),
    s(-36,  3), s(-26,  5), s(-12,  8), s( -1,  4), s( 9, -5), s(-7,  -6), s(  6,  -8), s(-23, -11),
    s(-45, -4), s(-25,  0), s(-16, -5), s(-17, -1), s( 3, -7), s( 0, -12), s( -5,  -8), s(-33, -16),
    s(-44, -6), s(-16, -6), s(-20,  0), s( -9,  2), s(-1, -9), s(11,  -9), s( -6, -11), s(-71,  -3),
    s(-19, -9), s(-13,  2), s(  1,  3), s( 17, -1), s(16, -5), s( 7, -13), s(-37,   4), s(-26, -20),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const QUEEN: [i32; 64] = [
    s(-28,  -9), s(  0,  22), s( 29,  22), s( 12,  27), s( 59,  27), s( 44,  19), s( 43,  10), s( 45,  20),
    s(-24, -17), s(-39,  20), s( -5,  32), s(  1,  41), s(-16,  58), s( 57,  25), s( 28,  30), s( 54,   0),
    s(-13, -20), s(-17,   6), s(  7,   9), s(  8,  49), s( 29,  47), s( 56,  35), s( 47,  19), s( 57,   9),
    s(-27,   3), s(-27,  22), s(-16,  24), s(-16,  45), s( -1,  57), s( 17,  40), s( -2,  57), s(  1,  36),
    s( -9, -18), s(-26,  28), s( -9,  19), s(-10,  47), s( -2,  31), s( -4,  34), s(  3,  39), s( -3,  23),
    s(-14, -16), s(  2, -27), s(-11,  15), s( -2,   6), s( -5,   9), s(  2,  17), s( 14,  10), s(  5,   5),
    s(-35, -22), s( -8, -23), s( 11, -30), s(  2, -16), s(  8, -16), s( 15, -23), s( -3, -36), s(  1, -32),
    s( -1, -33), s(-18, -28), s( -9, -22), s( 10, -43), s(-15,  -5), s(-25, -32), s(-31, -20), s(-50, -41),
];

#[cfg(feature = "classical")]
#[rustfmt::skip]
const KING: [i32; 64] = [
    s(-65, -74), s( 23, -35), s( 16, -18), s(-15, -18), s(-56, -11), s(-34,  15), s(  2,   4), s( 13, -17),
    s( 29, -12), s( -1,  17), s(-20,  14), s( -7,  17), s( -8,  17), s( -4,  38), s(-38,  23), s(-29,  11),
    s( -9,  10), s( 24,  17), s(  2,  23), s(-16,  15), s(-20,  20), s(  6,  45), s( 22,  44), s(-22,  13),
    s(-17,  -8), s(-20,  22), s(-12,  24), s(-27,  27), s(-30,  26), s(-25,  33), s(-14,  26), s(-36,   3),
    s(-49, -18), s( -1,  -4), s(-27,  21), s(-39,  24), s(-46,  27), s(-44,  23), s(-33,   9), s(-51, -11),
    s(-14, -19), s(-14,  -3), s(-22,  11), s(-46,  21), s(-44,  23), s(-30,  16), s(-15,   7), s(-27,  -9),
    s(  1, -27), s(  7, -11), s( -8,   4), s(-64,  13), s(-43,  14), s(-16,   4), s(  9,  -5), s(  8, -17),
    s(-15, -53), s( 36, -34), s( 12, -21), s(-54, -11), s(  8, -28), s(-28, -14), s( 24, -24), s( 14, -43),
];
