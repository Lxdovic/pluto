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

use shakmaty::{Role, Square};

pub struct HistoryTable {
    table: [[i32; 64]; 6],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            table: [[0; 64]; 6],
        }
    }

    pub fn update(&mut self, piece: Role, to: Square, value: i32) {
        self.table[piece as usize - 1][to as usize] += value;
    }

    pub fn get(&self, piece: Role, to: Square) -> i32 {
        self.table[piece as usize - 1][to as usize]
    }

    pub fn new_search(&mut self) {
        for p in self.table.iter_mut() {
            for val in p.iter_mut() {
                *val /= 2;
            }
        }
    }
}
