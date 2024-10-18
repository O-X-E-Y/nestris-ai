use std::io::Write;

use crate::prelude::*;

use itertools::Itertools;
use nanorand::{Rng, WyRand};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvalWeights {
    pub hole_multiplier: i32,
    pub over_three_multiplier: i32,
    pub highest_point_multiplier: i32,
    pub tetris_ready: i32,
    pub col_weights: [i32; 10],
    pub well_height: [i32; 20],
    pub line_multipliers: [i32; 4],
    pub surface: Vec<i32>
}

impl Default for EvalWeights {
    fn default() -> Self {
        let surface = (0..37).into_iter().map(|_| 0i32).collect();
        Self {
            hole_multiplier: Default::default(),
            over_three_multiplier: Default::default(),
            highest_point_multiplier: Default::default(),
            tetris_ready: Default::default(),
            col_weights: Default::default(),
            well_height: Default::default(),
            line_multipliers: Default::default(),
            surface
        }
    }
}

impl EvalWeights {
    // #[time_this::time_this]
    pub fn optimize_from_random() -> EvalWeights {
        let mut rng = WyRand::new();

        let piece_sequence = time_this::time!((0..u16::MAX)
            .into_iter()
            .map(|_| Piece::PIECES[rng.generate_range(0..7)].start_pos())
            .collect::<Vec<_>>());

        let to_optimize = 5;

        let mut weights = (0..to_optimize)
            .into_iter()
            .map(|_| EvalWeights::random(&mut rng, 48000))
            .collect::<Vec<_>>();

        let mut best = (0.0, weights[0].clone());

        let generations = 2000;

        for generation in 0..generations {

            if generation % 500 == 0 {
                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("./eval_weights_new.json")
                    .unwrap();

                let s = serde_json::to_string_pretty(&best.1).unwrap();
    
                f.write_all(s.as_bytes()).unwrap();
            }

            let seed = weights
                .iter()
                .map(|w| {
                    let mut i = rng.generate_range(0..piece_sequence.len());
                    
                    let mut score = 0;

                    for _ in 0..45 {
                        let mut lines = 1;
                        let mut game = State::new(piece_sequence[i].piece, 18, INPUT_30HZ, w);
                        
                        while !game.is_topped_out() {
                            let states = game.search_drop_first_specialized();
    
                            let (_, best_state) = states
                                .clone()
                                .into_iter()
                                .map(|state| {
                                    let mut cloned_state = game.clone();
                                    cloned_state.pos = state;
                                    cloned_state.lock();
                                    (cloned_state.eval_board(), cloned_state)
                                })
                                .sorted_by(|(e1, _), (e2, _)| e1.cmp(&e2))
                                .next()
                                .unwrap();
    
                            game = best_state;
                            i += 1;
                            i %= piece_sequence.len();
                            game.pos = piece_sequence[i];
                        }

                        score += game.score;
                        lines += game.lines;

                        if lines >= 250 {
                            // println!("reached 250 lines!!");
                            break;
                        }
                    }

                    ((score as f64) / 45.0, w)

                })
                .sorted_by(|(e1, _), (e2, _)| e2.total_cmp(&e1))
                .next()
                .unwrap();

            if seed.0 > best.0 {
                best = (seed.0, seed.1.clone());
            }
            println!("generation {generation} scored {}", best.0);

            weights = std::iter::repeat_n(&best, to_optimize)
                .map(|(_, w)| w.clone().random_neighbour(&mut rng, generation + 1, generations))
                .collect();
        }

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("./eval_weights_new.json")
            .unwrap();

        let s = serde_json::to_string_pretty(&best.1).unwrap();

        f.write_all(s.as_bytes()).unwrap();

        best.1
    }

