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

pub const TEST: i32 = 0;

#[cfg(target_arch = "wasm32")]
use crate::uci::UciController;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Worker;

mod bound;
mod config;
pub mod eval;
mod logger;
#[cfg(not(feature = "classical"))]
mod nnue;
pub mod packing;
mod rng;
mod search;
mod time_control;
mod uci;

#[cfg(target_arch = "wasm32")]
use std::sync::{LazyLock, Mutex};

#[cfg(target_arch = "wasm32")]
static UCI: LazyLock<Mutex<UciController>> = LazyLock::new(|| Mutex::new(UciController::web()));

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_wasm() {
    log("id name CastledEngine");
    log("id author CastledChess");
    log("uciok");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_wasm(command: &str) {
    let mut uci = UCI.lock().unwrap();

    uci.parse_command(&command);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = self)]
    fn postMessage(s: &str);
}
