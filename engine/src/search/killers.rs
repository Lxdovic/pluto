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

use shakmaty::Move;

pub struct Killers {
    table: [Vec<Move>; 64],
}

impl Killers {
    pub fn new() -> Self {
        Self {
            table: [const { Vec::new() }; 64],
        }
    }

    pub fn get(&self, ply: usize) -> &Vec<Move> {
        &self.table[ply]
    }

    pub fn store(&mut self, ply: usize, m: Move) {
        if ply >= 64 {
            return;
        }

        if !self.get(ply).contains(&m) {
            self.table[ply].pop();
            self.table[ply].insert(0, m);
        }
    }
}
