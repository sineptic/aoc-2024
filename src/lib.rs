#![feature(
    iter_collect_into,
    portable_simd,
    unbounded_shifts,
    strict_overflow_ops
)]

mod utils {
    pub fn _parse_digit(input: &[u8]) -> (u8, &[u8]) {
        (input[0] - b'0', &input[1..])
    }
    pub fn parse_2digit(input: &[u8]) -> (u8, &[u8]) {
        ((input[0] - b'0') * 10 + input[1] - b'0', &input[2..])
    }
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
