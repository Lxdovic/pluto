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
#[repr(u8)]
pub enum Bound {
    Exact = 0,
    Beta = 1,
    Alpha = 2,
}

impl Clone for Bound {
    fn clone(&self) -> Self {
        match self {
            Bound::Exact => Bound::Exact,
            Bound::Beta => Bound::Beta,
            Bound::Alpha => Bound::Alpha,
        }
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Bound::Exact, Bound::Exact)
                | (Bound::Beta, Bound::Beta)
                | (Bound::Alpha, Bound::Alpha)
        )
    }
}
