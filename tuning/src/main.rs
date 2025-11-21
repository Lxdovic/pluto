use core::f32;
use engine::eval::Eval;
use parser::EpdParser;
use sample::Sample;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use tunecoef::TuneCoef;
use tuner::Tuner;

mod outcome;
mod param;
mod parser;
mod sample;
mod tunecoef;
mod tuner;

const EPOCHS: usize = 10;
const LR: f32 = 0.25;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} epd_file x", args[0]);
        return;
    }

    let mut coefs: Vec<TuneCoef> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let mut weights_indices: HashSet<i32> = HashSet::new();

    let samples = load_epd(&args[1], &mut coefs, &mut indices, &mut weights_indices);

    // let params = Tuner::init_params();
    // let coefficients = vec![(1.0, 1.0); params.len()];

    // for s in samples.iter().take(200) {
    //     let tuner_eval = Tuner::eval(&params, s, &coefficients) as i32;
    //     let engine_eval = Eval::eval(&s.pos);
    //
    //     if tuner_eval.abs() != engine_eval.abs() {
    //         println!(
    //             "Wrong eval: Tuner Eval: {}, Engine Eval: {}",
    //             tuner_eval, engine_eval,
    //         );
    //     }
    // }

    Tuner::tune(samples, &coefs, &indices, weights_indices, EPOCHS, LR);
}

fn load_epd(
    path: &str,
    coefs: &mut Vec<TuneCoef>,
    indices: &mut Vec<u16>,
    weights_indices: &mut HashSet<i32>,
) -> Vec<Sample> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    EpdParser::parse(reader.lines(), coefs, indices, weights_indices)
}

