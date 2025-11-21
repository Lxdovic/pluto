use std::{collections::HashSet, time::SystemTime};

use engine::{
    eval::{Eval, PIECE_VALUES, PSQTS},
    packing::{extract_eg, extract_mg},
};
use shakmaty::{Color, Position, Role};

use crate::{
    param::{Param, TunerParam},
    sample::Sample,
    tunecoef::TuneCoef,
};

pub struct Tuner {
    params: Vec<Param>,
    samples: Vec<Sample>,
    k: f32,
}
const B1: f32 = 0.9;
const B2: f32 = 0.999;

const GAME_PHASES: [f32; 6] = [0.0, 1.0, 1.0, 2.0, 4.0, 0.0];

pub fn sigmoid(eval: f32, k: f32) -> f32 {
    1.0 / (1.0 + (-k * eval / 400.0).exp())
}

impl Tuner {
    pub fn init_params() -> Vec<(f32, f32)> {
        let mut params: Vec<(f32, f32)> = Vec::new();

        // 0 -> 5
        for r in Role::ALL {
            let role_index = r as usize - 1;
            let value = PIECE_VALUES[role_index];

            params.push((extract_mg(value) as f32, extract_eg(value) as f32));
        }

        // 6 -> 390
        for r in Role::ALL {
            let role_index = r as usize - 1;

            for sq in 0..64 {
                let value = PSQTS[role_index][sq];

                params.push((extract_mg(value) as f32, extract_eg(value) as f32));
            }
        }

        params
    }

    fn optimal_k(samples: &[Sample], params: &[(f32, f32)], coefficients: &[(f32, f32)]) -> f32 {
        let mut k = 2.5;
        let delta = 1e-5;
        let deviation_goal = 1e-6;
        let rate = 10.0;
        let mut deviation: f32 = 1.0;

        println!("Finding optimal K...");

        while deviation.abs() > deviation_goal {
            let up = Self::error(k + delta, params, samples, coefficients);
            let down = Self::error(k - delta, params, samples, coefficients);

            deviation = (up - down) / (2.0 * delta);

            k -= deviation * rate;

            println!(
                "K: {}, up: {}, down: {}, deviation: {}",
                k, up, down, deviation
            );
        }

        println!("Found Optimal K: {}", k);

        k
    }

    pub fn eval(
        sample: &Sample,
        indices: &[u16],
        weights: &[f32],
        coefficients: &[TuneCoef],
    ) -> f32 {
        let mut mg = 0.0;
        let mut eg = 0.0;
        let phase = (sample.phase / 24) as f32;

        for i in 0..sample.coeffs_count {
            let index = sample.base_index as usize + i as usize;
            let value = weights[indices[index] as usize] * coefficients[index].value as f32;

            if indices[index] < 6 {
                mg += value;
                eg += value;
            } else if phase == 0.0 {
                mg += value;
            } else {
                eg += value;
            }
        }

        (mg * phase) + (eg * (1.0 - phase))

        // for (sq, piece) in sample.pos.board() {
        //     let role_idx = piece.role as usize - 1;
        //     let sq_idx = sq as usize;
        //
        //     let (mg_val, eg_val) = match piece.color {
        //         Color::White => {
        //             let psqt_index = 6 + role_idx * 64 + (sq_idx ^ 56);
        //
        //             (
        //                 params[role_idx].0 + coefficients[psqt_index].0 * params[psqt_index].0,
        //                 params[role_idx].1 + coefficients[psqt_index].1 * params[psqt_index].1,
        //             )
        //         }
        //         Color::Black => {
        //             let psqt_index = 6 + role_idx * 64 + sq_idx;
        //
        //             (
        //                 -(params[role_idx].0 + coefficients[psqt_index].0 * params[psqt_index].0),
        //                 -(params[role_idx].1 + coefficients[psqt_index].1 * params[psqt_index].1),
        //             )
        //         }
        //     };
        //
        //     mg += mg_val;
        //     eg += eg_val;
        //     phase += GAME_PHASES[role_idx];
        // }
        //
        // (mg * phase + eg * (24.0 - phase)) / 24.0
    }

    pub fn error(
        k: f32,
        samples: &[Sample],
        indices: &[u16],
        weights: &[f32],
        coefficients: &[TuneCoef],
    ) -> f32 {
        let mut err = 0.0;

        for sample in samples {
            let score = Self::eval(sample, indices, weights, coefficients);
            let diff = sample.outcome.0 - sigmoid(score, k);

            err += diff.powi(2);
        }

        err / samples.len() as f32
    }

