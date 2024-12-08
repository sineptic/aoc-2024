use fxhash::{FxBuildHasher, FxHashSet};
use itertools::Itertools;

use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardPosition {
    pub row: isize,
    pub col: isize,
}
impl GuardPosition {
    fn is_in_boundaries(&self, len: usize) -> bool {
        (0..len as isize).contains(&self.row) && (0..len as isize).contains(&self.col)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pub position: GuardPosition,
    pub speed: GuardSpeed,
}
impl Guard {
    fn new(row: usize, col: usize) -> Self {
        Self {
            position: GuardPosition {
                row: row as isize,
                col: col as isize,
            },
            speed: GuardSpeed::default(),
        }
    }
    fn next_position(&self) -> GuardPosition {
        GuardPosition {
            row: self.position.row + self.speed.y,
            col: self.position.col + self.speed.x,
        }
    }
    fn step(&mut self) {
        self.position = self.next_position();
    }
    fn rotate_right(&mut self) {
        let a = match (self.speed.y, self.speed.x) {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => unreachable!(),
        };

        self.speed.y = a.0;
        self.speed.x = a.1;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardSpeed {
    x: isize,
    y: isize,
}
impl Default for GuardSpeed {
    fn default() -> Self {
        Self { x: 0, y: -1 }
    }
}

fn parse_maze(input: &str) -> (Vec<Vec<bool>>, Guard) {
    let input = input.as_bytes();
    let len = utils::get_square_input_len(input.len());

    let mut guard = None;

    let mut maze = vec![vec![false; len]; len];
    for row in 0..len {
        for col in 0..len {
            let char = input[row * (len + 1) + col];
            maze[row][col] = char == b'#';
            if char == b'^' {
                guard = Some(Guard::new(row, col));
            }
        }
    }

    (maze, guard.unwrap())
}

pub fn part_1(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let (maze, mut guard) = parse_maze(input);
    let len = maze.len();
    let mut visited = vec![vec![false; len]; len];
    while guard.position.is_in_boundaries(len) {
        visited[guard.position.row as usize][guard.position.col as usize] = true;
        let next = guard.next_position();
        if !next.is_in_boundaries(len) {
            break;
        }
        if maze[guard.next_position().row as usize][guard.next_position().col as usize] {
            guard.rotate_right();
        }
        guard.step();
    }

    let answer = visited
        .into_iter()
        .flatten()
        .map(|x| x as usize)
        .sum::<usize>();

    writeln!(output, "{answer}")?;
    Ok(())
}
pub fn part_2(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let (mut maze, guard) = parse_maze(input);
    let len = maze.len();
    let answer = (0..len)
        .cartesian_product(0..len)
        .filter(|(row, col)| {
            !(*row == guard.position.row as usize && *col == guard.position.col as usize)
        })
        .map(|(row, col)| {
            let initial = maze[row][col];
            maze[row][col] = true;

            let is_loop = is_loop(&maze, guard);

            maze[row][col] = initial;

            is_loop as usize
        })
        .sum::<usize>();

    writeln!(output, "{answer}")?;
    Ok(())
}
fn is_loop(maze: &[Vec<bool>], mut guard: Guard) -> bool {
    let len = maze.len();
    let mut states = FxHashSet::with_capacity_and_hasher(len * 100, FxBuildHasher::new());
    loop {
        if !guard.next_position().is_in_boundaries(len) {
            return false;
        }
        while maze[guard.next_position().row as usize][guard.next_position().col as usize] {
            guard.rotate_right();
        }

        if !states.insert(guard) {
            return true;
        }

        guard.step();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_solution;

    const SMALL_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    test_solution!(day_6_1_small, part_1, SMALL_INPUT, 41);
    test_solution!(day_6_2_small, part_2, SMALL_INPUT, 6);
}
