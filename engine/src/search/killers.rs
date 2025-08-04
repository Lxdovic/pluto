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
use std::u8;

pub struct Killers {
    table: Vec<Vec<Move>>,
}

impl Killers {
    pub fn new() -> Self {
        Self {
            table: vec![Vec::new(); u8::MAX as usize],
        }
    }

    pub fn get(&self, ply: usize) -> &Vec<Move> {
        &self.table[ply]
    }

    pub fn store(&mut self, ply: usize, m: Move) {
        if ply >= self.table.len() {
            return;
        }

        let killers = &mut self.table[ply];

        if !killers.contains(&m) {
            killers.pop();
            killers.insert(0, m);
        }
    }
}
