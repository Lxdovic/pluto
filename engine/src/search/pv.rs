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

use shakmaty::{CastlingMode, Move};
use std::u8;

pub struct PvTable {
    pub length: Vec<i32>,
    pub table: Vec<Vec<Option<Move>>>,
}

impl PvTable {
    pub fn default() -> PvTable {
        const MAX_DEPTH: usize = u8::MAX as usize + 1;

        PvTable {
            length: vec![0; MAX_DEPTH],
            table: vec![vec![None; MAX_DEPTH]; MAX_DEPTH],
        }
    }

    pub fn store(&mut self, ply: usize, m: Move) {
        self.table[ply][ply] = Some(m);

        for next_ply in (ply as i32 + 1)..self.length[ply + 1] {
            self.table[ply][next_ply as usize] = self.table[ply + 1][next_ply as usize].clone();
        }

        self.length[ply] = self.length[ply + 1];
    }

    pub fn update_length(&mut self, ply: usize) {
        self.length[ply] = ply as i32;
    }

    pub fn collect(&self) -> Vec<String> {
        self.table[0][0..self.length[0] as usize]
            .iter()
            .map(|m| match m {
                Some(m) => m.to_uci(CastlingMode::Standard).to_string(),
                None => "".to_string(),
            })
            .collect()
    }

    pub fn get_best_move(&self) -> Option<Move> {
        self.table[0][0].clone()
    }
}
