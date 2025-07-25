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
use crate::nnue::{NNUEState, NNUE};
use shakmaty::{Chess, Color, Position};

pub struct Eval {}

impl Eval {
    pub fn has_pieces(pos: &Chess) -> bool {
        let material = pos.board().material_side(pos.turn());

        if material.knight > 0 || material.bishop > 0 || material.rook > 0 || material.queen > 0 {
            return true;
        }

        false
    }

    pub fn nnue_eval(state: &NNUEState, pos: &Chess) -> i32 {
        #[rustfmt::skip]
        let (us, them) = match pos.turn() {
            Color::White => (state.stack[state.current].white, state.stack[state.current].black),
            Color::Black => ( state.stack[state.current].black, state.stack[state.current].white),
        };

        NNUE.evaluate(&us, &them)
    }
}
