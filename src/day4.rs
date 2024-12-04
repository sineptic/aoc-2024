use std::{collections::HashSet, io::Write, sync::LazyLock};

use itertools::Itertools;
use memchr::memmem::Finder;

pub fn part_1(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();
    let len = lines[0].len();

    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();
    let mut vert = Vec::new();
    diag1.resize_with(2 * len - 1, || {
        let mut a = Vec::new();
        a.resize(len, b' ');
        a
    });
    diag2.resize_with(2 * len - 1, || {
        let mut a = Vec::new();
        a.resize(len, b' ');
        a
    });
    vert.resize_with(len, || {
        let mut a = Vec::new();
        a.resize(len, b' ');
        a
    });
    for row in 0..len {
        for col in 0..len {
            diag1[len - 1 + row - col][col] = lines[row][col];
            diag2[row + col][col] = lines[row][col];
            vert[col][row] = lines[row][col];
        }
    }
    let horiz = lines;

    fn find_xmas(input: &[u8]) -> usize {
        static XMAS: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"XMAS"));
        static SAMX: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"SAMX"));
        XMAS.find_iter(input).count() + SAMX.find_iter(input).count()
    }

    let horiz = horiz.into_iter().map(find_xmas).sum::<usize>();
    let diag1 = diag1.into_iter().map(|x| find_xmas(&x)).sum::<usize>();
    let diag2 = diag2.into_iter().map(|x| find_xmas(&x)).sum::<usize>();
    let vert = vert.into_iter().map(|x| find_xmas(&x)).sum::<usize>();

    let answer = horiz + vert + diag1 + diag2;
    writeln!(output, "{answer}")?;
    Ok(())
}
pub fn part_2(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();
    let len = lines[0].len();
    let mut diag1 = Vec::new();
    let mut diag2 = Vec::new();
    diag1.resize_with(2 * len - 1, || {
        let mut a = Vec::new();
        a.resize(len, b' ');
        a
    });
    diag2.resize_with(2 * len - 1, || {
        let mut a = Vec::new();
        a.resize(len, b' ');
        a
    });
    for row in 0..len {
        for col in 0..len {
            diag1[len - 1 + row - col][col] = lines[row][col];
            diag2[row + col][col] = lines[row][col];
        }
    }

    fn find_mas(input: &[u8]) -> impl Iterator<Item = usize> + '_ {
        static MAS: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"MAS"));
        static SAM: LazyLock<Finder> = LazyLock::new(|| Finder::new(b"SAM"));
        MAS.find_iter(input)
            .map(|x| x + 1)
            .chain(SAM.find_iter(input).map(|x| x + 1))
    }

    let diag1 = diag1
        .iter()
        .enumerate()
        .flat_map(|(row, chars)| find_mas(chars).map(move |col| (row + col - (len - 1), col)))
        .collect::<HashSet<_>>();
    let diag2 = diag2
        .iter()
        .enumerate()
        .flat_map(|(row, chars)| find_mas(chars).map(move |col| (row - col, col)))
        .collect::<HashSet<_>>();

    let answer = diag1.intersection(&diag2).count();
    writeln!(output, "{answer}")?;
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
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

    #[test]
    fn day_4_1() {
        let input = INPUT.trim();
        let answer = 18.to_string();
        let mut my_answer = Vec::new();
        part_1(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
    #[test]
    fn day_4_2() {
        let input = INPUT.trim();
        let answer = 9.to_string();
        let mut my_answer = Vec::new();
        part_2(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
}
