use crate::{consts::*, state::PiecePos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

use Rotation::*;

impl Rotation {
    pub const ROTATIONS: [Self; 4] = [North, East, South, West];

    #[inline(always)]
    pub const fn cw(&mut self) {
        *self = self.as_cw();
    }

    #[inline(always)]
    pub const fn ccw(&mut self) {
        *self = self.as_ccw();
    }

    #[inline(always)]
    pub const fn flip(&mut self) {
        *self = self.as_flip();
    }

    #[inline(always)]
    pub const fn as_cw(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    #[inline(always)]    
    pub const fn as_ccw(self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    #[inline(always)]    
    pub const fn as_flip(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

use Piece::*;

impl Piece {
    pub const PIECES: [Self; 7] = [I, J, L, O, S, T, Z];
    pub const START_POSITIONS: [PiecePos; 7] = [
        I.start_pos(),
        J.start_pos(),
        L.start_pos(),
        O.start_pos(),
        S.start_pos(),
        T.start_pos(),
        Z.start_pos(),
    ];

    #[inline(always)]
    pub const fn start_pos(self) -> PiecePos {
        let (x, masks) = match self {
            I => (6, I_NORTH_SOUTH),
            J => (7, J_NORTH),
            L => (7, L_NORTH),
            O => (7, O_ALL),
            S => (7, S_NORTH_SOUTH),
            T => (7, T_NORTH),
            Z => (7, Z_NORTH_SOUTH),
        };

        PiecePos {
            x,
            y: 1,
            piece: self,
            rot: North,
            masks: masks >> x,
        }
    }
}
