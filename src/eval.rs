use crate::{consts::*, state::*};

impl<'a> State<'a> {
    pub fn eval_board(&self) -> i32 {
        self.eval_surface()
            + self.eval_holes()
            + self.eval_score()
            + self.eval_left_well()
            + self.eval_highest_point()
    }

    pub fn eval_highest_point(&self) -> i32 {
        self.highest_block as i32 * self.weights.highest_point_multiplier
    }

    pub fn eval_left_well(&self) -> i32 {
        const RIGHT_COLUMN: u16 = 1 << 3;
        const RIGHT_WELL: u16 = FULL_ROW & (!RIGHT_COLUMN);

        let mut well_height = 0;

        let floor = BOARD_ROWS - 2;
        let ceil = floor - self.surface[9] as usize - 1;

        for y in ceil..=floor {
            let row = self.board[y];

            if RIGHT_COLUMN & row != 0 {
                break;
            }
            if RIGHT_WELL & row == RIGHT_WELL {
                well_height += 1;
            } else {
                well_height = 0;
            }
        }

        self.weights.well_height[well_height]

        // match well_height {
        //     0 => 0,
        //     1 => 1250,
        //     2 => 2500,
        //     3 => 3750,
        //     4 => 6000,
        //     _ => 6000 + (well_height - 4) * 1250
        // }
    }

    pub fn eval_score(&self) -> i32 {
        // match self.lines_delta {
        //     0 => 0,
        //     1 => -(1200 / 4) * 6,       // 40
        //     2 => -(1200 / 2) * 6,       // 100
        //     3 => -(1200 / 3 * 2) * 6,   // 400
        //     4 => 1200 * 6,              // 1200
        //     _ => unreachable!()
        // }
        match self.lines_delta {
            0 => 0,
            _ => self.weights.line_multipliers[self.lines_delta as usize - 1],
        }
    }

    pub fn eval_holes(&self) -> i32 {
        let mut res = 0;

        for i in 0..10 {
            let s = self.surface[i] as usize;
            for h in (BOARD_ROWS - s)..(BOARD_ROWS - 1) {
                if (LEFT_BIT >> i + 3) & self.board[h] == 0 {
                    res += h.abs_diff(BOARD_ROWS - 1 - s) as i32 * self.weights.hole_multiplier;
                    // res += match h.abs_diff(BOARD_ROWS - 1 - s) {
                    //     0 => 0,
                    //     1 => 1250,
                    //     2 => 3000,
                    //     n @ 3.. => (n as i32 - 1) * 2500,
                    // };
                }
            }
        }

        res
    }

