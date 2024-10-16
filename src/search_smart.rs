use ahash::AHashSet as HashSet;

use crate::{const_arrayvec::ArrayVec, consts::*, pieces::*, state::*};

impl<'a> State<'a> {
    pub fn search_smart_depth(&self, depth: u8) -> Vec<Board> {
        match depth {
            0 => vec![self.board],
            1 => {
                let mut state = self.const_clone();

                state
                    .search_smart()
                    .into_iter()
                    .map(|pos| {
                        state.pos = pos;
                        state.fast_lock();

                        let b = state.board;
                        state.board = self.board;
                        b
                    })
                    .collect()
            }
            depth => {
                let mut encountered = HashSet::new();
                let mut final_states = Vec::new();

                let mut state = self.const_clone();

                for pos in state.search_smart() {
                    state.pos = pos;
                    state.lock();
                    state.search_rec_separate_final_smart_depth_helper(
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

    fn search_rec_separate_final_smart_depth_helper(
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

                    for pos in state.search_smart() {
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

                    for pos in state.search_smart() {
                        state.pos = pos;
                        state.lock();

                        if encountered.insert(state.board) {
                            state.search_rec_separate_final_smart_depth_helper(
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

    // fn generate_dropping_positions_left(&mut self, ) {}

    pub fn search_smart(&mut self) -> ArrayVec<PiecePos, 128> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        // self.input_frame = self.input_sequence.len() - 1;
        self.search_smart_helper(&mut visited, &mut final_states, 0);

        final_states
    }

    fn search_smart_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 128>,
        frames_since_last_move: u8,
    ) {
        if frames_since_last_move >= self.input_sequence.longest_non_press {
            self.input_frame = 0;
        }

        // println!("{}, {}", self.input_frame % self.input_sequence.len(), self.is_drop());
        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart_helper(visited, final_states, frames_since_last_move);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart_helper(visited, final_states, frames_since_last_move);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_smart_helper(visited, final_states, frames_since_last_move);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }

        if self.is_input() {
            self.pos.x += 1;
            if !self.visited(visited) {
                self.pos.masks >>= 1;
                if !self.collision() {
                    self.visit(visited);

                    self.input_frame = 1;
                    self.search_smart_helper(visited, final_states, frames_since_last_move);
                    // self.input_frame += 1;
                }
                self.pos.masks <<= 1;
            }
            self.pos.x -= 1;

            self.pos.x -= 1;
            if !self.visited(visited) {
                self.pos.masks <<= 1;
                if !self.collision() {
                    self.visit(visited);

                    self.input_frame = 1;
                    self.search_smart_helper(visited, final_states, frames_since_last_move);
                    // self.input_frame += 1;
                }
                self.pos.masks >>= 1;
            }
            self.pos.x += 1;
        } else {
            self.input_frame += 1;
        }

        // self.input_frame += 1;

        if self.is_drop() {
            self.pos.down();

            if self.visited(visited) {
                self.pos.up();
                return;
            }

            self.visit(visited);

            self.drop_frame = 0;

            if !self.collision() {
                self.search_smart_helper(
                    visited,
                    final_states,
                    frames_since_last_move + self.drop_speed as u8,
                );
                self.pos.up();
            } else {
                self.pos.up();
                final_states.push(self.pos);
            }
        } else {
            self.drop_frame += 1;
            self.search_smart_helper(
                visited,
                final_states,
                frames_since_last_move + self.drop_speed as u8,
            );
            // dbg!(self.drop_frame);
            // self.drop_frame -= 1;
        }
    }
}

// impl State {
//     pub fn search_rec_visited_first_depth(&self, depth: u8) -> Vec<Board> {
//         match depth {
//             0 => vec![self.board],
//             1 => {
//                 let mut state = self.const_clone();

//                 state
//                     .search()
//                     .into_iter()
//                     .map(|pos| {
//                         state.pos = pos;
//                         state.fast_lock();

//                         let b = state.board;
//                         state = self.const_clone();
//                         b
//                     })
//                     .collect()
//             }
//             depth => {
//                 let mut encountered = HashSet::new();

//                 let mut state = self.const_clone();

//                 for pos in state.search() {
//                     state.pos = pos;
//                     state.fast_lock();
//                     state.search_rec_visited_first_depth_helper(depth - 1, &mut encountered);

//                     state = self.const_clone();
//                 }

//                 encountered
//                     .into_iter()
//                     .filter(|(f, _)| *f)
//                     .map(|(_, b)| b)
//                     .collect()
//             }
//         }
//     }

//     fn search_rec_visited_first_depth_helper(
//         &self,
//         depth: u8,
//         // sequence: &mut ArrayVec<PiecePos, 10>,
//         encountered: &mut HashSet<(bool, Board)>,
//     ) {
//         match depth {
//             0 => {},
//             1 => {
//                 let mut state = self.const_clone();

//                 for start_pos in Piece::START_POSITIONS {
//                     state.pos = start_pos;

//                     for pos in state.search() {
//                         state.pos = pos;
//                         state.fast_lock();

//                         encountered.insert((true, state.board));

//                         state = self.const_clone();
//                     }
//                 }
//             },
//             _ => {
//                 let mut state = self.const_clone();

//                 for start_pos in Piece::START_POSITIONS {
//                     state.pos = start_pos;

//                     for pos in state.search() {
//                         state.pos = pos;
//                         state.fast_lock();

//                         if encountered.insert((false, state.board)) {
//                             state.search_rec_visited_first_depth_helper(depth - 1, encountered);
//                         }

//                         state = self.const_clone();
//                     }
//                 }
//             },
//         }
//     }

//     pub const fn search_rec_naive(&mut self) -> ArrayVec<PiecePos, 128> {
//         let mut visited = [[0; BOARD_SIZE]; 4];
//         let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

//         self.search_rec_naive_helper(&mut visited, &mut final_states);

//         final_states
//     }

//     const fn search_rec_naive_helper(
//         &mut self,
//         visited: &mut [Board; 4],
//         final_states: &mut ArrayVec<PiecePos, 128>,
//     ) {
//         if self.try_right() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_naive_helper(visited, final_states);
//             }
//             self.pos.left();
//         }

//         if self.try_left() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_naive_helper(visited, final_states);
//             }
//             self.pos.right();
//         }

//         if self.try_cw() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_naive_helper(visited, final_states);
//             }
//             self.pos.ccw();
//         }

//         if self.try_ccw() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_naive_helper(visited, final_states);
//             }
//             self.pos.cw();
//         }

//         self.pos.down();

//         if self.visited(visited) {
//             self.pos.up();
//             return;
//         }

//         self.visit(visited);

//         if !self.collision() {
//             self.search_rec_naive_helper(visited, final_states);
//             self.pos.up();
//         } else {
//             self.pos.up();
//             final_states.push(self.pos);
//         }
//     }

//     pub const fn search_rec_select_rot(&mut self) -> ArrayVec<PiecePos, 128> {
//         let mut visited = [[0; BOARD_SIZE]; 4];
//         let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

//         self.search_rec_select_rot_helper(&mut visited, &mut final_states);

//         final_states
//     }

//     const fn search_rec_select_rot_helper(
//         &mut self,
//         visited: &mut [Board; 4],
//         final_states: &mut ArrayVec<PiecePos, 128>,
//     ) {
//         if self.try_right() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_select_rot_helper(visited, final_states);
//             }
//             self.pos.left();
//         }

//         if self.try_left() {
//             if !self.visited(visited) {
//                 self.visit(visited);
//                 self.search_rec_select_rot_helper(visited, final_states);
//             }
//             self.pos.right();
//         }

//         match self.pos.piece {
//             Piece::J | Piece::L | Piece::T => {
//                 if self.try_cw() {
//                     if !self.visited(visited) {
//                         self.visit(visited);
//                         self.search_rec_select_rot_helper(visited, final_states);
//                     }
//                     self.pos.ccw();
//                 }

//                 if self.try_ccw() {
//                     if !self.visited(visited) {
//                         self.visit(visited);
//                         self.search_rec_select_rot_helper(visited, final_states);
//                     }
//                     self.pos.cw();
//                 }
//             }
//             Piece::I | Piece::S | Piece::Z => {
//                 if self.try_cw() {
//                     if !self.visited(visited) {
//                         self.visit(visited);
//                         self.search_rec_select_rot_helper(visited, final_states);
//                     }
//                     self.pos.ccw();
//                 }
//             }
//             Piece::O => {}
//         }

//         self.pos.down();

//         if self.visited(visited) {
//             self.pos.up();
//             return;
//         }

//         self.visit(visited);

//         if !self.collision() {
//             self.search_rec_select_rot_helper(visited, final_states);
//             self.pos.up();
//         } else {
//             self.pos.up();
//             final_states.push(self.pos);
//         }
//     }

//     pub const fn search_rec_specialized(&mut self) -> ArrayVec<PiecePos, 128> {
//         let mut visited = [[0; BOARD_SIZE]; 4];
//         let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

//         match self.pos.piece {
//             Piece::I | Piece::S | Piece::Z => self.search_rec_isz(&mut visited, &mut final_states),
//             Piece::J | Piece::L | Piece::T => self.search_rec_jlt(&mut visited, &mut final_states),
//             Piece::O => self.search_rec_o(&mut visited, &mut final_states),
//         }

//         final_states
//     }

//     const fn search_rec_jlt(
//         &mut self,
//         visited: &mut [Board; 4],
//         final_states: &mut ArrayVec<PiecePos, 128>,
//     ) {
//         self.pos.x += 1;
//         if !self.visited(visited) {
//             self.pos.masks >>= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks <<= 1;
//         }
//         self.pos.x -= 1;

//         self.pos.x -= 1;
//         if !self.visited(visited) {
//             self.pos.masks <<= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks >>= 1;
//         }
//         self.pos.x += 1;

//         self.pos.cw();
//         if !self.visited(visited) && !self.collision() {
//             self.visit(visited);
//             self.search_rec_jlt(visited, final_states);
//         }
//         // equivalent to rotating ccw twice
//         self.pos.flip();

//         if !self.visited(visited) && !self.collision() {
//             self.visit(visited);
//             self.search_rec_jlt(visited, final_states);
//         }
//         self.pos.cw();

//         self.pos.down();

//         if self.visited(visited) {
//             self.pos.up();
//             return;
//         }

//         self.visit(visited);

//         if !self.collision() {
//             self.search_rec_jlt(visited, final_states);
//             self.pos.up();
//         } else {
//             self.pos.up();
//             final_states.push(self.pos);
//         }
//     }

//     const fn search_rec_isz(
//         &mut self,
//         visited: &mut [Board; 4],
//         final_states: &mut ArrayVec<PiecePos, 128>,
//     ) {
//         self.pos.x += 1;
//         if !self.visited(visited) {
//             self.pos.masks >>= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks <<= 1;
//         }
//         self.pos.x -= 1;

//         self.pos.x -= 1;
//         if !self.visited(visited) {
//             self.pos.masks <<= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks >>= 1;
//         }
//         self.pos.x += 1;

//         self.pos.cw();
//         if !self.visited(visited) && !self.collision() {
//             self.visit(visited);
//             self.search_rec_jlt(visited, final_states);
//         }
//         self.pos.ccw();

//         self.pos.down();

//         if self.visited(visited) {
//             self.pos.up();
//             return;
//         }

//         self.visit(visited);

//         if !self.collision() {
//             self.search_rec_isz(visited, final_states);
//             self.pos.up();
//         } else {
//             self.pos.up();
//             final_states.push(self.pos);
//         }
//     }

//     const fn search_rec_o(
//         &mut self,
//         visited: &mut [Board; 4],
//         final_states: &mut ArrayVec<PiecePos, 128>,
//     ) {
//         self.pos.x += 1;
//         if !self.visited(visited) {
//             self.pos.masks >>= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks <<= 1;
//         }
//         self.pos.x -= 1;

//         self.pos.x -= 1;
//         if !self.visited(visited) {
//             self.pos.masks <<= 1;
//             if !self.collision() {
//                 self.visit(visited);
//                 self.search_rec_jlt(visited, final_states);
//             }
//             self.pos.masks >>= 1;
//         }
//         self.pos.x += 1;

//         self.pos.down();

//         if self.visited(visited) {
//             self.pos.up();
//             return;
//         }

//         self.visit(visited);

//         if !self.collision() {
//             self.search_rec_o(visited, final_states);
//             self.pos.up();
//         } else {
//             self.pos.up();
//             final_states.push(self.pos);
//         }
//     }
// }
