use arrayvec::ArrayVec;
use itertools::iproduct;

use crate::utils;

fn parse(input: &str) -> [ArrayVec<(usize, usize), 5>; (128 - b'0') as usize] {
    let input = input.as_bytes();
    let len = utils::get_square_input_len(input.len());
    let mut answer = [const { ArrayVec::new_const() }; _];
    for row in 0..len {
        for col in 0..len {
            match input[row * (len + 1) + col] {
                b'.' => {}
                frequency @ (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') => {
                    answer[(frequency - b'0') as usize].push((row, col));
                }
                _ => panic!("Frequency should be indicated by a single lowercase letter, uppercase letter, or digit."),
            }
        }
    }
    answer
}
fn find_antinode(
    second: (usize, usize),
    diff_row: isize,
    diff_col: isize,
    multiplier: isize,
) -> (usize, usize) {
    (
        second.0.wrapping_add_signed(diff_row * multiplier),
        second.1.wrapping_add_signed(diff_col * multiplier),
    )
}

/// Assumes, that antenas is sorted lexicographical.
fn find_antinodes(
    len: usize,
    antenas: &[(usize, usize)],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..antenas.len(), 0..antenas.len())
        .filter(|(a, b)| a < b)
        .map(|(first, second)| (antenas[first], antenas[second]))
        .flat_map(move |(first, second)| {
            let (diff_row, diff_col) = (
                second.0 as isize - first.0 as isize,
                second.1 as isize - first.1 as isize,
            );
            [
                find_antinode(first, diff_row, diff_col, -1),
                find_antinode(second, diff_row, diff_col, 1),
            ]
        })
        .filter(move |(row, col)| (0..len).contains(row) && (0..len).contains(col))
}
pub fn part_1(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let len = utils::get_square_input_len(input.len());
    let antenas = parse(input);
    let mut antinodes = [[false; 50]; 50];
    antenas
        .iter()
        .flat_map(|antenas| find_antinodes(len, antenas))
        .for_each(|(row, col)| antinodes[row][col] = true);

    let answer = antinodes
        .into_iter()
        .flat_map(|x| x.into_iter())
        .map(|x| x as u64)
        .sum::<u64>();
    writeln!(output, "{answer}")?;
    Ok(())
}
/// Assumes, that antenas is sorted lexicographical.
fn find_antinodes2(
    len: usize,
    antenas: &[(usize, usize)],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    iproduct!(0..antenas.len(), 0..antenas.len())
        .filter(|(a, b)| a < b)
        .map(|(first, second)| (antenas[first], antenas[second]))
        .flat_map(move |(first, second)| {
            let (diff_row, diff_col) = (
                second.0 as isize - first.0 as isize,
                second.1 as isize - first.1 as isize,
            );

            let mut antinodes = Vec::new();
            for i in -100..=100 {
                antinodes.push(find_antinode(first, diff_row, diff_col, i));
            }
            antinodes
        })
        .filter(move |(row, col)| (0..len).contains(row) && (0..len).contains(col))
}

pub fn part_2(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let len = utils::get_square_input_len(input.len());
    let antenas = parse(input);
    let mut antinodes = vec![vec![false; len]; len];
    antenas
        .iter()
        .flat_map(|antenas| find_antinodes2(len, antenas))
        .for_each(|(row, col)| antinodes[row][col] = true);

    let answer = antinodes
        .into_iter()
        .flat_map(|x| x.into_iter())
        .map(|x| x as u64)
        .sum::<u64>();
    writeln!(output, "{answer}")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn day_8_1() {
        let input = INPUT.trim();
        let answer = 14.to_string();
        let mut my_answer = Vec::new();
        part_1(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
    #[test]
    fn day_8_2() {
        let input = INPUT.trim();
        let answer = 34.to_string();
        let mut my_answer = Vec::new();
        part_2(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
}
