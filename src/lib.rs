#![feature(const_fmt_arguments_new, const_option)]

pub mod const_arrayvec;
pub mod consts;
pub mod eval;
pub mod pieces;
pub mod search;
pub mod search_drop_specialized;
pub mod search_other;
pub mod search_smart;
pub mod search_smart2;
pub mod search_visited_alt;
pub mod state;
pub mod util;
pub mod weights;

pub mod prelude {
    pub use crate::{
        consts::{input_hz::*, BOARD_ROWS, EMPTY_BOARD},
        pieces::{Piece, Rotation},
        state::State,
        weights::EvalWeights,
    };
}