    pub fn optimize(self) -> EvalWeights {
        let mut rng = WyRand::new();

        let piece_sequence = time_this::time!((0..u16::MAX)
            .into_iter()
            .map(|_| Piece::PIECES[rng.generate_range(0..7)].start_pos())
            .collect::<Vec<_>>());

        let to_optimize = 80;

        let mut weights = (0..to_optimize)
            .into_iter()
            .map(|_| self.clone())
            .collect::<Vec<_>>();
        
        let mut seeds = Vec::with_capacity(to_optimize);

        let mut best = (0.0, weights[0].clone());

        let generations = 400;

        for generation in 0..generations {

            if generation % (generations / 10) == 0 {
                let mut f = std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open("./eval_weights_new.json")
                    .unwrap();

                let s = serde_json::to_string_pretty(&best.1).unwrap();
    
                f.write_all(s.as_bytes()).unwrap();
            }

            let intermediate = weights
                .into_iter()
                .map(|w| (w, rng.generate_range(0..piece_sequence.len())))
                .collect::<Vec<_>>();

            intermediate
                .into_par_iter()
                .map(|(w, mut i)| {
                    let mut score = 0;

                    for _ in 0..100 {
                        let mut lines = 1;
                        let mut game = State::new(piece_sequence[i].piece, 18, INPUT_30HZ, &w);
                        
                        while !game.is_topped_out() {
                            let states = game.search_drop_first_specialized();

                            let score = game.score;
                            let highest = game.highest_block;
                            let surface = game.surface;
                            let lines = game.lines;
                            let delta = game.lines_delta;
    
                            let (_, best_pos) = states
                                .into_iter()
                                .map(|pos| {
                                    game.pos = pos;
                                    game.fast_lock();

                                    let eval = game.eval_board();

                                    game.score = score;
                                    game.highest_block = highest;
                                    game.surface = surface;
                                    game.lines = lines;
                                    game.lines_delta = delta;

                                    (eval, pos)
                                })
                                .sorted_by(|(e1, _), (e2, _)| e1.cmp(&e2))
                                .next()
                                .unwrap();
    
                            game.pos = best_pos;
                            game.lock();
                            i += 1;
                            i %= piece_sequence.len();
                            game.pos = piece_sequence[i];
                        }

                        score += game.score;
                        lines += game.lines;

                        if lines >= 250 {
                            // println!("reached 250 lines!!");
                            break;
                        }
                    }

                    ((score as f64) / 100.0, w)
                })
                .collect_into_vec(&mut seeds);

            seeds.sort_by(|(e1, _), (e2, _)| e2.total_cmp(&e1));

            if seeds[0].0 > best.0 {
                best = (seeds[0].0, seeds[0].1.clone());
            }
            println!("generation {generation} scored {}", best.0);

            weights = seeds
                .iter()
                .take(10)
                .cycle()
                .take(to_optimize)
                .map(|(_, w)| w.random_neighbour(&mut rng, generation + 1, generations))
                .collect();
        }

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("./eval_weights_new.json")
            .unwrap();

        let s = serde_json::to_string_pretty(&best.1).unwrap();

        f.write_all(s.as_bytes()).unwrap();

        best.1
    }

    fn random_neighbour(&self, rng: &mut WyRand, generation: usize, generations: usize) -> Self {
        let changes = 10;
        let steps = 1000.0;
        let initial_diff = steps - generation as f64 * (0.98 * steps / generations as f64) * 2.0;

        let mut res = self.clone();
        
        for _ in 0..(changes - (generation / (generations / changes))).max(1) {
            let diff = (initial_diff * (rng.generate::<f64>() - 0.5)) as i32;

            match rng.generate_range(0..74) {
                0 => res.hole_multiplier += diff,
                1 => res.over_three_multiplier += diff,
                i @ 2..12 => res.col_weights[i - 2] += diff,
                i @ 12..32 => res.well_height[i - 12] += diff,
                i @ 32..36 => res.line_multipliers[i - 32] += diff,
                i @ 36..73 => res.surface[i - 36] += diff,
                73 => res.highest_point_multiplier += diff,
                _ => unreachable!(),
            }
        }
        
        res
    }

    fn random(rng: &mut WyRand, range: i32) -> Self {
        let mut w = Self::default();

        w.hole_multiplier = rng.generate_range(0..range) - range / 2;
        w.over_three_multiplier = rng.generate_range(0..range) - range / 2;
        for i in 0..(w.col_weights.len()) {
            w.col_weights[i] = rng.generate_range(0..range) - range / 2;
        }
        for i in 0..(w.well_height.len()) {
            w.well_height[i] = rng.generate_range(0..range) - range / 2;
        }
        for i in 0..(w.line_multipliers.len()) {
            w.line_multipliers[i] = rng.generate_range(0..range) - range / 2;
        }
        for i in 0..(w.surface.len()) {
            w.surface[i] = rng.generate_range(0..range) - range / 2;
        }
        w.highest_point_multiplier = rng.generate_range(0..range) - range / 2;

        w
    }
}

#[test]
fn thing() {
    let mut rng = WyRand::new();

    for _ in 0..25 {
        println!("{}", (rng.generate::<f64>() - 0.5) * 4.0);
    }
}
