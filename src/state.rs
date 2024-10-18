use crate::{consts::*, pieces::*, util::*, weights::EvalWeights};

pub type Board = [u16; BOARD_ROWS];
pub type VisitedAlt = [[[u8; 13]; BOARD_ROWS]; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PiecePos {
    pub x: u8,
    pub y: u8,
    pub piece: Piece,
    pub rot: Rotation,
    pub masks: u64,
}

impl PiecePos {
    pub const DEFAULT: Self = Self {
        x: 0,
        y: 0,
        piece: Piece::I,
        rot: Rotation::North,
        masks: 0,
    };

    #[inline(always)]
    pub const fn left(&mut self) {
        self.x -= 1;
        self.masks <<= 1;
    }

    #[inline(always)]
    pub const fn right(&mut self) {
        self.x += 1;
        self.masks >>= 1;
    }

    #[inline(always)]
    pub const fn up(&mut self) {
        self.y -= 1;
    }

    #[inline(always)]
    pub const fn down(&mut self) {
        self.y += 1;
    }

    #[inline(always)]
    pub const fn cw(&mut self) {
        use Piece::*;
        use Rotation::*;

        let (rot, masks) = match (self.piece, self.rot) {
            (I, North | South) => (East, I_EAST_WEST),
            (I, East | West) => (North, I_NORTH_SOUTH),

            (J, North) => (East, J_EAST),
            (J, East) => (South, J_SOUTH),
            (J, South) => (West, J_WEST),
            (J, West) => (North, J_NORTH),

            (L, North) => (East, L_EAST),
            (L, East) => (South, L_SOUTH),
            (L, South) => (West, L_WEST),
            (L, West) => (North, L_NORTH),

            (O, _) => return,

            (S, North | South) => (East, S_EAST_WEST),
            (S, East | West) => (North, S_NORTH_SOUTH),

            (T, North) => (East, T_EAST),
            (T, East) => (South, T_SOUTH),
            (T, South) => (West, T_WEST),
            (T, West) => (North, T_NORTH),

            (Z, North | South) => (East, Z_EAST_WEST),
            (Z, East | West) => (North, Z_NORTH_SOUTH),
        };

        self.rot = rot;
        self.masks = masks >> self.x;
    }

    #[inline(always)]
    pub const fn ccw(&mut self) {
        use Piece::*;
        use Rotation::*;

        let (rot, masks) = match (self.piece, self.rot) {
            (I, North | South) => (East, I_EAST_WEST),
            (I, East | West) => (North, I_NORTH_SOUTH),

            (J, North) => (West, J_WEST),
            (J, East) => (North, J_NORTH),
            (J, South) => (East, J_EAST),
            (J, West) => (South, J_SOUTH),

            (L, North) => (West, L_WEST),
            (L, East) => (North, L_NORTH),
            (L, South) => (East, L_EAST),
            (L, West) => (South, L_SOUTH),

            (O, _) => return,

            (S, North | South) => (East, S_EAST_WEST),
            (S, East | West) => (North, S_NORTH_SOUTH),

            (T, North) => (West, T_WEST),
            (T, East) => (North, T_NORTH),
            (T, South) => (East, T_EAST),
            (T, West) => (South, T_SOUTH),

            (Z, North | South) => (East, Z_EAST_WEST),
            (Z, East | West) => (North, Z_NORTH_SOUTH),
        };

        self.rot = rot;
        self.masks = masks >> self.x;
    }

    #[inline(always)]
    pub const fn flip(&mut self) {
        use Piece::*;
        use Rotation::*;

        let (rot, masks) = match (self.piece, self.rot) {
            (I | O | S | Z, _) => return,

            (J, North) => (South, J_SOUTH),
            (J, East) => (West, J_WEST),
            (J, South) => (North, J_NORTH),
            (J, West) => (East, J_EAST),

            (L, North) => (South, L_SOUTH),
            (L, East) => (West, L_WEST),
            (L, South) => (North, L_NORTH),
            (L, West) => (East, L_EAST),

            (T, North) => (South, T_SOUTH),
            (T, East) => (West, T_WEST),
            (T, South) => (North, T_NORTH),
            (T, West) => (East, T_EAST),
        };

        self.rot = rot;
        self.masks = masks >> self.x;
    }

    #[inline(always)]
    pub(crate) const fn _sync(&mut self) {
        use Piece::*;
        use Rotation::*;

        let masks = match (self.piece, self.rot) {
            (I | O | S | Z, _) => return,

            (J, North) => J_NORTH,
            (J, East) => J_EAST,
            (J, South) => J_SOUTH,
            (J, West) => J_WEST,

            (L, North) => L_NORTH,
            (L, East) => L_EAST,
            (L, South) => L_SOUTH,
            (L, West) => L_WEST,

            (T, North) => T_NORTH,
            (T, East) => T_EAST,
            (T, South) => T_SOUTH,
            (T, West) => T_WEST,
        };

        self.masks = masks >> self.x;
    }

    #[inline(always)]
    pub const fn masks(&self) -> [u16; 4] {
        unsafe { std::mem::transmute(self.masks) }
    }

    #[inline(always)]
    pub const fn placement(&self) -> PiecePlacement {
        PiecePlacement {
            x: self.x,
            y: self.y,
            piece: self.piece,
            rot: self.rot,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PiecePlacement {
    pub x: u8,
    pub y: u8,
    pub piece: Piece,
    pub rot: Rotation,
}

impl PiecePlacement {
    pub const DEFAULT: Self = Self {
        x: 0,
        y: 0,
        piece: Piece::I,
        rot: Rotation::North,
    };

    pub const fn compact(self) -> CompactPlacement {
        let mut v = 0u16;

        v |= (self.x as u16) << 12;
        v |= (self.y as u16) << 7;
        v |= (self.piece as u16) << 4;
        v |= (self.rot as u16) << 2;

        CompactPlacement(v)
    }
}

impl From<PiecePlacement> for PiecePos {
    fn from(PiecePlacement { x, y, piece, rot }: PiecePlacement) -> Self {
        use Piece::*;
        use Rotation::*;

        let mut masks = match (piece, rot) {
            (I, North | South) => I_NORTH_SOUTH,
            (I, East | West) => I_EAST_WEST,

            (J, North) => J_NORTH,
            (J, East) => J_EAST,
            (J, South) => J_SOUTH,
            (J, West) => J_WEST,

            (L, North) => L_NORTH,
            (L, East) => L_EAST,
            (L, South) => L_SOUTH,
            (L, West) => L_WEST,

            (O, _) => O_ALL,

            (S, North | South) => S_NORTH_SOUTH,
            (S, East | West) => S_EAST_WEST,

            (T, North) => T_NORTH,
            (T, East) => T_EAST,
            (T, South) => T_SOUTH,
            (T, West) => T_WEST,

            (Z, North | South) => Z_NORTH_SOUTH,
            (Z, East | West) => Z_EAST_WEST,
        };

        masks >>= x;

        Self {
            x,
            y,
            piece,
            rot,
            masks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactPlacement(u16);

impl CompactPlacement {
    pub const fn decompact(self) -> Option<PiecePlacement> {
        let x = (self.0 >> 12) as u8;
        let y = (self.0 >> 7 & 0b11111) as u8;

        let piece = match self.0 >> 4 & 0b111 {
            0 => Piece::I,
            1 => Piece::J,
            2 => Piece::L,
            3 => Piece::O,
            4 => Piece::S,
            5 => Piece::T,
            6 => Piece::Z,
            _ => return None,
        };
        let rot = match self.0 >> 2 & 0b11 {
            0 => Rotation::North,
            1 => Rotation::East,
            2 => Rotation::South,
            3 => Rotation::West,
            _ => return None,
        };

        Some(PiecePlacement { x, y, piece, rot })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputSequence {
    seq: [bool; 32],
    len: usize,
    pub longest_non_press: u8,
}

impl InputSequence {
    pub const fn new() -> Self {
        Self {
            seq: [false; 32],
            len: 1,
            longest_non_press: 32,
        }
    }

    pub const fn with_tapping_speed(tapping_speed: usize) -> Self {
        let mut seq = [false; 32];
        if tapping_speed == 0 {
            return Self::new();
        }

        seq[0] = true;

        let tapping_speed = if tapping_speed > 30 {
            30
        } else {
            tapping_speed
        };

        let tap_every = 60f64 / tapping_speed as f64;
        let mut i = 1;
        let mut frames_passed = 1f64;
        let mut len = 1;
        let mut longest_non_press = 0;
        let mut last_press = 0;

        while i < seq.len() {
            if frames_passed >= tap_every {
                seq[i] = true;
                frames_passed -= tap_every;

                if i as u8 - last_press > longest_non_press {
                    longest_non_press = i as u8 - last_press;
                }
                last_press = i as u8;
                len = i;
            }
            frames_passed += 1.0;
            i += 1;
        }

        if len == 1 {
            len = 32;
            longest_non_press = 32;
        }

        Self {
            seq,
            len,
            longest_non_press,
        }
    }

    #[inline(always)]
    pub const fn is_input(&self, frame: usize) -> bool {
        self.seq[frame % self.len]
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Display for InputSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.seq.into_iter().take(self.len) {
            match b {
                true => write!(f, "x ")?,
                false => write!(f, ". ")?,
            }
        }

        Ok(())
    }
}

#[test]
fn seq() {
    let hz10 = INPUT_10HZ;
    let hz12 = INPUT_12HZ;
    let hz15 = INPUT_15HZ;
    let hz18 = INPUT_18HZ;
    let hz20 = INPUT_20HZ;
    let hz25 = INPUT_25HZ;
    let hz30 = INPUT_30HZ;

    println!("10 hz: {hz10}");
    println!("12 hz: {hz12}");
    println!("15 hz: {hz15}");
    println!("18 hz: {hz18}");
    println!("20 hz: {hz20}");
    println!("25 hz: {hz25}");
    println!("30 hz: {hz30}");
}

impl Default for InputSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct State<'a> {
    pub board: Board,
    pub surface: [u8; BOARD_COLS],
    pub highest_block: u8,
    pub pos: PiecePos,
    pub next: Option<Piece>,
    pub level: u8,
    pub lines: u16,
    pub lines_delta: u8,
    pub score: u32,
    pub input_sequence: &'a InputSequence,
    pub input_frame: usize,
    pub drop_speed: u16,
    pub drop_frame: u16,
    pub weights: &'a EvalWeights,
}

impl<'a> State<'a> {
    #[inline(always)]
    pub const fn new(
        piece: Piece,
        level: u8,
        input_sequence: &'a InputSequence,
        weights: &'a EvalWeights,
    ) -> Self {
        Self {
            board: EMPTY_BOARD,
            surface: [0; BOARD_COLS],
            highest_block: 0,
            pos: piece.start_pos(),
            next: None,
            level,
            lines: 0,
            lines_delta: 0,
            score: 0,
            input_sequence,
            input_frame: 0,
            drop_speed: drop_speed(level),
            drop_frame: 0,
            weights,
        }
    }

    #[inline(always)]
    pub const fn const_clone(&self) -> Self {
        Self {
            board: self.board,
            surface: self.surface,
            highest_block: self.highest_block,
            pos: self.pos,
            next: self.next,
            level: self.level,
            lines: self.lines,
            lines_delta: self.lines_delta,
            score: self.score,
            input_sequence: self.input_sequence,
            input_frame: self.input_frame,
            drop_speed: self.drop_speed,
            drop_frame: self.drop_frame,
            weights: self.weights,
        }
    }

    #[inline(always)]
    pub const fn collision(&self) -> bool {
        let [mask1, mask2, mask3, mask4] = self.pos.masks();

        mask1 & self.board[self.pos.y as usize] != 0
            || mask2 & self.board[self.pos.y as usize + 1] != 0
            || mask3 & self.board[self.pos.y as usize + 2] != 0
            || mask4 & self.board[self.pos.y as usize + 3] != 0
    }

    #[inline(always)]
    pub const fn try_left(&mut self) -> bool {
        self.pos.left();

        if self.collision() {
            self.pos.right();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub const fn try_right(&mut self) -> bool {
        self.pos.right();

        if self.collision() {
            self.pos.left();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub const fn try_up(&mut self) -> bool {
        self.pos.up();

        if self.collision() {
            self.pos.down();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub const fn try_down(&mut self) -> bool {
        self.pos.down();

        if self.collision() {
            self.pos.up();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub const fn try_cw(&mut self) -> bool {
        self.pos.cw();

        if self.collision() {
            self.pos.ccw();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub const fn try_ccw(&mut self) -> bool {
        self.pos.ccw();

        if self.collision() {
            self.pos.cw();
            false
        } else {
            true
        }
    }

    #[inline(always)]
    pub fn fast_lock(&mut self) {
        use Piece::*;
        use Rotation::*;

        let (y_vals, x_offset, y_offset) = match (self.pos.piece, self.pos.rot) {
            (I, North | South) => (
                I_NORTH_SOUTH_Y_VALS,
                I_NORTH_SOUTH_X_OFFSET,
                I_NORTH_SOUTH_Y_OFFSET,
            ),
            (I, East | West) => (
                I_EAST_WEST_Y_VALS,
                I_EAST_WEST_X_OFFSET,
                I_EAST_WEST_Y_OFFSET,
            ),

            (J, North) => (J_NORTH_Y_VALS, J_NORTH_X_OFFSET, J_NORTH_Y_OFFSET),
            (J, East) => (J_EAST_Y_VALS, J_EAST_X_OFFSET, J_EAST_Y_OFFSET),
            (J, South) => (J_SOUTH_Y_VALS, J_SOUTH_X_OFFSET, J_SOUTH_Y_OFFSET),
            (J, West) => (J_WEST_Y_VALS, J_WEST_X_OFFSET, J_WEST_Y_OFFSET),

            (L, North) => (L_NORTH_Y_VALS, L_NORTH_X_OFFSET, L_NORTH_Y_OFFSET),
            (L, East) => (L_EAST_Y_VALS, L_EAST_X_OFFSET, L_EAST_Y_OFFSET),
            (L, South) => (L_SOUTH_Y_VALS, L_SOUTH_X_OFFSET, L_SOUTH_Y_OFFSET),
            (L, West) => (L_WEST_Y_VALS, L_WEST_X_OFFSET, L_WEST_Y_OFFSET),

            (O, _) => (O_ALL_Y_VALS, O_ALL_X_OFFSET, O_ALL_Y_OFFSET),

            (S, North | South) => (
                S_NORTH_SOUTH_Y_VALS,
                S_NORTH_SOUTH_X_OFFSET,
                S_NORTH_SOUTH_Y_OFFSET,
            ),
            (S, East | West) => (
                S_EAST_WEST_Y_VALS,
                S_EAST_WEST_X_OFFSET,
                S_EAST_WEST_Y_OFFSET,
            ),

            (T, North) => (T_NORTH_Y_VALS, T_NORTH_X_OFFSET, T_NORTH_Y_OFFSET),
            (T, East) => (T_EAST_Y_VALS, T_EAST_X_OFFSET, T_EAST_Y_OFFSET),
            (T, South) => (T_SOUTH_Y_VALS, T_SOUTH_X_OFFSET, T_SOUTH_Y_OFFSET),
            (T, West) => (T_WEST_Y_VALS, T_WEST_X_OFFSET, T_WEST_Y_OFFSET),

            (Z, North | South) => (
                Z_NORTH_SOUTH_Y_VALS,
                Z_NORTH_SOUTH_X_OFFSET,
                Z_NORTH_SOUTH_Y_OFFSET,
            ),
            (Z, East | West) => (
                Z_EAST_WEST_Y_VALS,
                Z_EAST_WEST_X_OFFSET,
                Z_EAST_WEST_Y_OFFSET,
            ),
        };

        let x = self.pos.x - x_offset;
        let y = y_offset - self.pos.y;

        for i in x..(x + 4).min(10) {
            let new_y = self.surface[i as usize].max(y + y_vals[(i - x) as usize]);
            self.highest_block = self.highest_block.max(new_y);
            self.surface[i as usize] = new_y;
        }

        let mut cleared = 0;
        let mut y = BOARD_ROWS - 2;

        while y >= (BOARD_ROWS - 2).saturating_sub(self.highest_block as usize).saturating_sub(cleared).max(1) {
            if self.board[y] & FULL_ROW == FULL_ROW {
                cleared += 1;
            }

            y -= 1;
        }

        if cleared != 0 {
            self.highest_block -= cleared as u8;
            self.surface.iter_mut().for_each(|s| *s -= cleared as u8);

            let delta = match cleared {
                1 => 40 * (cleared + 1) as u32,
                2 => 100 * (cleared + 1) as u32,
                3 => 300 * (cleared + 1) as u32,
                4 => 1200 * (cleared + 1) as u32,
                _ => panic!("Clearing more than 4 lines at once should still be impossible"),
            };

            self.score += delta;
            self.lines_delta = cleared as u8;
        }
    }

    /// NOTE: assumes the board is in a legal state.
    #[inline(always)]
    pub fn lock(&mut self) {
        let [mask1, mask2, mask3, mask4] = self.pos.masks();

        self.board[self.pos.y as usize] |= mask1;
        self.board[self.pos.y as usize + 1] |= mask2;
        self.board[self.pos.y as usize + 2] |= mask3;
        self.board[self.pos.y as usize + 3] |= mask4;

        use Piece::*;
        use Rotation::*;

        let (y_vals, x_offset, y_offset) = match (self.pos.piece, self.pos.rot) {
            (I, North | South) => (
                I_NORTH_SOUTH_Y_VALS,
                I_NORTH_SOUTH_X_OFFSET,
                I_NORTH_SOUTH_Y_OFFSET,
            ),
            (I, East | West) => (
                I_EAST_WEST_Y_VALS,
                I_EAST_WEST_X_OFFSET,
                I_EAST_WEST_Y_OFFSET,
            ),

            (J, North) => (J_NORTH_Y_VALS, J_NORTH_X_OFFSET, J_NORTH_Y_OFFSET),
            (J, East) => (J_EAST_Y_VALS, J_EAST_X_OFFSET, J_EAST_Y_OFFSET),
            (J, South) => (J_SOUTH_Y_VALS, J_SOUTH_X_OFFSET, J_SOUTH_Y_OFFSET),
            (J, West) => (J_WEST_Y_VALS, J_WEST_X_OFFSET, J_WEST_Y_OFFSET),

            (L, North) => (L_NORTH_Y_VALS, L_NORTH_X_OFFSET, L_NORTH_Y_OFFSET),
            (L, East) => (L_EAST_Y_VALS, L_EAST_X_OFFSET, L_EAST_Y_OFFSET),
            (L, South) => (L_SOUTH_Y_VALS, L_SOUTH_X_OFFSET, L_SOUTH_Y_OFFSET),
            (L, West) => (L_WEST_Y_VALS, L_WEST_X_OFFSET, L_WEST_Y_OFFSET),

            (O, _) => (O_ALL_Y_VALS, O_ALL_X_OFFSET, O_ALL_Y_OFFSET),

            (S, North | South) => (
                S_NORTH_SOUTH_Y_VALS,
                S_NORTH_SOUTH_X_OFFSET,
                S_NORTH_SOUTH_Y_OFFSET,
            ),
            (S, East | West) => (
                S_EAST_WEST_Y_VALS,
                S_EAST_WEST_X_OFFSET,
                S_EAST_WEST_Y_OFFSET,
            ),

            (T, North) => (T_NORTH_Y_VALS, T_NORTH_X_OFFSET, T_NORTH_Y_OFFSET),
            (T, East) => (T_EAST_Y_VALS, T_EAST_X_OFFSET, T_EAST_Y_OFFSET),
            (T, South) => (T_SOUTH_Y_VALS, T_SOUTH_X_OFFSET, T_SOUTH_Y_OFFSET),
            (T, West) => (T_WEST_Y_VALS, T_WEST_X_OFFSET, T_WEST_Y_OFFSET),

            (Z, North | South) => (
                Z_NORTH_SOUTH_Y_VALS,
                Z_NORTH_SOUTH_X_OFFSET,
                Z_NORTH_SOUTH_Y_OFFSET,
            ),
            (Z, East | West) => (
                Z_EAST_WEST_Y_VALS,
                Z_EAST_WEST_X_OFFSET,
                Z_EAST_WEST_Y_OFFSET,
            ),
        };

        let x = self.pos.x - x_offset;
        let y = y_offset - self.pos.y;

        for i in x..(x + 4).min(10) {
            let new_y = self.surface[i as usize].max(y + y_vals[(i - x) as usize]);
            self.highest_block = self.highest_block.max(new_y);
            self.surface[i as usize] = new_y;
        }

        let mut cleared = 0;
        let mut y = BOARD_ROWS - 2;

        while y >= (BOARD_ROWS - 2).saturating_sub(self.highest_block as usize).saturating_sub(cleared).max(1) {
            if self.board[y] & FULL_ROW == FULL_ROW {
                cleared += 1;
            }

            match cleared {
                0 => {}
                1 => self.board[y] = self.board[y - 1],
                c @ 2..=4 => self.board[y + (c - 1)] = self.board[y.saturating_sub(1)],
                _ => panic!("Clearing more than 4 lines at once should be impossible"),
            }

            y -= 1;
        }

        if cleared != 0 {
            self.highest_block -= cleared as u8;
            self.surface.iter_mut().for_each(|s| *s -= cleared as u8);

            self.lines += cleared as u16;
            let delta = match cleared {
                1 => 40 * (cleared + 1) as u32,
                2 => 100 * (cleared + 1) as u32,
                3 => 300 * (cleared + 1) as u32,
                4 => 1200 * (cleared + 1) as u32,
                _ => panic!("Clearing more than 4 lines at once should still be impossible"),
            };

            self.score += delta;
            self.lines_delta = cleared as u8;
        }
    }

    #[inline(always)]
    pub const fn is_topped_out(&self) -> bool {
        self.board[3] & 0b0000_0111_1110_0000 != 0
    }

    pub const fn drop(&mut self) {
        while self.try_down() {}
        // let mut i = 0;
        // while i < BOARD_ROWS {
        //     self.try_down();
        //     i += 1;
        // }
    }

    #[inline(always)]
    pub const fn placement(&self) -> PiecePlacement {
        self.pos.placement()
    }

    #[inline(always)]
    pub const fn is_input(&self) -> bool {
        self.input_sequence.is_input(self.input_frame)
    }

    #[inline(always)]
    pub const fn is_drop(&self) -> bool {
        debug_assert!(self.drop_frame <= self.drop_speed);
        self.drop_frame == self.drop_speed
    }

    #[inline(always)]
    pub(crate) const fn visited(&self, visited: &[Board; 4]) -> bool {
        visited[self.pos.rot as usize][self.pos.y as usize] & (LEFT_BIT >> self.pos.x) != 0
    }

    #[inline(always)]
    pub(crate) const fn visit(&self, visited: &mut [Board; 4]) {
        visited[self.pos.rot as usize][self.pos.y as usize] |= LEFT_BIT >> self.pos.x;
    }

    #[inline(always)]
    pub(crate) const fn visited_alt(&self, visited: &VisitedAlt) -> bool {
        visited[self.pos.rot as usize][self.pos.y as usize][self.pos.x as usize] != 0
    }

    #[inline(always)]
    pub(crate) const fn visit_alt(&self, visited: &mut VisitedAlt) {
        visited[self.pos.rot as usize][self.pos.y as usize][self.pos.x as usize] = 1;
    }
}

impl<'a> std::fmt::Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y = self.pos.y as usize;
        let masks = self.pos.masks();

        (3..(BOARD_ROWS - 1)).for_each(|i| {
            let mut row = self.board[i].reverse_bits();
            if (y..(y + 4)).contains(&i) {
                row |= masks[i - y].reverse_bits();
            }
            row >>= 3;

            for _ in 0..10 {
                if row & 1 == 0 {
                    let _ = write!(f, ". ");
                } else {
                    let _ = write!(f, "O ");
                }
                row >>= 1;
            }
            let _ = writeln!(f);
        });

        Ok(())
    }
}