//
// // #[derive(Clone)]
// // struct Sample {
// //     pos: Chess,
// //     outcome: KnownOutcome,
// //     phase: f32,
// //     /// represents the indices of each eval feature evaulated in the position
// //     coefs: [Vec<TuneCoef>; 2],
// // }
//
// #[derive(Debug, Clone)]
// enum EvalPhase {
//     MG,
//     EG,
// }
//
// #[derive(Debug, Clone)]
// struct TuneCoef {
//     pub tunable: bool,
//     pub color: Color,
//     pub index: usize,
// }
//
// impl TuneCoef {
//     pub fn new(color: Color, index: usize, tunable: bool) -> Self {
//         Self {
//             tunable,
//             color,
//             index,
//         }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct TuneParam(pub f32);
//
// impl Display for TuneParam {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// enum InitMode {
//     Initial,
//     Random,
//     Zero,
// }
//
// fn load_params(mode: InitMode) -> Vec<TuneParam> {
//     let mut params = Vec::new();
//
//     // from 0 to 9
//     for v in PIECE_VALUES.iter().take(5) {
//         let (mg, eg) = (extract_mg(*v), extract_eg(*v));
//
//         params.push(TuneParam(mg as f32));
//         params.push(TuneParam(eg as f32));
//     }
//
//     // from 10 to 778
//     PSQTS.map(|p| {
//         p.map(|v| {
//             let (mg, eg) = match mode {
//                 InitMode::Random => (
//                     thread_rng().gen_range(0..100),
//                     thread_rng().gen_range(0..100),
//                 ),
//                 InitMode::Zero => (0, 0),
//                 InitMode::Initial => (extract_mg(v), extract_eg(v)),
//             };
//
//             params.push(TuneParam(mg as f32));
//             params.push(TuneParam(eg as f32));
//         })
//     });
//
//     params
// }
//
// // const EPOCHS: u16 = 100;
// const LR: f32 = 0.001;
// const BATCH_SIZE: usize = 128;
//
// fn eval(sample: &Sample, params: &[TuneParam]) -> f32 {
//     let mut mg = 0.0;
//     let mut eg = 0.0;
//
//     let phase = Eval::phase(&sample.pos) as f32;
//
//     for coef in sample.coefs[0].iter() {
//         let c = match coef.color {
//             Color::White => 1.0,
//             Color::Black => -1.0,
//         };
//
//         mg += params[coef.index].0 * c;
//     }
//
//     for coef in sample.coefs[1].iter() {
//         let c = match coef.color {
//             Color::White => 1.0,
//             Color::Black => -1.0,
//         };
//
//         eg += params[coef.index].0 * c;
//     }
//
//     mg * phase + eg * (24.0 - phase)
// }
//
// fn average_error(k: f32, samples: &[Sample], params: &[TuneParam]) -> f32 {
//     let mut err = 0.0;
//
//     for sample in samples {
//         let eval = eval(sample, params);
//         let sig = sigmoid(eval, k);
//
//         err += (sample.wdl - sig).powi(2);
//     }
//
//     err / samples.len() as f32
// }
//
// fn find_k(samples: &[Sample], params: &[TuneParam]) -> f32 {
//     let rate = 10.0;
//     let delta = 1e-5;
//     let deviation_goal = 1e-6;
//     let mut deviation: f32 = 1.0;
//     let mut k = 1.0;
//
//     while deviation.abs() > deviation_goal {
//         let up = average_error(k + delta, samples, params);
//         let down = average_error(k - delta, samples, params);
//
//         deviation = (up - down) / (2.0 * delta);
//
//         k -= deviation * rate;
//         println!(
//             "Current K: {}, up: {}, down: {}, deviation: {}",
//             k, up, down, deviation
//         );
//     }
//
//     k
// }
//
// fn tune(samples: &mut [Sample]) {
//     let mut params = load_params(InitMode::Random);
//     let mut best_parms = params.clone();
//     let mut best_loss = 1.0;
//     let mut best_loss_idx = 0;
//     let mut idx = 0;
//     let initial_params = params.clone();
//
//     println!("Finding optimal K...");
//     let k = find_k(samples, &params);
//     println!("Optimal value for k found: {}", k);
//
//     loop {
//         samples.shuffle(&mut thread_rng());
//
//         let mut total_loss = 0.0;
//         let mut count = 0;
//
//         for batch in samples.chunks(BATCH_SIZE) {
//             for sample in batch {
//                 let eval = eval(sample, &params);
//                 let sig = sigmoid(eval, k);
//                 let res = (sample.wdl - sig) * sig * (1.0 - sig);
//
//                 let mg = res * (sample.phase / 24.0);
//                 let eg = res - mg;
//
//                 // println!(
//                 //     "diff {}, result {}, sig {}, eval {}",
//                 //     diff, sample.result, sig, eval
//                 // );
//
//                 let loss = res.powi(2);
//                 total_loss += loss;
//                 count += 1;
//
//                 for coef in sample.coefs[0].iter() {
//                     if !coef.tunable {
//                         continue;
//                     }
//
//                     params[coef.index].0 += mg * params[coef.index].0;
//                 }
//
//                 for coef in sample.coefs[1].iter() {
//                     if !coef.tunable {
//                         continue;
//                     }
//
//                     params[coef.index].0 += eg * params[coef.index].0;
//                 }
//             }
//         }
//
//         if (total_loss / count as f32) < best_loss {
//             best_loss = total_loss / count as f32;
//             best_loss_idx = idx;
//             best_parms = params.clone();
//         } else if idx > best_loss_idx + 3 {
//             break;
//         }
//
//         if idx % 10 == 0 {
//             for r in Role::ALL {
//                 if r == Role::King {
//                     continue;
//                 };
//
//                 let index = (r as usize - 1) * 2;
//                 println!(
//                     "{:?}: [MG: {}, EG: {}]",
//                     r,
//                     params[index],
//                     params[index + 1]
//                 )
//             }
//
//             for role in Role::ALL {
//                 println!("{:?}:", role);
//
//                 let offset = 10;
//                 let role_index = role as usize - 1;
//
//                 for i in 0..128 {
//                     let index = offset + role_index * 128 + i;
//                     let initial = initial_params[index].0 as i32;
//                     let actual = params[index].0 as i32;
//
//                     if i % 16 == 0 {
//                         println!();
//                     }
//                     if i % 2 == 0 {
//                         print!("({:>4}({:>3}),", actual, actual - initial)
//                     } else {
//                         print!("{:>4}({:>3})), ", actual, actual - initial)
//                     }
//                 }
//                 println!("\n");
//             }
//         }
//
//         println!("EPOCH {} LOSS: {:.6}", idx, total_loss / count as f32);
//
//         idx += 1;
//     }
//
//     for r in Role::ALL {
//         if r == Role::King {
//             continue;
//         };
//
//         let index = (r as usize - 1) * 2;
//         println!(
//             "{:?}: [MG: {}, EG: {}]",
//             r,
//             params[index],
//             params[index + 1]
//         )
//     }
//
//     for role in Role::ALL {
//         println!("{:?}:", role);
//
//         let offset = 10;
//         let role_index = role as usize - 1;
//
//         for i in 0..128 {
//             let index = offset + role_index * 128 + i;
//             let actual = best_parms[index].0 as i32;
//
//             if i % 16 == 0 {
//                 println!();
//             }
//             if i % 2 == 0 {
//                 print!("({:>4},", actual)
//             } else {
//                 print!("{:>4}), ", actual)
//             }
//         }
//         println!("\n");
//     }
// }
//
// fn load_epd(path: &str) -> Vec<Sample> {
//     let file = File::open(path).unwrap();
//     let reader = BufReader::new(file);
//
//     EpdParser::parse(reader.lines())
//
//     // for line in reader.lines().take(1000000).map(|l| l.unwrap()) {
//     let parts: Vec<&str> = line.split(";").collect();
//     let mut result;
//     let pos: Chess = Fen::from_str(parts[0].trim())
//         .unwrap()
//         .into_position(shakmaty::CastlingMode::Standard)
//         .unwrap();
//
//     for comment in parts.iter().skip(1) {
//         if let Some(start) = comment.find("result:") {
//             let rest = &comment[start + "result:".len()..];
//
//             result = match rest.trim_matches(|c| c == ' ' || c == '"') {
//                 "1-0" => KnownOutcome::Decisive {
//                     winner: Color::White,
//                 },
//                 "1/2-1/2" => KnownOutcome::Draw,
//                 "0-1" => KnownOutcome::Decisive {
//                     winner: Color::Black,
//                 },
//                 _ => panic!("Result is invalid: {}", rest),
//             };
//         }
//     }
//
//     let phase = Eval::phase(&pos) as f32;
//
//     let mut coefs: [Vec<TuneCoef>; 2] = [const { Vec::new() }; 2];
//
//     for (sq, piece) in pos.board() {
//         // * 2 because we have mg and eg values
//         let role_index = (piece.role as usize - 1) * 2;
//         let sq_index = sq as usize;
//
//         if piece.role != Role::King {
//             coefs[0].push(TuneCoef::new(piece.color, role_index, false)); // piece value mg
//             coefs[1].push(TuneCoef::new(piece.color, role_index + 1, false)); // piece value eg
//         }
//
//         let psqt_index = match piece.color {
//             // 10 to start after value index (10 because we're not tuing static king value)
//             // * 2 again for mg & eg
//             Color::White => 10 + role_index * 64 + (sq_index ^ 56) * 2,
//             Color::Black => 10 + role_index * 64 + sq_index * 2,
//         };
//
//         // 0 = mg, 1 = eg
//         coefs[0].push(TuneCoef::new(piece.color, psqt_index, true));
//         coefs[1].push(TuneCoef::new(piece.color, psqt_index + 1, true));
//     }
//
//     // samples.push(Sample {
//     //     pos,
//     //     wdl: result,
//     //     phase,
//     //     coefs,
//     // });
// }
//
// }
