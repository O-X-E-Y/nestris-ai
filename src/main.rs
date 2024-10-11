// #![feature(const_mut_refs)]

use itertools::Itertools;
// use nestris_ai::{const_arrayvec::ArrayVec, consts::input_hz::*, pieces::*, state::*};

fn read_input() {
    let _ = std::io::stdin().read_line(&mut String::new());
}

pub trait PrintBitsNum {
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

use nestris_ai::{consts::INPUT_10HZ, pieces::Piece, state::State};

pub fn preset_state(piece: Piece) -> State<'static> {
    let seq = INPUT_10HZ;
    let mut c = State::new(piece, 29, &seq);

    c.try_ccw();
    c.try_left();
    c.try_left();
    c.try_left();
    c.try_left();
    c.try_left();
    c.drop();

    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_right();
    c.try_right();
    c.try_right();
    c.try_cw();
    c.try_cw();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_right();
    c.try_right();
    c.try_cw();
    c.try_cw();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_left();
    c.try_left();
    c.try_left();
    c.drop();
    c.lock();
    c.pos = piece.start_pos();

    c.pos = piece.start_pos();
    c.try_left();
    c.try_cw();
    c.try_cw();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_cw();
    c.try_cw();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_right();
    c.try_right();
    c.try_right();
    c.try_right();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_left();
    c.try_left();
    c.try_left();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_right();
    c.try_cw();
    c.try_cw();
    c.drop();
    c.lock();

    c.pos = piece.start_pos();
    c.try_left();
    c.try_left();
    c.try_left();
    c.drop();
    c.lock();

    c
}

fn messing() {
    // for p in Piece::PIECES {
    let seq = INPUT_10HZ;
    let mut c = State::new(Piece::T, 29, &seq);


    let mut rng = nanorand::WyRand::new();
    use nanorand::Rng;

    loop {
        let states = c.search_drop_first_specialized();
    
        let (score, best_state) = states
            .clone()
            .into_iter()
            .map(|state| {
                let mut cloned_state = c.clone();
                cloned_state.pos = state;
                cloned_state.lock();
                (cloned_state.eval_board(), cloned_state)
            })
            .sorted_by(|(e1, _), (e2, _)| e1.cmp(&e2))
            .next()
            .unwrap();
    
        c = best_state;
        c.pos = Piece::PIECES[rng.generate_range(0..7)].start_pos();
        
        println!("best state:\n{c}\nscore: {score}\nsurface: {:?}\n", c.surface);

        read_input();
    }


    // println!("surface: {:?}", c.surface);
}
