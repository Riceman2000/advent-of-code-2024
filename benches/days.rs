use criterion::{criterion_group, criterion_main, Criterion};

#[allow(clippy::wildcard_imports)]
use aoc::*;

criterion_group! {
    name=benches;
    config=Criterion::default();
    targets=day_benches
}
criterion_main!(benches);

fn day_benches(c: &mut Criterion) {
    // To add days to the criterion benchmark platform uncomment them here
    c.bench_function("day01_0", |b| b.iter(&mut day01_0::day));
    c.bench_function("day01_1", |b| b.iter(&mut day01_1::day));
    c.bench_function("day02_0", |b| b.iter(&mut day02_0::day));
    c.bench_function("day02_1", |b| b.iter(&mut day02_1::day));
    c.bench_function("day03_0", |b| b.iter(&mut day03_0::day));
    c.bench_function("day03_1", |b| b.iter(&mut day03_1::day));
    c.bench_function("day04_0", |b| b.iter(&mut day04_0::day));
    c.bench_function("day04_1", |b| b.iter(&mut day04_1::day));
    c.bench_function("day05_0", |b| b.iter(&mut day05_0::day));
    c.bench_function("day05_1", |b| b.iter(&mut day05_1::day));
    c.bench_function("day06_0", |b| b.iter(&mut day06_0::day));
    c.bench_function("day06_1", |b| b.iter(&mut day06_1::day));
    c.bench_function("day07_0", |b| b.iter(&mut day07_0::day));
    c.bench_function("day07_1", |b| b.iter(&mut day07_1::day));
    c.bench_function("day08_0", |b| b.iter(&mut day08_0::day));
    c.bench_function("day08_1", |b| b.iter(&mut day08_1::day));
    c.bench_function("day09_0", |b| b.iter(&mut day09_0::day));
    c.bench_function("day09_1", |b| b.iter(&mut day09_1::day));
    c.bench_function("day10_0", |b| b.iter(&mut day10_0::day));
    c.bench_function("day10_1", |b| b.iter(&mut day10_1::day));
    c.bench_function("day11_0", |b| b.iter(&mut day11_0::day));
    c.bench_function("day11_1", |b| b.iter(&mut day11_1::day));
    c.bench_function("day12_0", |b| b.iter(&mut day12_0::day));
    c.bench_function("day12_1", |b| b.iter(&mut day12_1::day));
    c.bench_function("day13_0", |b| b.iter(&mut day13_0::day));
    c.bench_function("day13_1", |b| b.iter(&mut day13_1::day));
    c.bench_function("day14_0", |b| b.iter(&mut day14_0::day));
    c.bench_function("day14_1", |b| b.iter(&mut day14_1::day));
    c.bench_function("day15_0", |b| b.iter(&mut day15_0::day));
    c.bench_function("day15_1", |b| b.iter(&mut day15_1::day));
    c.bench_function("day16_0", |b| b.iter(&mut day16_0::day));
    c.bench_function("day16_1", |b| b.iter(&mut day16_1::day));
    // c.bench_function("day17_0", |b| b.iter(&mut day17_0::day));
    // c.bench_function("day17_1", |b| b.iter(&mut day17_1::day));
    // c.bench_function("day18_0", |b| b.iter(&mut day18_0::day));
    // c.bench_function("day18_1", |b| b.iter(&mut day18_1::day));
    // c.bench_function("day19_0", |b| b.iter(&mut day19_0::day));
    // c.bench_function("day19_1", |b| b.iter(&mut day19_1::day));
    // c.bench_function("day20_0", |b| b.iter(&mut day20_0::day));
    // c.bench_function("day20_1", |b| b.iter(&mut day20_1::day));
    // c.bench_function("day21_0", |b| b.iter(&mut day21_0::day));
    // c.bench_function("day21_1", |b| b.iter(&mut day21_1::day));
    // c.bench_function("day22_0", |b| b.iter(&mut day22_0::day));
    // c.bench_function("day22_1", |b| b.iter(&mut day22_1::day));
    // c.bench_function("day23_0", |b| b.iter(&mut day23_0::day));
    // c.bench_function("day23_1", |b| b.iter(&mut day23_1::day));
    // c.bench_function("day24_0", |b| b.iter(&mut day24_0::day));
    // c.bench_function("day24_1", |b| b.iter(&mut day24_1::day));
    // c.bench_function("day25_0", |b| b.iter(&mut day25_0::day));
    // c.bench_function("day25_1", |b| b.iter(&mut day25_1::day));
}
