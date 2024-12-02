#![feature(iter_collect_into, portable_simd)]

pub mod day1;
pub mod day2 {
    use std::io::Write;

    use anyhow::Result;
    use itertools::Itertools;
    use nom::{bytes::complete::tag, multi::separated_list0};

    fn parse(input: &str) -> Vec<Vec<u8>> {
        use nom::character::complete::u8;
        // TODO: Use iterator to create SmallVec instead of Vec.
        let (_tail, a) =
            separated_list0(tag::<_, _, ()>("\n"), separated_list0(tag(" "), u8))(input.as_bytes())
                .unwrap();
        debug_assert!(_tail.is_empty());
        a
    }

    fn is_valid(level: i32) -> bool {
        (1..=3).contains(&level)
    }
    fn validate_report(report: Vec<u8>, has_extra_attempt: bool) -> bool {
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
        let input = parse(input.trim());
        let answer: u32 = input
            .into_iter()
            .map(|report| validate_report(report, false) as u32)
            .sum();

        writeln!(output, "{answer}")?;
        Ok(())
    }

    pub fn part_2(input: &str, output: &mut impl Write) -> Result<()> {
        let input = parse(input);

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
        #[test]
        fn day_2_1() {
            let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
            .trim();
            let answer = "
2
    "
            .trim();
            let mut my_answer = Vec::new();
            part_1(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
        #[test]
        fn day_2_2() {
            let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
            .trim();
            let answer = "
4
    "
            .trim();
            let mut my_answer = Vec::new();
            part_2(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
    }
}
