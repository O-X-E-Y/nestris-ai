use crate::state::Board;

pub const BOARD_SIZE: usize = 24;
pub const EMPTY_ROW: u16 = 0b0010000000000100;
pub const FULL_ROW: u16 = 0b0011111111111100;
pub const BOT_ROW: u16 = u16::MAX;
pub const LEFT_BIT: u16 = 0b1000_0000_0000_0000;
pub const EMPTY_BOARD: Board = {
    let mut board = [EMPTY_ROW; BOARD_SIZE];
    board[BOARD_SIZE - 1] = BOT_ROW;
    board
};
pub const TRY_TUCK: u16 = 0b1000_0000_0000_0000;
pub const FLAG2: u16 = 0b0100_0000_0000_0000;
pub const FLAG3: u16 = 0b0000_0000_0000_0010;
pub const FLAG4: u16 = 0b0000_0000_0000_0001;

pub mod input_hz {
    use crate::state::InputSequence;
    
    pub const INPUT_10HZ: &InputSequence = &InputSequence::with_tapping_speed(10);
    pub const INPUT_12HZ: &InputSequence = &InputSequence::with_tapping_speed(12);
    pub const INPUT_15HZ: &InputSequence = &InputSequence::with_tapping_speed(15);
    pub const INPUT_18HZ: &InputSequence = &InputSequence::with_tapping_speed(18);
    pub const INPUT_20HZ: &InputSequence = &InputSequence::with_tapping_speed(20);
    pub const INPUT_25HZ: &InputSequence = &InputSequence::with_tapping_speed(25);
    pub const INPUT_30HZ: &InputSequence = &InputSequence::with_tapping_speed(30);
}

pub use input_hz::*;

pub const I_NORTH_SOUTH: u64 = 263882790666240;
pub const I_EAST_WEST: u64 = 2305878194122661888;
pub const L_NORTH: u64 = 9223618327459397632;
pub const L_EAST: u64 = 4611756390392791040;
pub const L_SOUTH: u64 = 246291141492736;
pub const L_WEST: u64 = 6917599397459001344;
pub const J_NORTH: u64 = 2306089299818315776;
pub const J_EAST: u64 = 13835128425100083200;
pub const J_SOUTH: u64 = 246292752105472;
pub const J_WEST: u64 = 4611756388782178304;
pub const O_ALL: u64 = 13835269161514696704;
pub const T_NORTH: u64 = 4611932309032009728;
pub const T_EAST: u64 = 4611897125733662720;
pub const T_SOUTH: u64 = 246291678363648;
pub const T_WEST: u64 = 4611791572617396224;
pub const S_NORTH_SOUTH: u64 = 13835163608398430208;
pub const S_EAST_WEST: u64 = 2305948563403702272;
pub const Z_NORTH_SOUTH: u64 = 6917740133873614848;
pub const Z_EAST_WEST: u64 = 4611791572080525312;

pub const I_NORTH_SOUTH_Y_VALS: [u8; 4] = [2, 2, 2, 2];
pub const I_EAST_WEST_Y_VALS: [u8; 4] = [0, 0, 4, 0];
pub const L_NORTH_Y_VALS: [u8; 4] = [2, 2, 2, 0];
pub const L_EAST_Y_VALS: [u8; 4] = [3, 3, 0, 0];
pub const L_SOUTH_Y_VALS: [u8; 4] = [2, 2, 3, 1];
pub const L_WEST_Y_VALS: [u8; 4] = [0, 3, 1, 0];
pub const J_NORTH_Y_VALS: [u8; 4] = [2, 2, 2, 0];
pub const J_EAST_Y_VALS: [u8; 4] = [1, 3, 0, 0];
pub const J_SOUTH_Y_VALS: [u8; 4] = [3, 2, 2, 1];
pub const J_WEST_Y_VALS: [u8; 4] = [0, 3, 3, 0];
pub const O_ALL_Y_VALS: [u8; 4] = [2, 2, 0, 0];
pub const T_NORTH_Y_VALS: [u8; 4] = [2, 2, 2, 0];
pub const T_EAST_Y_VALS: [u8; 4] = [2, 3, 0, 0];
pub const T_SOUTH_Y_VALS: [u8; 4] = [2, 3, 2, 1];
pub const T_WEST_Y_VALS: [u8; 4] = [0, 3, 2, 0];
pub const S_NORTH_SOUTH_Y_VALS: [u8; 4] = [1, 2, 2, 0];
pub const S_EAST_WEST_Y_VALS: [u8; 4] = [0, 3, 2, 0];
pub const Z_NORTH_SOUTH_Y_VALS: [u8; 4] = [2, 2, 1, 0];
pub const Z_EAST_WEST_Y_VALS: [u8; 4] = [0, 2, 3, 0];

pub const I_NORTH_SOUTH_Y_MAX: u8 = 2;
pub const I_EAST_WEST_Y_MAX: u8 = 4;
pub const L_NORTH_Y_MAX: u8 = 2;
pub const L_EAST_Y_MAX: u8 = 3;
pub const L_SOUTH_Y_MAX: u8 = 3;
pub const L_WEST_Y_MAX: u8 = 3;
pub const J_NORTH_Y_MAX: u8 = 2;
pub const J_EAST_Y_MAX: u8 = 3;
pub const J_SOUTH_Y_MAX: u8 = 3;
pub const J_WEST_Y_MAX: u8 = 3;
pub const O_ALL_Y_MAX: u8 = 2;
pub const T_NORTH_Y_MAX: u8 = 2;
pub const T_EAST_Y_MAX: u8 = 3;
pub const T_SOUTH_Y_MAX: u8 = 3;
pub const T_WEST_Y_MAX: u8 = 3;
pub const S_NORTH_SOUTH_Y_MAX: u8 = 2;
pub const S_EAST_WEST_Y_MAX: u8 = 3;
pub const Z_NORTH_SOUTH_Y_MAX: u8 = 2;
pub const Z_EAST_WEST_Y_MAX: u8 = 3;

