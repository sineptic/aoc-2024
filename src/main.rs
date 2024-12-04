use std::io::stdout;

use clap::Parser;

struct FakeOutput {}
impl FakeOutput {
    const fn new() -> FakeOutput {
        FakeOutput {}
    }
}
impl std::io::Write for FakeOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    bench: bool,

    day: usize,
    part: usize,
}

macro_rules! bench {
    ($function:expr, $count:literal) => {
        for _ in 0..$count {
            let _ = $function;
        }
    };
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.bench {
        let input = std::fs::read_to_string(format!("data/day_{}.txt", args.day))?;
        match args.day {
            1 => match args.part {
                1 => bench!(
                    aoc_2024::day1::part_1(&input, &mut FakeOutput::new()),
                    600_000
                ),
                2 => bench!(
                    aoc_2024::day1::part_2(&input, &mut FakeOutput::new()),
                    600_000
                ),
                _ => todo!(),
            },
            2 => match args.part {
                1 => bench!(
                    aoc_2024::day2::part_1(&input, &mut FakeOutput::new()),
                    100_000
                ),
                2 => bench!(
                    aoc_2024::day2::part_2(&input, &mut FakeOutput::new()),
                    100_000
                ),
                _ => todo!(),
            },
            3 => match args.part {
                1 => bench!(aoc_2024::day3::part_1(&input, &mut FakeOutput::new()), 100),
                2 => bench!(aoc_2024::day3::part_2(&input, &mut FakeOutput::new()), 100),
                _ => todo!(),
            },
            4 => match args.part {
                1 => bench!(
                    aoc_2024::day4::part_1(&input, &mut FakeOutput::new()),
                    80_000
                ),
                2 => bench!(
                    aoc_2024::day4::part_2(&input, &mut FakeOutput::new()),
                    80_000
                ),
                _ => todo!(),
            },

            day => panic!("solution {day} not found"),
        };
    } else {
        let input = std::fs::read_to_string("input.txt")?;
        match args.day {
            1 => match args.part {
                1 => aoc_2024::day1::part_1(&input, &mut stdout())?,
                2 => aoc_2024::day1::part_2(&input, &mut stdout())?,
                _ => todo!(),
            },
            2 => match args.part {
                1 => aoc_2024::day2::part_1(&input, &mut stdout())?,
                2 => aoc_2024::day2::part_2(&input, &mut stdout())?,
                _ => todo!(),
            },
            3 => match args.part {
                1 => aoc_2024::day3::part_1(&input, &mut stdout())?,
                2 => aoc_2024::day3::part_2(&input, &mut stdout())?,
                _ => todo!(),
            },
            4 => match args.part {
                1 => aoc_2024::day4::part_1(&input, &mut stdout())?,
                2 => aoc_2024::day4::part_2(&input, &mut stdout())?,
                _ => todo!(),
            },
            day => panic!("day {day} not found"),
        };
    }
    Ok(())
}