    fn compute_gradient(
        gradient: &mut [(f32, f32)],
        samples: &[Sample],
        params: &[(f32, f32)],
        coefficients: &[(f32, f32)],
        k: f32,
    ) {
        for sample in samples {
            Self::update_single_gradient(gradient, sample, params, coefficients, k);
        }

        print!("Gradient: \n[");

        for (i, g) in gradient.iter().skip(6).enumerate() {
            if i % 8 == 0 {
                println!();
            }

            print!("({:>3}, {:>3}), ", g.0 as i32, g.1 as i32);
        }

        println!();
    }

    fn update_single_gradient(
        gradient: &mut [(f32, f32)],
        sample: &Sample,
        params: &[(f32, f32)],
        coefficients: &[(f32, f32)],
        k: f32,
    ) {
        let eval = Self::eval(params, sample, coefficients);
        let sig = sigmoid(eval, k);
        let res = (sample.outcome.0 - sig) * sig * (1.0 - sig);
        let phase = Eval::phase(&sample.pos) as f32 / 24.0;

        let mg_base = res * phase;
        let eg_base = res - mg_base;

        for (sq, piece) in sample.pos.board() {
            let role_idx = piece.role as usize - 1;
            let sq_idx = sq as usize;

            let index = match piece.color {
                Color::White => 6 + role_idx * 64 + (sq_idx ^ 56),
                Color::Black => 6 + role_idx * 64 + sq_idx,
            };

            gradient[index].0 += mg_base * coefficients[index].0;
            gradient[index].1 += eg_base * coefficients[index].1;
        }
    }

    fn load_params() -> Vec<TunerParam> {
        let mut params = Vec::new();

        for pv in PIECE_VALUES {
            params.push(TunerParam::new(pv, pv, pv, pv, pv));
        }

        for parameter in &mut params {
            parameter.value = parameter.value.clamp(parameter.min, parameter.max)
        }

        params
    }

