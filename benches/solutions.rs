use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

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

fn day_1_1(c: &mut Criterion) {
    let input = include_str!("../data/day_1.txt");
    c.bench_function("day_1_1", |b| {
        b.iter(|| aoc_2024::day1::part_1(black_box(input), &mut FakeOutput::new()))
    });
}
fn day_1_2(c: &mut Criterion) {
    let input = include_str!("../data/day_1.txt");
    c.bench_function("day_1_2", |b| {
        b.iter(|| aoc_2024::day1::part_2(black_box(input), &mut FakeOutput::new()))
    });
}
fn day_2_1(c: &mut Criterion) {
    let input = include_str!("../data/day_2.txt");
    c.bench_function("day_2_1", |b| {
        b.iter(|| aoc_2024::day2::part_1(black_box(input), &mut FakeOutput::new()))
    });
}
fn day_2_2(c: &mut Criterion) {
    let input = include_str!("../data/day_2.txt");
    c.bench_function("day_2_2", |b| {
        b.iter(|| aoc_2024::day2::part_2(black_box(input), &mut FakeOutput::new()))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1_000);
    targets = day_1_1, day_1_2, day_2_1, day_2_2,
}
criterion_main!(benches);
