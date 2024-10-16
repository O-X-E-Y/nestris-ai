// use std::collections::HashSet;
use ahash::AHashSet as HashSet;

// use arrayvec::ArrayVec;
use crate::const_arrayvec::ArrayVec;

use crate::{consts::*, pieces::*, state::*};

impl<'a> State<'a> {
    pub fn search_depth(&self, depth: u8) -> Vec<Board> {
        match depth {
            0 => vec![self.board],
            1 => {
                let mut state = self.const_clone();

                state
                    .search_visited_first()
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

                for pos in state.search_visited_first() {
                    state.pos = pos;
                    state.lock();
                    state.search_rec_separate_final_depth_helper(
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

    fn search_rec_separate_final_depth_helper(
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
                            state.search_rec_separate_final_depth_helper(
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

    #[inline(always)]
    pub const fn search_drop_first(&mut self) -> ArrayVec<PiecePos, 128> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        self.search_rec_drop_first_helper(&mut visited, &mut final_states);

        final_states
    }

    #[inline(always)]
    const fn search_rec_drop_first_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 128>,
    ) {
        self.pos.down();

        if self.visited(visited) {
            self.pos.up();
            self.search_drop_first_movement_helper(visited, final_states);
            return;
        }

        self.visit(visited);

        self.search_drop_first_movement_helper(visited, final_states);

        if !self.collision() {
            self.search_rec_drop_first_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    #[inline(always)]
    const fn search_drop_first_movement_helper(
        &mut self,
        visited: &mut [Board; 4],
        final_states: &mut ArrayVec<PiecePos, 128>,
    ) {
        match self.pos.piece {
            Piece::J | Piece::L | Piece::T => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_rec_drop_first_helper(visited, final_states);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_rec_drop_first_helper(visited, final_states);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_rec_drop_first_helper(visited, final_states);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }

        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_drop_first_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;
    }

    #[inline(always)]
    pub const fn search_visited_first(&mut self) -> ArrayVec<PiecePos, 128> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);

        self.search_rec_select_rot_visited_first_helper(&mut visited, &mut final_states);

        final_states
    }

    #[inline(always)]
    const fn search_rec_select_rot_visited_first_helper(
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
                    self.search_rec_select_rot_visited_first_helper(visited, final_states);
                }
                // equivalent to rotating ccw twice
                self.pos.flip();

                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_rec_select_rot_visited_first_helper(visited, final_states);
                }
                self.pos.cw();
            }
            Piece::I | Piece::S | Piece::Z => {
                self.pos.cw();
                if !self.visited(visited) && !self.collision() {
                    self.visit(visited);
                    self.search_rec_select_rot_visited_first_helper(visited, final_states);
                }
                self.pos.ccw();
            }
            Piece::O => {}
        }

        self.pos.x += 1;
        if !self.visited(visited) {
            self.pos.masks >>= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_select_rot_visited_first_helper(visited, final_states);
            }
            self.pos.masks <<= 1;
        }
        self.pos.x -= 1;

        self.pos.x -= 1;
        if !self.visited(visited) {
            self.pos.masks <<= 1;
            if !self.collision() {
                self.visit(visited);
                self.search_rec_select_rot_visited_first_helper(visited, final_states);
            }
            self.pos.masks >>= 1;
        }
        self.pos.x += 1;

        self.pos.down();

        if self.visited(visited) {
            self.pos.up();
            return;
        }

        self.visit(visited);

        if !self.collision() {
            self.search_rec_select_rot_visited_first_helper(visited, final_states);
            self.pos.up();
        } else {
            self.pos.up();
            final_states.push(self.pos);
        }
    }

