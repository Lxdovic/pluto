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

pub mod history;
pub mod history_stack;
pub mod info;
pub mod killers;
pub mod move_picker;
pub mod params;
pub mod pv;
pub mod search;
pub mod tt;

use history::HistoryTable;
use info::SearchInfo;
use killers::Killers;
use params::SearchParams;
use pv::PvTable;
use shakmaty::Chess;
#[cfg(not(feature = "classical"))]
use shakmaty::Position;
use tt::TranspositionTable;

#[cfg(not(feature = "classical"))]
use crate::nnue::NNUEState;
use crate::search::history_stack::HistoryStack;
use crate::{config::Config, time_control::time_controller::TimeController};

pub struct SearchState {
    pub game: Chess,
    pub params: SearchParams,
    pub info: SearchInfo,
    pub tc: TimeController,
    #[cfg(not(feature = "classical"))]
    pub nnue: NNUEState,
    pub tt: TranspositionTable,
    pub hstack: HistoryStack,
    pub pv: PvTable,
    pub km: Killers,
    pub cfg: Config,
    pub hist: HistoryTable,
}

impl SearchState {
    pub fn new() -> Self {
        let cfg = Config::default();

        Self {
            game: Chess::default(),
            tt: TranspositionTable::new(cfg.hash.value * 1024 * 1024 / 24),
            info: SearchInfo::default(),
            tc: TimeController::default(),
            params: SearchParams::default(),
            #[cfg(not(feature = "classical"))]
            nnue: NNUEState::from_board(Chess::default().board()),
            hstack: HistoryStack::new(),
            pv: PvTable::default(),
            km: Killers::new(),
            hist: HistoryTable::new(),
            cfg,
        }
    }
}
