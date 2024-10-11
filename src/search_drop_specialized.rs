use crate::const_arrayvec::ArrayVec;

use crate::{consts::*, pieces::*, state::*};

impl<'a> State<'a> {
    #[inline(always)]
    pub const fn search_drop_first_specialized(&mut self) -> ArrayVec<PiecePos, 64> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.search_rec_drop_first_jlt_helper(&mut visited, &mut final_states);
            }
            Piece::I | Piece::S | Piece::Z => {
                self.search_rec_drop_first_isz_helper(&mut visited, &mut final_states);
            }
            Piece::O => {
                self.search_rec_drop_first_o_helper(&mut visited, &mut final_states);
            }
        }

        final_states
    }

    #[inline(always)]
    const fn search_rec_drop_first_jlt_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.down();

        if self.visited(visited) {
            self.pos.up();
            self.search_drop_first_jlt_movement_helper(visited, final_states);
            return;
        }

        self.visit(visited);
        
        self.search_drop_first_jlt_movement_helper(visited, final_states);

        if !self.collision() {
            self.search_rec_drop_first_jlt_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    #[inline(always)]
    const fn search_drop_first_jlt_movement_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.cw();
        if !self.visited(visited) && !self.collision() {
            self.visit(visited);
            self.search_rec_drop_first_jlt_helper(visited, final_states);
        }
        // equivalent to rotating ccw twice
        self.pos.flip();

        if !self.visited(visited) && !self.collision() {
            self.visit(visited);
            self.search_rec_drop_first_jlt_helper(visited, final_states);
        }
        self.pos.cw();

        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_jlt_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_jlt_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;
    }

    #[inline(always)]
    const fn search_rec_drop_first_isz_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.down();

        if self.visited(visited) {
            self.pos.up();
            self.search_drop_first_isz_movement_helper(visited, final_states);
            return;
        }

        self.visit(visited);
        
        self.search_drop_first_isz_movement_helper(visited, final_states);

        if !self.collision() {
            self.search_rec_drop_first_isz_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    #[inline(always)]
    const fn search_drop_first_isz_movement_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.cw();
        if !self.visited(visited) && !self.collision() {
            self.visit(visited);
            self.search_rec_drop_first_isz_helper(visited, final_states);
        }
        self.pos.ccw();

        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_isz_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_isz_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;
    }

    #[inline(always)]
    const fn search_rec_drop_first_o_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.down();

        if self.visited(visited) {
            self.pos.up();
            self.search_drop_first_o_movement_helper(visited, final_states);
            return;
        }

        self.visit(visited);
        
        self.search_drop_first_o_movement_helper(visited, final_states);

        if !self.collision() {
            self.search_rec_drop_first_o_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    #[inline(always)]
    const fn search_drop_first_o_movement_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_o_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_o_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;
    }
}