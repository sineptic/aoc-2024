use std::io::stdout;

use clap::Parser;

#[derive(Parser)]
struct Args {
    day: String,
}
fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let args = Args::parse();

    match args.day.trim() {
        "1_1" => aoc_2024::day1::part_1(input, &mut stdout())?,
        "1_2" => aoc_2024::day1::part_2(input, &mut stdout())?,
        day => panic!("solution {day} not found"),
    }

    Ok(())
}