#[cfg(test)]
mod tests {
    #[test]
    fn piece_orientations() {
        let orientations = [
            ((0u64, 0u64, 0b0001111000000000u64, 0u64), "I_NORTH_SOUTH"),
            (
                (
                    0b0000010000000000,
                    0b0000010000000000,
                    0b0000010000000000,
                    0b0000010000000000,
                ),
                "I_EAST_WEST",
            ),
            ((0, 0, 0b0001110000000000, 0b0001000000000000), "L_NORTH"),
            (
                (
                    0,
                    0b0001100000000000,
                    0b0000100000000000,
                    0b0000100000000000,
                ),
                "L_EAST",
            ),
            ((0, 0b0000010000000000, 0b0001110000000000, 0), "L_SOUTH"),
            (
                (
                    0,
                    0b0000100000000000,
                    0b0000100000000000,
                    0b0000110000000000,
                ),
                "L_WEST",
            ),
            ((0, 0, 0b0001110000000000, 0b0000010000000000), "J_NORTH"),
            (
                (
                    0,
                    0b0000100000000000,
                    0b0000100000000000,
                    0b0001100000000000,
                ),
                "J_EAST",
            ),
            ((0, 0b0001000000000000, 0b0001110000000000, 0), "J_SOUTH"),
            (
                (
                    0,
                    0b0000110000000000,
                    0b0000100000000000,
                    0b0000100000000000,
                ),
                "J_WEST",
            ),
            ((0, 0, 0b0001100000000000, 0b0001100000000000), "O_All"),
            ((0, 0, 0b0001110000000000, 0b0000100000000000), "T_NORTH"),
            (
                (
                    0,
                    0b0000100000000000,
                    0b0001100000000000,
                    0b0000100000000000,
                ),
                "T_EAST",
            ),
            ((0, 0b0000100000000000, 0b0001110000000000, 0), "T_SOUTH"),
            (
                (
                    0,
                    0b0000100000000000,
                    0b0000110000000000,
                    0b0000100000000000,
                ),
                "T_WEST",
            ),
            (
                (0, 0, 0b0000110000000000, 0b0001100000000000),
                "S_NORTH_SOUTH",
            ),
            (
                (
                    0,
                    0b0000100000000000,
                    0b0000110000000000,
                    0b0000010000000000,
                ),
                "S_EAST_WEST",
            ),
            (
                (0, 0, 0b0001100000000000, 0b0000110000000000),
                "Z_NORTH_SOUTH",
            ),
            (
                (
                    0,
                    0b0000010000000000,
                    0b0000110000000000,
                    0b0000100000000000,
                ),
                "Z_EAST_WEST",
            ),
        ];

        for ((r1, r2, r3, r4), apply_to) in orientations {
            let (r1, r2, r3, r4) = (r1 << 3, r2 << 3, r3 << 3, r4 << 3);
            let mask = (r4 << 48) + (r3 << 32) + (r2 << 16) + r1;

            println!("pub const {}: u64 = {mask};", apply_to.to_uppercase());
        }

        println!("\n");

        for ((r1, r2, r3, r4), apply_to) in orientations {
            let (r1, r2, r3, r4) = (r1 as u16, r2 as u16, r3 as u16, r4 as u16);
            let masks = [r4 << 3, r3 << 3, r2 << 3, r1 << 3];

            let mut ys = [0; 4];
            for (y, mask) in masks
                .into_iter()
                .filter(|&v| v != 0)
                .map(|v| v.reverse_bits())
                .enumerate()
            {
                let y = y as u8 + 1;
                (0..4).for_each(|i| {
                    if (mask >> i) & 1 != 0 && ys[i] <= y {
                        ys[i] = y;
                    }
                });
            }

            if r4 == 0 {
                ys.iter_mut().for_each(|v| *v += 1);
            }

            println!(
                "pub const {}_Y_VALS: [u8; 4] = {ys:?};",
                apply_to.to_uppercase()
            );
        }

        println!("\n");

        for ((r1, r2, r3, r4), apply_to) in orientations {
            let (r1, r2, r3, r4) = (r1 as u16, r2 as u16, r3 as u16, r4 as u16);
            let masks = [r4 << 3, r3 << 3, r2 << 3, r1 << 3];

            let mut ys = [0; 4];
            for (y, mask) in masks
                .into_iter()
                .filter(|&v| v != 0)
                .map(|v| v.reverse_bits())
                .enumerate()
            {
                let y = y as u8 + 1;
                (0..4).for_each(|i| {
                    if (mask >> i) & 1 != 0 && ys[i] <= y {
                        ys[i] = y;
                    }
                });
            }

            if r4 == 0 {
                ys.iter_mut().for_each(|v| *v += 1);
            }

            let max = ys.into_iter().max().unwrap_or_default();

            println!("pub const {}_Y_MAX: u8 = {max:?};", apply_to.to_uppercase());
        }
    }
}
