#![feature(iter_collect_into, portable_simd, unbounded_shifts)]

pub mod day1;
pub mod day2;
pub mod day3 {
    use std::io::Write;

    use memchr::memmem::Finder;
    use nom::{
        bytes::complete::tag,
        character::complete::u64,
        combinator::iterator,
        multi::separated_list1,
        sequence::{delimited, separated_pair, terminated, tuple},
        AsBytes, IResult,
    };

    pub fn part_1(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
        let mut input = input.as_bytes();
        let finder = Finder::new(b"mul(");
        let mut checker = tuple((tag(b"mul("), u64::<&[u8], ()>, tag(b","), u64, tag(b")")));
        let mut sum = 0;
        while let Some(a) = finder.find(input) {
            if let Ok((remainder, (_, x, _, y, _))) = checker(&input[a..]) {
                input = remainder;
                sum += x * y;
            } else {
                input = &input[(a + 1)..]
            }
        }
        writeln!(output, "{sum}")?;
        Ok(())
    }
    pub fn part_2(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
        todo!()
    }
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn day_3_1() {
            let input = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"
            .trim();
            let answer = "
161
"
            .trim();
            let mut my_answer = Vec::new();
            part_1(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
        #[test]
        fn day_3_2() {
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
