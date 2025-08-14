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

use crate::out;
use std::fmt::{self};

#[derive(Debug)]
pub enum OptionKind {
    Spin,
    String,
    Check,
}

impl OptionKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Spin => "spin",
            Self::String => "string",
            Self::Check => "check",
        }
    }
}

impl fmt::Display for OptionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct OptionDescriptor<T> {
    pub name: &'static str,
    pub kind: OptionKind,
    pub value: T,
    pub min: T,
    pub max: T,
    pub tunable: bool,
}

#[cfg(feature = "tuning")]
macro_rules! impl_fmt_spsa {
    ($($t:ty => $ty_str:expr),*) => {
        $(impl OptionDescriptor<$t> {
            pub fn fmt_spsa(&self) -> String {
                format!(
                    "{}, {}, {}, {}, {}, {}, {}",
                    self.name, $ty_str, self.value, self.min, self.max, 2.25, 0.002
                )
            }
        })*
    };
}

#[cfg(feature = "tuning")]
impl_fmt_spsa!(
    i32 => "int",
    u8 => "int",
    usize => "int",
    i64 => "int",
    u64 => "int",
    f64 => "float",
    bool => "BOOL (ERR, BOOL NOT TUNABLE)"
);

macro_rules! impl_fmt_display {
    ($($t:ty),*) => {
        $(impl fmt::Display for OptionDescriptor<$t> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "option name {} type {} default {} min {} max {}",
                    self.name, self.kind, self.value, self.min, self.max
                )
            }
        })*
    };
}

impl_fmt_display!(i32, u8, usize, i64, u64, f64, bool);

macro_rules! make_config {
    ($($field:ident: $t:ty = ($name:expr, $kind:expr, $val:expr, $min:expr, $max:expr, $tunable:expr);)*) => {
        pub struct Config {
            $(pub $field: OptionDescriptor<$t>,)*
        }

        impl Config {
            pub fn default() -> Self {
                Self {
                    $(
                        $field: OptionDescriptor {
                            name: $name,
                            kind: $kind,
                            value: $val,
                            min: $min,
                            max: $max,
                            tunable: $tunable,
                        },
                    )*
                }
            }

            pub fn set(&mut self, name: &str, val: &str) {
                match name {
                    $(
                        $name => {
                            if !cfg!(feature = "tuning") && self.$field.tunable {
                                out!("info string cannot modify tunable option in non tunable build");
                            }
                            else if let Ok(parsed) = val.parse::<$t>() {
                                self.$field.value = parsed;
                            } else {
                                out!("info string invalid value for {}: {}", $name, val)
                            }
                        },
                    )*
                    _ => out!("info string unknown option: {}", name),
                }
            }

            #[cfg(feature = "tuning")]
            pub fn all_spsa(&self) -> Vec<String> {
                vec![
                    $(self.$field.fmt_spsa(),)*
                ]
            }

            pub fn print_uci_options(&self) {
                $(

                    if cfg!(feature = "tuning") || !self.$field.tunable {
                        out!("{}", self.$field);
                    }
                )*
            }
        }
    };
}

make_config! {
    move_overhead: usize = ("MoveOverhead", OptionKind::Spin, 0, 0, 10000, false);
    threads: u8 = ("Threads", OptionKind::Spin, 1, 1, 1, false);
    hash: usize = ("Hash", OptionKind::Spin, 255, 1, 1024, false);
    silent_uci: bool = ("SilentUCI", OptionKind::Check, false, false, true, false);
    qsearch_depth: u8 = ("QSearchDepth", OptionKind::Spin, 17, 1, 20, true);
    rfp_depth: u8 = ("RFPDepth", OptionKind::Spin, 11, 1, 20, true);
    rfp_base_margin: i32 = ("RFPBaseMargin", OptionKind::Spin, 57, 1, 200, true);
    rfp_reduction_improving: i32 = ("RFPReductionImproving", OptionKind::Spin, 24, 1, 200, true);
    fp_depth_margin: u8 = ("FPDepthMargin", OptionKind::Spin, 5, 1, 20, true);
    fp_base_margin: i32 = ("FPBaseMargin", OptionKind::Spin, 40, 1, 200, true);
    fp_margin_depth_factor: i32 = ("FPMarginDepthFactor", OptionKind::Spin, 35, 1, 200, true);
    nmp_depth: u8 = ("NMPDepth", OptionKind::Spin, 5, 1, 20, true);
    nmp_margin: u8 = ("NMPMargin", OptionKind::Spin, 7, 1, 20, true);
    nmp_divisor: u8 = ("NMPDivisor",OptionKind::Spin, 2, 1, 20, true);
    nmp_divisor_improving: u8 = ("NMPDivisorImproving", OptionKind::Spin, 10, 1, 20, true);
    lmp_move_margin: usize = ("LMPMoveMargin", OptionKind::Spin, 5, 1, 20, true);
    lmp_depth_factor: u8 = ("LMPDepthFactor", OptionKind::Spin, 7, 1, 20, true);
    lmr_depth: u8 = ("LMRDepth", OptionKind::Spin, 15, 1, 20, true);
    lmr_move_margin: usize = ("LMRMoveMargin", OptionKind::Spin, 5, 1, 20, true);
    lmr_quiet_margin: f64 = ("LMRQuietMargin", OptionKind::String, 2.59, 0.0, 10.0, true);
    lmr_quiet_divisor: f64 = ("LMRQuietDivisor", OptionKind::String, 2.02, 1.0, 10.0, true);
    lmr_base_margin: f64 = ("LMRBaseMargin", OptionKind::String, 1.31, 0.0, 10.0, true);
    lmr_base_divisor: f64 = ("LMRBaseDivisor", OptionKind::String, 3.28, 1.0, 10.0, true);
    mo_tt_entry_value: i32 = ("MOTTEntryValue", OptionKind::Spin, 229, 1, 500, true);
    mo_capture_value: i32 = ("MOCaptureValue", OptionKind::Spin, 55, 0, 500, true);
    mo_killer_value: i32 = ("MOKillerValue", OptionKind::Spin, 78, 0, 500, true);
    tc_time_divisor: u64 = ("TCTimeDivisor", OptionKind::Spin, 2, 2, 100, true);
    tc_elapsed_factor: i64 = ("TCElapsedFactor", OptionKind::Spin, 8, 1, 10, true);
}
