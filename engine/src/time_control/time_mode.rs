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

#[derive(Debug)]
pub enum TimeMode {
    Infinite,
    MoveTime,
    WOrBTime,
}

impl TimeMode {
    pub(crate) fn is_finite(tc: &TimeMode) -> bool {
        match tc {
            TimeMode::MoveTime => true,
            TimeMode::WOrBTime => true,
            _ => false,
        }
    }
}

impl PartialEq for TimeMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeMode::Infinite, TimeMode::Infinite) => true,
            (TimeMode::MoveTime, TimeMode::MoveTime) => true,
            (TimeMode::WOrBTime, TimeMode::WOrBTime) => true,
            _ => false,
        }
    }
}
