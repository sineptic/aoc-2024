use std::{
    io::Write,
    simd::{num::SimdUint, u32x8, u8x8},
};

use anyhow::Result;

fn parse_line(line: &[u8]) -> (u32, u32) {
    debug_assert!(13 <= line.len() && line.len() <= 14);
    const WEIGHTS: u32x8 = u32x8::from_slice(&[10000u32, 1000u32, 100u32, 10u32, 1u32, 0, 0, 0]);
    const ZERO: u32x8 = u32x8::from_slice(&[b'0' as u32; 8]);
    let left_simd: u32x8 = u8x8::load_or_default(&line[..5]).cast();
    let right_simd: u32x8 = u8x8::load_or_default(&line[8..13]).cast();
    (
        ((left_simd - ZERO) * WEIGHTS).reduce_sum(),
        ((right_simd - ZERO) * WEIGHTS).reduce_sum(),
    )
}
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.trim().as_bytes().chunks(14).map(parse_line).unzip()
}

pub fn part_1(input: &str, output: &mut impl Write) -> Result<()> {
    let (mut first, mut second) = parse(input);

    first.sort_unstable();
    second.sort_unstable();

    let answer = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>();

    writeln!(output, "{answer}")?;

    Ok(())
}

pub fn part_2(input: &str, output: &mut impl Write) -> Result<()> {
    let mut left = Vec::with_capacity(1024);
    // from 10'000 to 99'999
    let mut right = [0_u8; 100_000];

    input.as_bytes().chunks(14).for_each(|line| {
        let (l, r) = parse_line(line);
        left.push(l);
        right[r as usize] += 1;
    });

    let answer = left
        .into_iter()
        .map(|x| x * right[x as usize] as u32)
        .sum::<u32>();

    writeln!(output, "{answer}")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    const SMALL_INPUT: &str = "
00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003
";
    test_solution!(day_1_1_small, part_1, SMALL_INPUT, 11);
    test_solution!(day_1_2_small, part_2, SMALL_INPUT, 31);
}
