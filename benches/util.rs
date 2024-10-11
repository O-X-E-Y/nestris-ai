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

    c.pos = piece.start_pos();
    c
}