    pub const fn search_const(&mut self) -> ArrayVec<PiecePos, 128> {
        let mut visited = [[0; BOARD_ROWS]; 4];
        let mut final_states = ArrayVec::new_const(PiecePos::DEFAULT);
        let mut stack = ArrayVec::<_, 128>::new_const(PiecePos::DEFAULT);

        let start = self.pos;
        stack.push(self.pos);

        while let Some(pos) = stack.pop() {
            self.pos = pos;

            self.pos.x += 1;
            if !self.visited(&visited) {
                self.pos.masks >>= 1;
                if !self.collision() {
                    self.visit(&mut visited);
                    stack.push(self.pos);
                }
                self.pos.masks <<= 1;
            }
            self.pos.x -= 1;

            self.pos.x -= 1;
            if !self.visited(&visited) {
                self.pos.masks <<= 1;
                if !self.collision() {
                    self.visit(&mut visited);
                    stack.push(self.pos);
                }
                self.pos.masks >>= 1;
            }
            self.pos.x += 1;

            match self.pos.piece {
                Piece::J | Piece::L | Piece::T => {
                    self.pos.cw();
                    if !self.visited(&visited) && !self.collision() {
                        self.visit(&mut visited);
                        stack.push(self.pos);
                    }
                    // equivalent to rotating ccw twice
                    self.pos.flip();

                    if !self.visited(&visited) && !self.collision() {
                        self.visit(&mut visited);
                        stack.push(self.pos);
                    }
                    self.pos.cw();
                }
                Piece::I | Piece::S | Piece::Z => {
                    self.pos.cw();
                    if !self.visited(&visited) && !self.collision() {
                        self.visit(&mut visited);
                        stack.push(self.pos);
                    }
                    self.pos.ccw();
                }
                Piece::O => {}
            }

            self.pos.down();

            if self.visited(&visited) {
                continue;
            }

            self.visit(&mut visited);

            if !self.collision() {
                stack.push(self.pos);
            } else {
                self.pos.up();
                final_states.push(self.pos);
            }
        }

        self.pos = start;

        final_states
    }
}

#[cfg(test)]
pub mod tests {
    use crate::weights::EvalWeights;

    use super::*;

    #[test]
    fn same_before_after() {
        let weights = EvalWeights::default();

        for p in Piece::PIECES {
            let mut state = State::new(p, 19, INPUT_30HZ, &weights);
            let reference = state.clone();

            state.search_rec_naive();
            assert_eq!(state, reference);

            state.search_rec_select_rot();
            assert_eq!(state, reference);

            state.search_visited_first();
            assert_eq!(state, reference);

            state.search_drop_first_alt();
            assert_eq!(state, reference);

            state.search_drop_first();
            assert_eq!(state, reference);

            state.search_drop_first_specialized();
            assert_eq!(state, reference);

            state.search_const();
            assert_eq!(state, reference);

            state.search_rec_specialized();
            assert_eq!(state, reference);
        }
    }

    #[test]
    fn correct_search_results() {
        let w = EvalWeights::default();

        for p in Piece::PIECES {
            let mut state = State::new(p, 19, INPUT_30HZ, &w);
            let naive = state.search_rec_naive().into_iter().collect::<HashSet<_>>();
            let select = state
                .search_rec_select_rot()
                .into_iter()
                .collect::<HashSet<_>>();
            let visited = state
                .search_visited_first()
                .into_iter()
                .map(Into::into)
                .collect::<HashSet<_>>();
            let drop_first = state
                .search_drop_first()
                .into_iter()
                .collect::<HashSet<_>>();
            let drop_first_specialized = state
                .search_drop_first_specialized()
                .into_iter()
                .collect::<HashSet<_>>();
            let iterative = state.search_const().into_iter().collect::<HashSet<_>>();
            let specialized = state
                .search_rec_specialized()
                .into_iter()
                .collect::<HashSet<_>>();

            assert_eq!(naive, select);
            assert_eq!(naive, visited);
            assert_eq!(naive, drop_first);
            assert_eq!(naive, drop_first_specialized);
            assert_eq!(naive, iterative);
            assert_eq!(naive, specialized);
        }
    }

    // #[test]
    fn _correct_depth_boards() {
        use Piece::*;

        fn board_ones(board: Board) -> u32 {
            board.iter().map(|r| r.count_ones()).sum()
        }

        let w = EvalWeights::default();

        let empty_board_ones = board_ones(State::new(Piece::I, 19, INPUT_30HZ, &w).board);

        // 23 * 2 + 16 = 62
        assert_eq!(empty_board_ones, 62);

        for p in Piece::PIECES {
            let _om = match p {
                L | J | T => 4usize,
                I | S | Z => 2,
                O => 1,
            };

            for depth in 0..4u32 {
                let state = State::new(p, 19, INPUT_30HZ, &w);

                state
                    .search_rec_visited_first_depth(depth as u8)
                    .into_iter()
                    .inspect(|b| {
                        let mut state = State::new(p, 19, INPUT_30HZ, &w);
                        state.board = *b;
                        println!("{state}");
                    })
                    .map(board_ones)
                    .for_each(|ones| assert_eq!(((empty_board_ones + 4 * depth) - ones) % 10, 0));
            }
        }
    }
}
