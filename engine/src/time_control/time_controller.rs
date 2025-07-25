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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::time_control::time_mode::TimeMode;
use crate::{config::Config, search::params::SearchParams};
use chrono::Local;
use shakmaty::{Chess, Color, Position};

pub struct TimeController {
    pub time_mode: TimeMode,
    pub play_time: u128,
    pub stop: Arc<AtomicBool>,
    start_time: i64,
}

impl TimeController {
    pub fn start(&mut self) {
        self.start_time = Local::now().timestamp_millis();
    }

    pub fn setup(&mut self, params: &SearchParams, game: &Chess, cfg: &Config) {
        self.play_time = match self.time_mode {
            TimeMode::MoveTime => params.move_time,
            TimeMode::WOrBTime => match game.turn() {
                Color::White => params.w_time / cfg.tc_time_divisor.value as u128,
                Color::Black => params.b_time / cfg.tc_time_divisor.value as u128,
            },
            _ => 0,
        };

        self.start();
    }

    pub fn elapsed(&self) -> i64 {
        Local::now().timestamp_millis() - self.start_time
    }

    pub fn is_time_up(&self) -> bool {
        if self.stop.load(Ordering::SeqCst) {
            return true;
        }

        if !TimeMode::is_finite(&self.time_mode) {
            return false;
        }

        let elapsed = Local::now().timestamp_millis() - self.start_time;

        elapsed as u128 > self.play_time
    }
}

impl Default for TimeController {
    fn default() -> Self {
        TimeController {
            start_time: Local::now().timestamp_millis(),
            time_mode: TimeMode::Infinite,
            stop: Arc::new(AtomicBool::new(false)),
            play_time: 0,
        }
    }
}
