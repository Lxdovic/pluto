use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
    str::FromStr,
};

use engine::eval::Eval;
use shakmaty::{Board, Chess, Color, Position, fen::Fen};

use crate::{
    outcome::{OUTCOMES, Outcome},
    sample::Sample,
    tunecoef::TuneCoef,
};

pub struct EpdParser;

impl EpdParser {
    pub fn parse(
        lines: Lines<BufReader<File>>,
        coeffs: &mut Vec<TuneCoef>,
        indices: &mut Vec<i32>,
        weights_indices: &mut HashSet<i32>,
    ) -> Vec<Sample> {
        let mut samples: Vec<Sample> = Vec::new();
        let mut total_lines = 0;

        println!("Parsing positions...");

        for line in lines.map(|l| l.unwrap()).take(1000000) {
            total_lines += 1;

            let parts: Vec<&str> = line.split(";").collect();
            let pos: Chess = Fen::from_str(parts[0].trim())
                .unwrap()
                .into_position(shakmaty::CastlingMode::Standard)
                .unwrap();
            let mut index = 0;
            let base_index = coeffs.len();
            let phase = Eval::phase(&pos);

            Self::get_coefs(pos.board(), &mut index, coeffs, indices);

            for i in base_index..coeffs.len() {
                weights_indices.insert(indices[i]);
            }

            for part in parts.iter().skip(1) {
                if let Some(outcome) = Self::parse_outcome(part, pos.turn()) {
                    samples.push(Sample {
                        outcome,
                        pos: pos.clone(),
                        phase,
                        base_index,
                        coeffs_count: coeffs.len() - base_index,
                    });
                }
            }
        }

        println!(
            "Loaded {} out of {} found positions",
            samples.len(),
            total_lines
        );

        samples
    }

    fn get_coefs(
        board: &Board,
        index: &mut i32,
        coefs: &mut Vec<TuneCoef>,
        indices: &mut Vec<i32>,
    ) {
        let white = board.white();
        let black = board.black();
        let pawns = board.pawns();
        let knights = board.knights();
        let bishops = board.bishops();
        let rooks = board.rooks();
        let queens = board.queens();

        #[rustfmt::skip]
        let coefficients = [
            TuneCoef { value: (pawns.intersect(white).count() - pawns.intersect(black).count()) as i32, phase: 0 },
            TuneCoef { value: (knights.intersect(white).count() - knights.intersect(black).count()) as i32, phase: 0 },
            TuneCoef { value: (bishops.intersect(white).count() - bishops.intersect(black).count()) as i32, phase: 0 },
            TuneCoef { value: (rooks.intersect(white).count() - rooks.intersect(black).count()) as i32, phase: 0 },
            TuneCoef { value: (queens.intersect(white).count() - queens.intersect(black).count()) as i32, phase: 0 },
        ];

        for c in coefficients {
            if c.value != 0 {
                coefs.push(c);
                indices.push(*index);
            }

            *index += 1;
        }
    }

    pub fn parse_outcome(part: &str, turn: Color) -> Option<Outcome> {
        for oc in OUTCOMES {
            if part.contains(oc) {
                let outcome = Outcome::from_str(oc);

                return Some(match turn {
                    Color::White => Outcome(outcome.0),
                    Color::Black => Outcome(1.0 - outcome.0),
                });
            }
        }

        None
    }
}
