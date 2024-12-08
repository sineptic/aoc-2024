use std::io::Write;

use memchr::memmem::Finder;
use nom::{bytes::complete::tag, character::complete::u64, sequence::tuple};

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

// TODO:
pub fn part_2(_input: &str, _output: &mut impl Write) -> anyhow::Result<()> {
    todo!()
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    const SMALL_INPUT: &str = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    test_solution!(day_3_1_small, part_1, SMALL_INPUT, 161);
    // test_solution!(day_3_2_small, part_2, SMALL_INPUT, todo!());
}
