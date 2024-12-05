use std::time::Instant;

use clap::Parser;
use glob_match::glob_match;

// Needed to bring in all of the days
#[allow(clippy::wildcard_imports)]
use aoc::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day to test using glob syntax
    #[arg(short, long, default_value_t = String::from("*"))]
    target_day: String,

    /// Disable the display of day outputs
    #[arg(short, long)]
    output_disable: bool,

    /// Disable benchmarks (use criterion for more accurate results)
    #[arg(short, long)]
    bench_disable: bool,

    /// Print out a table of benchmark results, disables everything else
    #[arg(short = 'B', long)]
    bench_table: bool,

    /// Maximum number of benchmark trials to run
    #[arg(short = 'i', long, default_value_t = 100_000)]
    max_bench_iters: usize,

    /// Max ms to benchmark before canceling
    #[arg(short = 'm', long, default_value_t = 1_000)]
    max_bench_ms: u128,
}

fn main() {
    let args = Args::parse();

    if args.output_disable && args.bench_disable {
        eprintln!("No output mode selected, nothing to do");
        return;
    }
    if args.bench_table {
        println!(
            "|   Day   | Average time per iteration | Number of iterations | Execution time |"
        );
        println!(
            "| ------- | -------------------------- | -------------------- | -------------- |"
        );
    }

    let mut total_proc = 0;

    // All days are processed here, uncomment days to add them
    total_proc += process_day("day01_0", day01_0::day, &args);
    total_proc += process_day("day01_1", day01_1::day, &args);
    total_proc += process_day("day02_0", day02_0::day, &args);
    total_proc += process_day("day02_1", day02_1::day, &args);
    total_proc += process_day("day03_0", day03_0::day, &args);
    total_proc += process_day("day03_1", day03_1::day, &args);
    total_proc += process_day("day04_0", day04_0::day, &args);
    total_proc += process_day("day04_1", day04_1::day, &args);
    total_proc += process_day("day05_0", day05_0::day, &args);
    // total_proc += process_day("day05_1", day05_1::day, &args);
    // total_proc += process_day("day06_0", day06_0::day, &args);
    // total_proc += process_day("day06_1", day06_1::day, &args);
    // total_proc += process_day("day07_0", day07_0::day, &args);
    // total_proc += process_day("day07_1", day07_1::day, &args);
    // total_proc += process_day("day08_0", day08_0::day, &args);
    // total_proc += process_day("day08_1", day08_1::day, &args);
    // total_proc += process_day("day09_0", day09_0::day, &args);
    // total_proc += process_day("day09_1", day09_1::day, &args);
    // total_proc += process_day("day10_0", day10_0::day, &args);
    // total_proc += process_day("day10_1", day10_1::day, &args);
    // total_proc += process_day("day11_0", day11_0::day, &args);
    // total_proc += process_day("day11_1", day11_1::day, &args);
    // total_proc += process_day("day12_0", day12_0::day, &args);
    // total_proc += process_day("day12_1", day12_1::day, &args);
    // total_proc += process_day("day13_0", day13_0::day, &args);
    // total_proc += process_day("day13_1", day13_1::day, &args);
    // total_proc += process_day("day14_0", day14_0::day, &args);
    // total_proc += process_day("day14_1", day14_1::day, &args);
    // total_proc += process_day("day15_0", day15_0::day, &args);
    // total_proc += process_day("day15_1", day15_1::day, &args);
    // total_proc += process_day("day16_0", day16_0::day, &args);
    // total_proc += process_day("day16_1", day16_1::day, &args);
    // total_proc += process_day("day17_0", day17_0::day, &args);
    // total_proc += process_day("day17_1", day17_1::day, &args);
    // total_proc += process_day("day18_0", day18_0::day, &args);
    // total_proc += process_day("day18_1", day18_1::day, &args);
    // total_proc += process_day("day19_0", day19_0::day, &args);
    // total_proc += process_day("day19_1", day19_1::day, &args);
    // total_proc += process_day("day20_0", day20_0::day, &args);
    // total_proc += process_day("day20_1", day20_1::day, &args);
    // total_proc += process_day("day21_0", day21_0::day, &args);
    // total_proc += process_day("day21_1", day21_1::day, &args);
    // total_proc += process_day("day22_0", day22_0::day, &args);
    // total_proc += process_day("day22_1", day22_1::day, &args);
    // total_proc += process_day("day23_0", day23_0::day, &args);
    // total_proc += process_day("day23_1", day23_1::day, &args);
    // total_proc += process_day("day24_0", day24_0::day, &args);
    // total_proc += process_day("day24_1", day24_1::day, &args);
    // total_proc += process_day("day25_0", day25_0::day, &args);
    // total_proc += process_day("day25_1", day25_1::day, &args);

    if total_proc == 0 {
        eprintln!("Target day did not match anything, nothing was done");
    }
}

/// Process a given day, perform different actions for that day depending on the user args.
/// The signature of this function reads as follows:
/// `process_day` accepts a string literal, a type `F`, and the program arguments.
/// The type generic type `F` allows anything that is a function which
/// returns a generic type `R`.
/// The generic type `R` must be able to be printed to stdout using the `Display` trait.
#[allow(clippy::cast_precision_loss)]
fn process_day<F: Fn() -> R, R: std::fmt::Display>(day: &str, function: F, args: &Args) -> u8 {
    if !args.target_day.is_empty() && !glob_match(&args.target_day, day) {
        return 0;
    }

    if !args.output_disable && !args.bench_table {
        println!("Output of {day} -> {}", function());
    }

    if !args.bench_disable {
        let mut total_us = 0;
        let mut actual_iterations = 0;
        for i in 0..args.max_bench_iters {
            let start = Instant::now();
            // `black_box` -> Do not optimize out this function
            let _ = std::hint::black_box(function());
            total_us += start.elapsed().as_micros();

            // Limit total execution time
            actual_iterations = i + 1;
            if total_us > args.max_bench_ms * 1_000 {
                break;
            }
        }

        // We shouldn't overflow these unless one of the days is very very slow
        let average_us = total_us as f64 / actual_iterations as f64;
        let total_secs: f64 = total_us as f64 / 1e6;

        if args.bench_table {
            println!(
                "| {day} | {average_us:>24.3}us | {actual_iterations:>20} | {total_secs:>13.3}s |"
            );
        } else {
            println!("{day} -> {average_us:0.2}us per iteration {total_secs:0.3}s total");
        }
    }

    1
}
