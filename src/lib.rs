#![feature(
    const_mut_refs,
    hint_assert_unchecked,
    const_hint_assert_unchecked,
    const_fmt_arguments_new,
    const_option,
    const_slice_first_last,
    const_fn_floating_point_arithmetic
)]

pub mod const_arrayvec;
pub mod consts;
pub mod eval;
pub mod pieces;
pub mod search;
pub mod search_other;
pub mod search_smart;
pub mod state;
pub mod util;

pub mod prelude {
    pub use crate::{
        consts::{
            BOARD_SIZE,
            EMPTY_BOARD,
            input_hz::*
        },
        pieces::{Piece, Rotation},
        state::State,
    };
}