    pub fn eval_surface(&self) -> i32 {
        let mut res = 0;

        // const COL_WEIGHTS: [i32; 10] = [
        //     0, 0, 5, 5, 5, 5, 10, 10, 10, 50
        // ];

        for i in 0..10 {
            res += (self.surface[i] as i32) * self.weights.col_weights[i];
        }

        let mut surface = [20; 12];
        for i in 0..10 {
            surface[i + 1] = self.surface[i];
        }

        for i in 0..10 {
            let [v1, v2, v3] = [surface[0 + i], surface[1 + i], surface[2 + i]];
            let min = v1.min(v2).min(v3);
            let ot =
                (v1.max(v2).max(v3).saturating_sub(3) as i32) * self.weights.over_three_multiplier;
            let slice = [(v1 - min).min(3), (v2 - min).min(3), (v3 - min).min(3)];

            match slice {
                [0, 0, 0] => res += self.weights.surface[0],
                [1, 0, 0] => res += self.weights.surface[1],
                [0, 1, 0] => res += self.weights.surface[2],
                [0, 0, 1] => res += self.weights.surface[3],
                [1, 1, 0] => res += self.weights.surface[4],
                [0, 1, 1] => res += self.weights.surface[5],
                [1, 0, 1] => res += self.weights.surface[6],

                [2, 0, 0] => res += self.weights.surface[7],
                [0, 2, 0] => res += self.weights.surface[8],
                [0, 0, 2] => res += self.weights.surface[9],
                [2, 2, 0] => res += self.weights.surface[10],
                [0, 2, 2] => res += self.weights.surface[11],
                [2, 0, 2] => res += self.weights.surface[12],

                [3, 0, 0] => res += self.weights.surface[13] + ot,
                [0, 3, 0] => res += self.weights.surface[14] + ot,
                [0, 0, 3] => res += self.weights.surface[15] + ot,
                [3, 3, 0] => res += self.weights.surface[16] + ot,
                [0, 3, 3] => res += self.weights.surface[17] + ot,
                [3, 0, 3] => res += self.weights.surface[18] + ot,

                [1, 2, 0] => res += self.weights.surface[19],
                [0, 2, 1] => res += self.weights.surface[20],
                [1, 0, 2] => res += self.weights.surface[21],
                [2, 0, 1] => res += self.weights.surface[22],
                [2, 1, 0] => res += self.weights.surface[23],
                [0, 1, 2] => res += self.weights.surface[24],

                [1, 3, 0] => res += self.weights.surface[25] + ot,
                [0, 3, 1] => res += self.weights.surface[26] + ot,
                [1, 0, 3] => res += self.weights.surface[27] + ot,
                [3, 0, 1] => res += self.weights.surface[28] + ot,
                [3, 1, 0] => res += self.weights.surface[29] + ot,
                [0, 1, 3] => res += self.weights.surface[30] + ot,

                [2, 3, 0] => res += self.weights.surface[31] + ot,
                [0, 3, 2] => res += self.weights.surface[32] + ot,
                [2, 0, 3] => res += self.weights.surface[33] + ot,
                [3, 0, 2] => res += self.weights.surface[34] + ot,
                [3, 2, 0] => res += self.weights.surface[35] + ot,
                [0, 2, 3] => res += self.weights.surface[36] + ot,
                _ => {
                    panic!("slice {:?} not covered!", slice);
                    // res += 1
                }
            }

            // match slice {
            //     [0, 0, 0] => res += 100,
            //     [1, 0, 0] => res += 0,
            //     [0, 1, 0] => res += 400,
            //     [0, 0, 1] => res += 000,
            //     [1, 1, 0] => res += 200,
            //     [0, 1, 1] => res += 200,
            //     [1, 0, 1] => res += 400,

            //     [2, 0, 0] => res += 300,
            //     [0, 2, 0] => res += 600,
            //     [0, 0, 2] => res += 300,
            //     [2, 2, 0] => res += 400,
            //     [0, 2, 2] => res += 400,
            //     [2, 0, 2] => res += 500,

            //     [3, 0, 0] => res += 600 + ot,
            //     [0, 3, 0] => res += 900 + ot,
            //     [0, 0, 3] => res += 600 + ot,
            //     [3, 3, 0] => res += 700 + ot,
            //     [0, 3, 3] => res += 700 + ot,
            //     [3, 0, 3] => res += 2500 + ot,

            //     [1, 2, 0] => res += 400,
            //     [0, 2, 1] => res += 400,
            //     [1, 0, 2] => res += 500,
            //     [2, 0, 1] => res += 500,
            //     [2, 1, 0] => res += 300,
            //     [0, 1, 2] => res += 300,

            //     [1, 3, 0] => res += 600 + ot,
            //     [0, 3, 1] => res += 600 + ot,
            //     [1, 0, 3] => res += 800 + ot,
            //     [3, 0, 1] => res += 800 + ot,
            //     [3, 1, 0] => res += 400 + ot,
            //     [0, 1, 3] => res += 400 + ot,

            //     [2, 3, 0] => res += 700 + ot,
            //     [0, 3, 2] => res += 700 + ot,
            //     [2, 0, 3] => res += 1500 + ot,
            //     [3, 0, 2] => res += 1500 + ot,
            //     [3, 2, 0] => res += 500 + ot,
            //     [0, 2, 3] => res += 500 + ot,
            //     _ => {
            //         panic!("slice {:?} not covered!", slice);
            //         // res += 1
            //     }
            // }
        }

        res
    }
}
