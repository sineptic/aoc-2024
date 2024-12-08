use std::{
    cmp::min,
    io::Write,
    simd::{cmp::SimdPartialEq, u8x16, u8x32, Mask, Simd},
    sync::LazyLock,
};

use anyhow::Result;
use itertools::Itertools;
use smallvec::SmallVec;

fn generate_lookup_table() -> Vec<Simd<u8, 16>> {
    let mut answer = vec![u8x16::default(); 2_usize.pow(16)];
    for mut i in 0..=u16::MAX {
        let i_copy = i;
        let mut curr_answer = Vec::new();
        let mut shifted = 0;
        while i.leading_zeros() != 16 {
            let temp = i.leading_zeros() + 1;
            shifted += temp;
            curr_answer.push((shifted - 1) as u8);
            i = i.unbounded_shl(temp);
        }
        curr_answer.resize(16, 0);
        answer[i_copy as usize] = u8x16::from_slice(&curr_answer);
    }
    answer
}

fn parse_line_simd(line: &[u8]) -> SmallVec<[u8; 8]> {
    // eprintln!("input:       '{line:<32}'");

    const WHITESPACES: u8x32 = u8x32::from_array([b' '; 32]);

    let mut buf = [b' '; 32];
    buf[0..line.len()].copy_from_slice(line);

    let line = u8x32::from_array(buf);

    let whitespaces = line.simd_eq(WHITESPACES);
    let _whitespaces_bitmask = whitespaces.to_bitmask() as u32;

    let is_digit = !whitespaces;
    let is_digit_bitmask = is_digit.to_bitmask() as u32;

    let tens_bitmask = is_digit_bitmask >> 1 & is_digit_bitmask;
    let ones_bitmask = is_digit_bitmask ^ tens_bitmask;

    let numbers = line - u8x32::splat(b'0');
    let ones_mask = Mask::from_bitmask(ones_bitmask as u64);
    let ones = ones_mask.select(numbers, u8x32::splat(0));
    let tens = (Mask::from_bitmask(tens_bitmask as u64)).select(numbers, u8x32::splat(0))
        * u8x32::splat(10);
    let digits = tens + ones;
    let numbers_with_garbage = digits + digits.rotate_elements_right::<1>();
    let numbers = ones_mask.select(numbers_with_garbage, u8x32::splat(0));

    static LOOKUP_TABLE: LazyLock<Vec<u8x16>> = LazyLock::new(generate_lookup_table);

    let numbers_part0 = numbers.to_array()[0..16].try_into().unwrap();
    let numbers_part0 = u8x16::from_array(numbers_part0);
    let numbers_part0_bitmask = (ones_bitmask.reverse_bits() >> 16) as u16;
    let numbers_part0 = numbers_part0
        .swizzle_dyn(LOOKUP_TABLE[numbers_part0_bitmask as usize])
        .to_array();

    let numbers_part1 = numbers.to_array()[16..32].try_into().unwrap();
    let numbers_part1 = u8x16::from_array(numbers_part1);
    let numbers_part1_bitmask = ones_bitmask.reverse_bits() as u16;
    let numbers_part1 = numbers_part1
        .swizzle_dyn(LOOKUP_TABLE[numbers_part1_bitmask as usize])
        .to_array();

    numbers_part0
        .into_iter()
        .take(numbers_part0_bitmask.count_ones() as usize)
        .chain(
            numbers_part1
                .into_iter()
                .take(numbers_part1_bitmask.count_ones() as usize),
        )
        .collect()
    // answer.extend_from_slice(&numbers_part0[0..numbers_part0_bitmask.count_ones() as usize]);
    // answer.extend_from_slice(&numbers_part1[0..numbers_part1_bitmask.count_ones() as usize]);

    // eprintln!("whitespaces: '{:032b}'", _whitespaces_bitmask.reverse_bits());
    // eprintln!("is number:   '{:032b}'", is_digit_bitmask.reverse_bits());
    // eprintln!("tens:        '{:032b}'", tens_bitmask.reverse_bits());
    // eprintln!("ones:        '{:032b}'", ones_bitmask.reverse_bits());
}
fn parse_simd(input_str: &str) -> Vec<SmallVec<[u8; 8]>> {
    let input = input_str.as_bytes();
    let mut prev_newline = 0;
    let mut answer = Vec::with_capacity(1024);
    loop {
        let mut buf = [0; 32];
        buf[..(min(32, input.len() - prev_newline))]
            .copy_from_slice(&input[prev_newline..(min(prev_newline + 32, input.len()))]);
        let simd_input = u8x32::from_array(buf);
        let next_newline = prev_newline
            + (simd_input
                .simd_eq(u8x32::from_array([b'\n'; 32]))
                .to_bitmask() as u32)
                .reverse_bits()
                .leading_zeros() as usize;
        if next_newline >= input.len() {
            answer.push(parse_line_simd(&input[prev_newline..input.len()]));
            return answer;
        }
        answer.push(parse_line_simd(&input[prev_newline..next_newline]));

        prev_newline = next_newline + 1;
    }
    // input.lines().map(parse_line_simd).collect()
}

fn is_valid(level: i32) -> bool {
    (1..=3).contains(&level)
}
fn validate_report(report: SmallVec<[u8; 8]>, has_extra_attempt: bool) -> bool {
    fn is_increasing(mut diffs: Vec<i32>, mut has_extra_attempt: bool) -> bool {
        for i in 0..diffs.len() {
            let item = diffs[i];
            if let Some(&next) = diffs.get(i + 1) {
                if !is_valid(next) && has_extra_attempt {
                    if diffs.get(i + 2).is_none_or(|x| is_valid(next + x)) {
                    } else {
                        has_extra_attempt = false;
                        diffs[i + 1] += diffs[i];
                        continue;
                    }
                }
            }
            if !is_valid(item) {
                if has_extra_attempt {
                    has_extra_attempt = false;

                    if i == 0 {
                        continue;
                    }
                    if i + 1 < diffs.len() {
                        diffs[i + 1] += item;
                        continue;
                    }
                    if i + 1 == diffs.len() {
                        continue;
                    }
                    unreachable!();
                } else {
                    return false;
                }
            }
        }
        true
    }
    let diffs = report
        .windows(2)
        .map(|nums| nums[1] as i32 - nums[0] as i32)
        .collect_vec();

    let increasing = is_increasing(diffs.clone(), has_extra_attempt);
    let decreasing = is_increasing(diffs.into_iter().map(|x| -x).collect(), has_extra_attempt);

    increasing || decreasing
}

pub fn part_1(input: &str, output: &mut impl Write) -> Result<()> {
    let input = parse_simd(input.trim());
    let answer: u32 = input
        .into_iter()
        .map(|report| validate_report(report, false) as u32)
        .sum();

    writeln!(output, "{answer}")?;
    Ok(())
}

pub fn part_2(input: &str, output: &mut impl Write) -> Result<()> {
    let input = parse_simd(input.trim());

    let answer: u32 = input
        .into_iter()
        .map(|report| validate_report(report, true) as u32)
        .sum();

    writeln!(output, "{answer}")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    const SMALL_INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    test_solution!(day_2_1_small, part_1, SMALL_INPUT, 2);
    test_solution!(day_2_2_small, part_2, SMALL_INPUT, 4);

    const BIG_INPUT: &str = include_str!("../data/day_2.txt");
    test_solution!(day_2_1_big, part_1, BIG_INPUT, 341);
    test_solution!(day_2_2_big, part_2, BIG_INPUT, 404);
}
