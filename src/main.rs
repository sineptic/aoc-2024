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
        "2_1" => aoc_2024::day2::part_1(input, &mut stdout())?,
        "2_2" => aoc_2024::day2::part_2(input, &mut stdout())?,
        day => panic!("solution {day} not found"),
    }

    Ok(())
}

// struct FakeOutput {}
// impl FakeOutput {
//     const fn new() -> FakeOutput {
//         FakeOutput {}
//     }
// }
// impl std::io::Write for FakeOutput {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         Ok(buf.len())
//     }
//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }
// fn main() {
//     let input = include_str!("../input.txt");
//     for _ in 0..400_000 {
//         let _ = aoc_2024::day1::part_2(input, &mut FakeOutput::new());
//     }
// }
