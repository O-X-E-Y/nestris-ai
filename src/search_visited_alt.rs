use crate::const_arrayvec::ArrayVec;

use crate::{consts::*, pieces::*, state::*};

impl<'a> State<'a> {
    pub const fn search_drop_first_alt(&mut self) -> ArrayVec<PiecePos, 64> {
        let mut visited = [[[0; 13]; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        self.search_drop_first_alt_helper(&mut visited, &mut final_states);

        final_states
    }

    const fn search_drop_first_alt_helper(
        &mut self,
        visited: &mut VisitedAlt,
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        self.pos.down();

        if self.visited_alt(visited) {
            self.pos.up();
            self.search_drop_first_alt_movement_helper(visited, final_states);
            return;
        }

        self.visit_alt(visited);
        
        self.search_drop_first_alt_movement_helper(visited, final_states);

        if !self.collision() {
            self.search_drop_first_alt_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    pub const fn search_drop_first_alt_movement_helper(
        &mut self,
        visited: &mut VisitedAlt,
        final_states: &mut ArrayVec<PiecePos, 64>,
    ) {
        // flip before checking left/right seems to improve performance
        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.pos.cw();
                if !self.visited_alt(visited) && !self.collision() {
                    self.visit_alt(visited);
                    self.search_drop_first_alt_helper(visited, final_states);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited_alt(visited) && !self.collision() {
                    self.visit_alt(visited);
                    self.search_drop_first_alt_helper(visited, final_states);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited_alt(visited) && !self.collision() {
                    self.visit_alt(visited);
                    self.search_drop_first_alt_helper(visited, final_states);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }

        self.pos.x += 1;
        if !self.visited_alt(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit_alt(visited);
                self.search_drop_first_alt_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited_alt(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit_alt(visited);
                self.search_drop_first_alt_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;
    }

    pub const fn search_const_drop_first(&mut self) -> ArrayVec<PiecePos, 64> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);
        let mut stack = ArrayVec::<_, 128>::new_const(PiecePos::DEFAULT);

        let start = self.pos;
        stack.push(self.pos);

        while let Some(pos) = stack.pop() {
            self.pos = pos;

            self.pos.down();

            if self.visited(&visited) {
                self.pos.up();
                self.search_const_drop_first_helper(&mut visited, &mut stack);
                continue
            }

            self.visit(&mut visited);

            if !self.collision() {
                stack.push(self.pos);
            } else {
                self.pos.up();
                final_states.push(self.pos);
            }

            self.search_const_drop_first_helper(&mut visited, &mut stack);
        }

        self.pos = start;

        final_states
    }

    #[inline(always)]
    pub const fn search_const_drop_first_helper(&mut self, visited: &mut [Board; 4], stack: &mut ArrayVec<PiecePos, 128>) {
        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                stack.push(self.pos);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                stack.push(self.pos);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;

        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    stack.push(self.pos);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    stack.push(self.pos);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    stack.push(self.pos);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }
    }
}