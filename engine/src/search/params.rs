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

pub struct SearchParams {
    pub depth: u8,
    pub move_time: u128,
    pub w_time: u128,
    pub b_time: u128,
}

impl Default for SearchParams {
    fn default() -> Self {
        SearchParams {
            depth: 5,
            move_time: 0,
            w_time: 0,
            b_time: 0,
        }
    }
}
