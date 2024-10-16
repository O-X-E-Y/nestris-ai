// use std::collections::HashSet;
use ahash::AHashSet as HashSet;

// use arrayvec::ArrayVec;
use crate::const_arrayvec::ArrayVec;

use crate::{consts::*, pieces::*, state::*};

impl<'a> State<'a> {
    pub fn search_smart2_depth(&self, depth: u8) -> Vec<Board> {
        match depth {
            0 => vec![self.board],
            1 => {
                let mut state = self.const_clone();

                state
                    .search_smart2()
                    .into_iter()
                    .map(|pos| {
                        state.pos = pos;
                        state.lock();

                        let b = state.board;
                        state = self.const_clone();
                        b
                    })
                    .collect()
            }
            depth => {
                let mut encountered = HashSet::new();
                let mut final_states = Vec::new();

                let mut state = self.const_clone();

                for pos in state.search_smart2() {
                    state.pos = pos;
                    state.lock();
                    state.search_smart2_final_depth_helper(
                        depth - 1,
                        &mut encountered,
                        &mut final_states,
                    );

                    state = self.const_clone();
                }

                final_states
            }
        }
    }

    fn search_smart2_final_depth_helper(
        &self,
        depth: u8,
        encountered: &mut HashSet<Board>,
        final_states: &mut Vec<Board>,
    ) {
        match depth {
            0 => {}
            1 => {
                let mut state = self.const_clone();

                for start_pos in Piece::START_POSITIONS {
                    state.pos = start_pos;

                    for pos in state.search_visited_first() {
                        state.pos = pos;
                        state.lock();

                        if encountered.insert(state.board) {
                            final_states.push(state.board);
                        }

                        state = self.const_clone();
                    }
                }
            }
            _ => {
                let mut state = self.const_clone();

                for start_pos in Piece::START_POSITIONS {
                    state.pos = start_pos;

                    for pos in state.search_visited_first() {
                        state.pos = pos;
                        state.lock();

                        if encountered.insert(state.board) {
                            state.search_smart2_final_depth_helper(
                                depth - 1,
                                encountered,
                                final_states,
                            );
                        }

                        state = self.const_clone();
                    }
                }
            }
        }
    }

    pub const fn search_smart2(&mut self) -> ArrayVec<PiecePos, 128> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        self.search_smart2_helper(&mut visited, &mut final_states);

        final_states
    }

    const fn search_smart2_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 128>,
    ) {
        // flip before checking left/right seems to improve performance
        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart2_helper(visited, final_states);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart2_helper(visited, final_states);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart2_helper(visited, final_states);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }

        while !self.is_input() && !self.is_drop() {
            self.input_frame += 1;
            self.drop_frame += 1;
        }

        if self.is_input() {
            self.pos.x += 1;
            if !self.visited(visited) {
                self.pos.masks >>= 1;
                if !self.collision() {
                    self.visit(visited);

                    self.input_frame += 1;
                    self.search_smart2_helper(visited, final_states);
                    self.input_frame -= 1;
                }
                self.pos.masks <<= 1;
            }
            self.pos.x -= 1;

            self.pos.x -= 1;
            if !self.visited(visited) {
                self.pos.masks <<= 1;
                if !self.collision() {
                    self.visit(visited);

                    self.input_frame += 1;
                    self.search_smart2_helper(visited, final_states);
                    self.input_frame -= 1;
                }
                self.pos.masks >>= 1;
            }
            self.pos.x += 1;
        }

        if self.is_drop() {
            self.pos.down();

            if self.visited(visited) {
                self.pos.up();
                return;
            }

            self.visit(visited);

            if !self.collision() {
                self.search_smart2_helper(visited, final_states);
                self.pos.up();
            } else {
                self.pos.up();
                final_states.push(self.pos);
            }
        }
    }
}
