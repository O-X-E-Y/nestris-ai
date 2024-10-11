use crate::{
    consts::{BOARD_ROWS, LEFT_BIT}, pieces::*, state::*
};

impl<'a> State<'a> {
    pub fn eval_board(&self) -> u32 {
        self.eval_surface()
            + self.eval_holes()
    }

    fn eval_holes(&self) -> u32 {
        let mut res = 0;

        for i in 0..10 {
            let s = self.surface[i] as usize;
            for h in (BOARD_ROWS - s)..BOARD_ROWS {
                if (LEFT_BIT >> i + 3) & self.board[h] == 0 {
                    res += 1;
                }
            }
        }

        res * 2048
    }

    fn eval_surface(&self) -> u32 {
        let mut res = 0;

        const COL_WEIGHTS: [u32; 10] = [
            0, 10, 10, 20, 30, 40, 50, 60, 80, 100
        ];

        for i in 0..10 {
            res += (self.surface[i] as u32) * COL_WEIGHTS[i];
        }

        for i in 0..8 {
            let [v1, v2, v3] = [self.surface[0 + i], self.surface[1 + i], self.surface[2 + i]];
            let min = v1.min(v2).min(v3);
            let max = (v1.max(v2).max(v3).saturating_sub(3) as u32) * 10;
            let slice = [(v1 - min).min(3), (v2 - min).min(3), (v3 - min).min(3)];

            match slice {
                [0, 0, 0] => res += 100,
                [1, 0, 0] => res += 0,
                [0, 1, 0] => res += 400,
                [0, 0, 1] => res += 000,
                [1, 1, 0] => res += 200,
                [0, 1, 1] => res += 200,
                [1, 0, 1] => res += 400,

                [2, 0, 0] => res += 300,
                [0, 2, 0] => res += 600,
                [0, 0, 2] => res += 300,
                [2, 2, 0] => res += 400,
                [0, 2, 2] => res += 400,
                [2, 0, 2] => res += 500,

                [3, 0, 0] => res += 600 + max,
                [0, 3, 0] => res += 900 + max,
                [0, 0, 3] => res += 600 + max,
                [3, 3, 0] => res += 700 + max,
                [0, 3, 3] => res += 700 + max,
                [3, 0, 3] => res += 2500 + max,

                [1, 2, 0] => res += 400,
                [0, 2, 1] => res += 400,
                [1, 0, 2] => res += 500,
                [2, 0, 1] => res += 500,
                [2, 1, 0] => res += 300,
                [0, 1, 2] => res += 300,

                [1, 3, 0] => res += 600 + max,
                [0, 3, 1] => res += 600 + max,
                [1, 0, 3] => res += 800 + max,
                [3, 0, 1] => res += 800 + max,
                [3, 1, 0] => res += 400 + max,
                [0, 1, 3] => res += 400 + max,

                [2, 3, 0] => res += 700 + max,
                [0, 3, 2] => res += 700 + max,
                [2, 0, 3] => res += 1500 + max,
                [3, 0, 2] => res += 1500 + max,
                [3, 2, 0] => res += 500 + max,
                [0, 2, 3] => res += 500 + max,
                _ => {
                    panic!("slice {:?} not covered!", slice);
                    // res += 1
                }
            }
        }

        res
    }
}
