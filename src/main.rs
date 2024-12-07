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
macro_rules! day_bench_wrapper {
    ($path:ident, $first_count:literal, $second_count:literal, $args:ident, $input:ident) => {{
        match $args.part {
            1 => bench!($path::part_1(&$input, &mut FakeOutput::new()), $first_count),
            2 => bench!(
                $path::part_2(&$input, &mut FakeOutput::new()),
                $second_count
            ),
            part => panic!("unknown part {part}"),
        }
    }};
}
macro_rules! day_wrapper {
    ($path:ident, $args:ident, $input:ident) => {{
        match $args.part {
            1 => $path::part_1(&$input, &mut stdout())?,
            2 => $path::part_2(&$input, &mut stdout())?,
            _ => todo!(),
        }
    }};
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.bench {
        let input = std::fs::read_to_string(format!("data/day_{}.txt", args.day))?;
        use aoc_2024::*;
        match args.day {
            1 => day_bench_wrapper!(day1, 600_000, 600_000, args, input),
            2 => day_bench_wrapper!(day2, 100_000, 100_000, args, input),
            3 => day_bench_wrapper!(day3, 100, 100, args, input),
            4 => day_bench_wrapper!(day4, 80_000, 80_000, args, input),
            5 => day_bench_wrapper!(day5, 200_000, 40_000, args, input),
            6 => day_bench_wrapper!(day6, 100, 20, args, input),
            7 => day_bench_wrapper!(day7, 100, 30, args, input),

            day => panic!("solution {day} not found"),
        };
    } else {
        let input = std::fs::read_to_string("input.txt")?;
        use aoc_2024::*;
        match args.day {
            1 => day_wrapper!(day1, args, input),
            2 => day_wrapper!(day2, args, input),
            3 => day_wrapper!(day3, args, input),
            4 => day_wrapper!(day4, args, input),
            5 => day_wrapper!(day5, args, input),
            6 => day_wrapper!(day6, args, input),
            7 => day_wrapper!(day7, args, input),

            day => panic!("day {day} not found"),
        };
    }
    Ok(())
}
