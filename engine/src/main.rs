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

use std::env;

mod bound;
mod config;
mod eval;
mod logger;
#[cfg(not(feature = "classical"))]
mod nnue;
mod search;
mod time_control;
mod uci;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use uci::UciReader;

    let args: Vec<String> = env::args().collect();
    UciReader::default().run(args);
}
