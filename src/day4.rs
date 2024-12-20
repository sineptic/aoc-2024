use std::{io::Write, sync::LazyLock};

use itertools::Itertools;
use memchr::memmem::Finder;
use smallvec::SmallVec;

pub fn part_1(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();
    let len = lines[0].len();

    let mut diag1 = vec![b' '; (2 * len - 1) * (len + 1)];
    let mut diag2 = vec![b' '; (2 * len - 1) * (len + 1)];
    let mut vert = vec![b' '; len * (len + 1)];
    let mut horiz = vec![b' '; len * (len + 1)];
    for row in 0..len {
        for col in 0..len {
            diag1[(len - 1 + row - col) * (len + 1) + col] = lines[row][col];
            diag2[(row + col) * (len + 1) + col] = lines[row][col];
            vert[col * (len + 1) + row] = lines[row][col];
            horiz[row * (len + 1) + col] = lines[row][col];
        }
    }

    fn find_xmas(input: &[u8]) -> usize {
        static XMAS: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"XMAS"));
        static SAMX: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"SAMX"));
        XMAS.find_iter(input).count() + SAMX.find_iter(input).count()
    }

    let answer = find_xmas(&horiz) + find_xmas(&vert) + find_xmas(&diag1) + find_xmas(&diag2);
    writeln!(output, "{answer}")?;
    Ok(())
}
pub fn part_2(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    fn find_possible_matches(input: &[u8]) -> impl Iterator<Item = usize> + '_ {
        static MAS: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"MAS"));
        static SAM: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"SAM"));
        MAS.find_iter(input)
            .map(|x| x + 1)
            .chain(SAM.find_iter(input).map(|x| x + 1))
    }

    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();
    let len = lines[0].len();

    let mut diag1 = vec![b' '; (2 * len - 1) * (len + 1)];
    let mut diag2 = vec![b' '; (2 * len - 1) * (len + 1)];
    for row in 0..len {
        for col in 0..len {
            diag1[(len - 1 + row - col) * (len + 1) + col] = lines[row][col];
            diag2[(row + col) * (len + 1) + col] = lines[row][col];
        }
    }

    let diag1 = find_possible_matches(&diag1).map(|offset| {
        let (row, col) = (offset / (len + 1), offset % (len + 1));
        let initial_row = row + col - (len - 1);
        initial_row * (len + 1) + col
    });
    let diag2 = find_possible_matches(&diag2).map(|offset| {
        let (row, col) = (offset / (len + 1), offset % (len + 1));
        let initial_row = row - col;
        initial_row * (len + 1) + col
    });

    let answer = part2_find_answer(len, diag1, diag2);

    writeln!(output, "{answer}")?;
    Ok(())
}
fn part2_find_answer(
    len: usize,
    diag1: impl Iterator<Item = usize>,
    diag2: impl Iterator<Item = usize>,
) -> i32 {
    let mut table = SmallVec::<[bool; 2_usize.pow(16)]>::new();
    table.resize((len + 1) * len, false);

    for offset in diag1 {
        table[offset] = true;
    }
    let mut answer = 0;
    for offset in diag2 {
        if table[offset] {
            answer += 1;
        }
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    const SMALL_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    test_solution!(day_4_1_small, part_1, SMALL_INPUT, 18);
    test_solution!(day_4_2_small, part_2, SMALL_INPUT, 9);

    const BIG_INPUT: &str = include_str!("../data/day_4.txt");
    test_solution!(day_4_1_big, part_1, BIG_INPUT, 2454);
    test_solution!(day_4_2_big, part_2, BIG_INPUT, 1858);
}