    pub fn tune(
        samples: Vec<Sample>,
        coefficients: &[TuneCoef],
        indices: &[u16],
        weights_indices: HashSet<i32>,
        epochs: usize,
        lr: f32,
    ) {
        let start_time = SystemTime::now();
        let mut weights = Vec::new();
        let mut gradients = Vec::new();

        let params = Self::load_params();

        for parameter in params {
            weights.push(parameter.value as f32);
        }

        gradients.resize(weights.len(), 0.0);

        let mut m = Vec::new();
        m.resize(weights.len(), 0.0);

        let mut v = Vec::new();
        v.resize(weights.len(), 0.0);

        drop(weights_indices);

        let k = 0.0005;

        let mut last_err = Self::error(k, &samples, indices, &weights, coefficients);
        let mut itr = 0;

        loop {
            gradients.fill(0.0);

            for sample in samples.iter() {
                let result = sample.outcome.0;
                let pos_phase = sample.phase as f32 / 24.0;
                let eval = Self::eval(&sample, indices, &weights, coefficients);

                let sig = sigmoid(eval, k);
                let err = result - sig;

                for i in 0..sample.coeffs_count {
                    let index = sample.base_index as usize + i as usize;

                    if indices[index] < 6 {
                        continue;
                    }

                    let value = coefficients[index].value;
                    let phase = coefficients[index].phase;

                    let phase = if phase == 0 {
                        pos_phase
                    } else {
                        1.0 - pos_phase
                    };

                    let c = phase * value as f32;
                    gradients[indices[index] as usize] += err * c;
                }
            }

            for i in 6..weights.len() {
                let gradient = -2.0 * gradients[i] / samples.len() as f32;

                m[i] = B1 * m[i] + (1.0 - B1) * gradient;
                v[i] = B2 * v[i] + (1.0 - B2) * gradient.powi(2);

                weights[i] -= lr * m[i] / (v[i] + 0.00000001).sqrt();
                weights[i] = weights[i].clamp(params[i].min as f32, params[i].max as f32);
            }

            if itr % 10 == 0 {
                for i in 0..params.len() {
                    params[i].value = weights[i].round() as i32;
                }

                let mut weights_iter = weights.iter().skip(6);
                let error = Self::error(k, &samples, &indices, &weights, &coefficients);

                write_evaluation_parameters(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    wdl_ratio,
                );
                write_piece_square_table(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    "PAWN",
                    PIECE_VALUES[PAWN],
                );
                write_piece_square_table(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    "KNIGHT",
                    PIECE_VALUES[KNIGHT],
                );
                write_piece_square_table(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    "BISHOP",
                    PIECE_VALUES[BISHOP],
                );
                write_piece_square_table(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    "ROOK",
                    PIECE_VALUES[ROOK],
                );
                write_piece_square_table(
                    &mut weights_iter,
                    output_directory,
                    error,
                    k,
                    "QUEEN",
                    PIECE_VALUES[QUEEN],
                );
                write_piece_square_table(&mut weights_iter, output_directory, error, k, "KING", 0);

                if weights_iter.next().is_some() {
                    panic!("Weights iterator has not ended properly");
                }

                println!(
                    "Iteration {} done in {} seconds, error reduced from {:.6} to {:.6} ({:.6})",
                    itr,
                    (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
                    last_err,
                    error,
                    last_err - error
                );

                last_err = error;
                start_time = SystemTime::now();
            }

            itr += 1;
        }
        //     let parameters = Self::init_params();
        //     let mut coefficients = vec![(1.0, 1.0); parameters.len()];
        //     let k = Self::optimal_k(&samples, &parameters, &coefficients);
        //     let avg_error = Self::error(k, &parameters, &samples, &coefficients);
        //     let mut parameters = vec![(0.0, 0.0); parameters.len()];
        //
        //     println!("Initial Average Error: {}", avg_error);
        //
        //     let mut momentum = vec![(0.0, 0.0); parameters.len()];
        //     let mut velocity = vec![(0.0, 0.0); parameters.len()];
        //
        //     for epoch in 0..epochs {
        //         let mut gradient = vec![(0.0, 0.0); parameters.len()];
        //
        //         Self::compute_gradient(&mut gradient, &samples, &parameters, &coefficients, k);
        //
        //         let beta1 = 0.9;
        //         let beta2 = 0.999;
        //
        //         for i in 0..parameters.len() {
        //             let mg_grad = -k / 400.0 * gradient[i].0 / samples.len() as f32;
        //
        //             momentum[i].0 = beta1 * momentum[i].0 + (1.0 - beta1) * mg_grad;
        //             velocity[i].0 = beta2 * velocity[i].0 + (1.0 - beta2) * mg_grad.powi(2);
        //             parameters[i].0 -= lr * momentum[i].0 / (1e-8 + velocity[i].0.sqrt());
        //
        //             let eg_grad = -k / 400.0 * gradient[i].1 / samples.len() as f32;
        //
        //             momentum[i].1 = beta1 * momentum[i].1 + (1.0 - beta1) * eg_grad;
        //             velocity[i].1 = beta2 * velocity[i].1 + (1.0 - beta2) * eg_grad.powi(2);
        //             parameters[i].1 -= lr * momentum[i].1 / (1e-8 + velocity[i].1.sqrt());
        //         }
        //
        //         let avg_error = Self::error(k, &parameters, &samples, &coefficients);
        //
        //         println!("Epoch: {}/{}", epoch, epochs);
        //         println!("Initial Average Error: {}", avg_error);
        //     }
        // }

        //     for epoch in 0..epochs {
        //         let mut curr_loss = 0.0;
        //
        //         for s in self.samples.iter() {
        //             let eval = Self::eval(&self.params, s);
        //             let sig = sigmoid(eval, self.k);
        //             let res = (s.outcome.0 - sig) * sig * (1.0 - sig);
        //             let phase = Eval::phase(&s.pos) as f32 / 24.0;
        //
        //             let mg_base = res * phase;
        //             let eg_base = res - mg_base;
        //
        //             curr_loss += res;
        //
        //             for (sq, piece) in s.pos.board() {
        //                 let role_idx = piece.role as usize - 1;
        //                 let sq_idx = sq as usize;
        //
        //                 match piece.color {
        //                     Color::White => (
        //                         self.params[6 + role_idx * 64 + (sq_idx ^ 56)].mg -=
        //                             0.25 * (-self.k / 400.0 * mg_base / self.params.len() as f32),
        //                         self.params[6 + role_idx * 64 + (sq_idx ^ 56)].eg -=
        //                             0.25 * (-self.k / 400.0 * eg_base / self.params.len() as f32),
        //                     ),
        //                     Color::Black => (
        //                         self.params[6 + role_idx * 64 + sq_idx].mg -=
        //                             0.25 * (-self.k / 400.0 * mg_base / self.params.len() as f32),
        //                         self.params[6 + role_idx * 64 + sq_idx].eg -=
        //                             0.25 * (-self.k / 400.0 * eg_base / self.params.len() as f32),
        //                     ),
        //                 };
        //             }
        //         }
        //
        //         println!(
        //             "Epoch: {}/{}\nLoss: {}",
        //             epoch,
        //             epochs,
        //             curr_loss / self.samples.len() as f32,
        //         );
        //
        //         if epoch % 10 == 0 {
        //             for r in Role::ALL {
        //                 println!("{:?}:", r);
        //
        //                 for sq in 0..64 {
        //                     let role_idx = r as usize - 1;
        //                     let index = 6 + role_idx * 64 + sq;
        //
        //                     if sq % 8 == 0 {
        //                         println!();
        //                     }
        //                     print!(
        //                         "({:>3},{:>3}), ",
        //                         self.params[index].mg as i32, self.params[index].eg as i32
        //                     );
        //                 }
        //             }
        //
        //             println!();
        //         }
        //     }
    }
}
