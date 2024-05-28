#![allow(dead_code)]

use std::hint::black_box;

use diol::prelude::*;
use itertools::Itertools;
use nestris_ai::prelude::*;

fn main() -> std::io::Result<()> {
    let pieces_rot = Piece::PIECES
        .into_iter()
        .cartesian_product(Rotation::ROTATIONS)
        .collect::<Vec<_>>();

    let mut bench = Bench::new(BenchConfig::from_args()?);
    bench.register_many(
        list![
            search_naive,
            // search_specialized,
            // search_select_rot,
            // search_const,
            search_smart,
            search_visited_first,
        ],
        Piece::PIECES,
    );
    // bench.register(search_depth_3, Piece::PIECES);
    // bench.register_many(
    //     list![search_depth_final_states],
    //     [
    //         (Piece::J, 0),
    //         (Piece::J, 1),
    //         (Piece::J, 2),
    //         (Piece::J, 3),
    //         // (Piece::J, 4),
    //     ],
    // );
    // bench.register(lock, pieces_rot);
    // bench.register(clone_state, Piece::PIECES);
    bench.run()?;
    Ok(())
}

fn search_depth_3(bencher: Bencher, piece: Piece) {
    let state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| {
        state.search_rec_visited_first_depth(3);
    })
}

fn search_depth_n(bencher: Bencher, (piece, depth): (Piece, u8)) {
    let state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| {
        state.search_rec_visited_first_depth(depth);
    })
}

fn search_depth_final_states(bencher: Bencher, (piece, depth): (Piece, u8)) {
    let state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| {
        state.search_depth(depth);
    })
}

fn clone_state(bencher: Bencher, piece: Piece) {
    let state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| {
        black_box(state.clone());
    })
}

fn lock(bencher: Bencher, (piece, rot): (Piece, Rotation)) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    match rot {
        Rotation::North => {}
        Rotation::East => assert!(state.try_cw()),
        Rotation::South => {
            assert!(state.try_cw());
            assert!(state.try_cw());
        }
        Rotation::West => assert!(state.try_ccw()),
    }

    for _ in 0..10 {
        assert!(state.try_down());
    }
    let mut state = black_box(state);

    bencher.bench(|| {
        state.lock();
    })
}

fn search_smart(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search_smart())
}

fn search_naive(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search_rec_naive())
}

fn search_specialized(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search_rec_specialized())
}

fn search_const(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search_const())
}

fn search_select_rot(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search_rec_select_rot())
}

fn search_visited_first(bencher: Bencher, piece: Piece) {
    let mut state = State::new(piece, 19, INPUT_30HZ);

    bencher.bench(|| state.search())
}
