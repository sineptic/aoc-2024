use std::io::{stdout, Stdout};

use clap::Parser;

type Solution = Box<dyn Fn(&str, &mut Stdout) -> anyhow::Result<()>>;

#[derive(Parser)]
struct Args {
    day: usize,
}
fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let days: Vec<Solution> = vec![Box::new(aoc_2024::day1::solve)];

    let args = Args::parse();

    days[args.day - 1](input, &mut stdout())?;
    Ok(())
}
