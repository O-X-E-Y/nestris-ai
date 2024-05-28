// #![feature(const_mut_refs)]

use nestris_ai::{const_arrayvec::ArrayVec, consts::input_hz::*, pieces::*, state::*};

fn read_input() {
    let _ = std::io::stdin().read_line(&mut String::new());
}

trait PrintBitsNum {
    fn print_bits(self);
}

impl PrintBitsNum for u16 {
    fn print_bits(self) {
        let v = self.reverse_bits();
        print!("0b");
        for i in 0..16 {
            if (v >> i) & 1 == 1 {
                print!("1");
            } else {
                print!("0");
            }
            if i % 4 == 3 && i < 15 {
                print!("_");
            }
        }
        println!();
    }
}

// const STATES: ArrayVec<PiecePos, 100> = State::new(Piece::J, 18).search_iteratively();

fn main() {
    messing();
}

fn messing() {
    // for p in Piece::PIECES {
    let seq = InputSequence::with_tapping_speed(10);
    let mut c = State::new(Piece::I, 29, &seq);

    // c.try_ccw();
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.drop();
    // println!("{c}");

    // c.drop();
    // println!("{c}");
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    c.pos = Piece::L.start_pos();
    c.try_right();
    c.try_right();
    c.try_right();
    c.try_cw();
    c.try_cw();
    c.drop();
    println!("{c}");
    println!("surface: {:?}", c.surface);
    c.lock();
    println!("surface: {:?}", c.surface);

    c.pos = Piece::T.start_pos();
    c.try_right();
    c.try_right();
    c.try_cw();
    c.try_cw();
    c.drop();
    println!("surface: {:?}", c.surface);
    c.lock();
    println!("surface: {:?}", c.surface);

    c.pos = Piece::O.start_pos();
    println!("{c}");
    c.try_left();
    c.try_left();
    c.try_left();
    c.drop();
    println!("surface: {:?}", c.surface);
    c.lock();
    println!("surface: {:?}", c.surface);
    c.pos = Piece::S.start_pos();
    println!("{c}");

    c.pos = Piece::J.start_pos();
    // c.try_left();
    // c.try_cw();
    // c.try_cw();
    // c.drop();
    // println!("{c}");
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::J.start_pos();
    // c.try_cw();
    // c.try_cw();
    // c.drop();
    // println!("{c}");
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::O.start_pos();
    // println!("{c}");
    // c.try_right();
    // c.try_right();
    // c.try_right();
    // c.try_right();
    // c.drop();
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::S.start_pos();
    // println!("{c}");
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.drop();
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::T.start_pos();
    // c.try_right();
    // c.try_cw();
    // c.try_cw();
    // c.drop();
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::L.start_pos();
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.drop();
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);
    // println!("{c}");

    // c.pos = Piece::J.start_pos();
    let boards = c.search_depth(1);
    for b in boards.clone() {
        c.board = b;
        println!("{c}");
    }
    // println!("{}", c.input_sequence);
    println!("found: {:?}", boards.len());
    // c.try_left();
    // c.try_left();
    // c.try_cw();
    // c.try_left();
    // c.try_left();
    // c.try_left();
    // c.drop();
    // println!("{c}");
    // println!("surface: {:?}", c.surface);
    // c.lock();
    // println!("surface: {:?}", c.surface);

    // c.pos = Piece::S.start_pos();
    // println!("{c}");

    // read_input();
    // assert!(c.try_ccw());
    // println!("{c}\n{:?}", c.pos.rot);
    // read_input();
    // assert!(c.try_ccw());
    // println!("{c}\n{:?}", c.pos.rot);
    // read_input();
    // assert!(c.try_ccw());
    // println!("{c}\n{:?}", c.pos.rot);
    // read_input();
    // assert!(c.try_ccw());
    // println!("{c}\n{:?}", c.pos.rot);
    // read_input();
    // let states = c.search_iteratively();

    // for state in states {
    //     c.pos = state;
    //     println!("{c}");
    // }
    // }
}
