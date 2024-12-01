#![feature(exact_size_is_empty, iter_collect_into, portable_simd)]

pub mod day1 {
    use std::{
        io::Write,
        simd::{num::SimdUint, u32x8, u8x8},
    };

    use anyhow::Result;
    use itertools::Itertools;

    fn parse_line(line: &[u8]) -> (u32, u32) {
        debug_assert!(13 <= line.len() && line.len() <= 14);
        const WEIGHTS: u32x8 =
            u32x8::from_slice(&[10000u32, 1000u32, 100u32, 10u32, 1u32, 0, 0, 0]);
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
        let (mut first, mut second) = parse(input);
        first.sort_unstable();
        second.sort_unstable();

        let mut first = first.into_iter();
        let mut second = second.into_iter();
        let mut answer = 0;
        loop {
            if first.is_empty() || second.is_empty() {
                break;
            }
            let current_number = first.next().unwrap();
            let first_count = first.take_while_ref(|x| *x == current_number).count() + 1;

            second
                .take_while_ref(|x| *x < current_number)
                .for_each(|_| {});
            let second_count = second.take_while_ref(|x| *x == current_number).count();

            answer += current_number as usize * first_count * second_count;
        }

        writeln!(output, "{answer}")?;

        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn day_1_1() {
            let input = "
00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003
"
            .trim();
            let answer = "
11
"
            .trim();
            let mut my_answer = Vec::new();
            part_1(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
        #[test]
        fn day_1_2() {
            let input = "
00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003
    "
            .trim();
            let answer = "
31
    "
            .trim();
            let mut my_answer = Vec::new();
            part_2(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
    }
}
