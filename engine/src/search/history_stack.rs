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

use shakmaty::zobrist::Zobrist64;

pub struct HistoryStack {
    pub stack: Vec<HistoryStackEntry>,
}

pub struct HistoryStackEntry {
    key: Zobrist64,
    eval: Option<i32>,
}

impl HistoryStack {
    pub fn new() -> Self {
        HistoryStack { stack: Vec::new() }
    }
}

impl HistoryStack {
    pub fn push(&mut self, zobrist: Zobrist64, eval: Option<i32>) {
        self.stack.push(HistoryStackEntry { key: zobrist, eval });
    }

    pub fn pop(&mut self) -> Option<HistoryStackEntry> {
        self.stack.pop()
    }

    pub fn count_zobrist(&self, zobrist: Zobrist64) -> usize {
        self.stack
            .iter()
            .rev()
            .skip(1)
            .filter(|&h| h.key == zobrist)
            .count()
    }

    pub fn get_eval(&self, ply: usize) -> Option<i32> {
        if ply >= self.stack.len() {
            return None;
        }

        self.stack[ply].eval
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
