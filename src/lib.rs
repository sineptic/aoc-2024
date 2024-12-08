#![feature(
    iter_collect_into,
    portable_simd,
    unbounded_shifts,
    strict_overflow_ops,
    mixed_integer_ops_unsigned_sub,
    generic_arg_infer
)]

mod utils {
    pub fn _parse_digit(input: &[u8]) -> (u8, &[u8]) {
        (input[0] - b'0', &input[1..])
    }
    pub fn parse_2digit(input: &[u8]) -> (u8, &[u8]) {
        ((input[0] - b'0') * 10 + input[1] - b'0', &input[2..])
    }
    pub fn get_square_input_len(size: usize) -> usize {
        // x^2 + x = size + 1
        // x^2 + x - size - 1 = 0
        // D = 1 + 4 * size + 4 = 4 * size + 5
        // x = (sqrt(5 + 4 * sizse) - 1) / 2
        ((((size * 4 + 5) as f64).sqrt() - 1.) / 2.) as usize
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
