use std::time::Instant;

use atoi::atoi;
use clap::Parser;
use glob_match::glob_match;
use textplots::{Chart, Plot, Shape};

// Needed to bring in all of the days
#[allow(clippy::wildcard_imports)]
use aoc::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct Args {
    /// Which day to test using glob syntax
    #[arg(short, long, default_value_t = String::from("*"))]
    target_day: String,

    /// Disable the display of day outputs
    #[arg(short, long)]
    output_disable: bool,

    /// Print benchmark reports (use criterion for more accurate results)
    #[arg(short, long)]
    bench_enable: bool,

    /// Print out a table of benchmark results
    #[arg(short = 'B', long)]
    bench_table: bool,

    /// Print out a graph of benchmark results
    #[arg(short = 'g', long)]
    bench_graph: bool,

    /// Maximum number of benchmark trials to run
    #[arg(short = 'i', long, default_value_t = 100_000)]
    max_bench_iters: usize,

    /// Max ms to benchmark before canceling
    #[arg(short = 'm', long, default_value_t = 1_000)]
    max_bench_ms: u128,
}

struct DayResult {
    day_name: &'static str,
    day_ran: bool,
    passed_test: bool,
    benchmark: Option<BenchmarkResult>,
}

struct BenchmarkResult {
    iterations: usize,
    average_us: f32,
    average_formatted: String,
    total_formatted: String,
}

/// Please just ignore this, I am sorry for creating it
macro_rules! process_day {
    ($day:ident, $args:expr) => {{
        'runner: {
            let day_name = stringify!($day);

            let day_ran = $args.target_day.is_empty() || glob_match(&$args.target_day, day_name);
            if !day_ran {
                break 'runner DayResult {
                    day_name,
                    day_ran,
                    passed_test: false,
                    benchmark: None,
                };
            }

            if !$args.output_disable && !($args.bench_table || $args.bench_graph) {
                println!("{day_name} -> {}", $day::day());
            }

            // It is best to avoid testing when it wont be reported because it will duplicate user
            // debug statements
            let (benchmark, passed_test) = if $args.bench_table || $args.bench_graph {
                (Some(bench_day($day::day, &$args)), $day::verify_day(false))
            } else if $args.bench_enable {
                println!("Benchmarking {day_name}...");
                (Some(bench_day($day::day, &$args)), $day::verify_day(false))
            } else {
                (None, false)
            };

            DayResult {
                day_name,
                day_ran,
                passed_test,
                benchmark,
            }
        }
    }};
}

#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_precision_loss)]
fn main() {
    let args = Args::parse();

    if args.output_disable && !args.bench_enable {
        eprintln!("No output mode selected, nothing to do");
        return;
    }

    let mut results = Vec::new();

    // All days are processed here, uncomment days to add them
    results.push(process_day!(day01_0, args));
    results.push(process_day!(day01_1, args));
    results.push(process_day!(day02_0, args));
    results.push(process_day!(day02_1, args));
    results.push(process_day!(day03_0, args));
    results.push(process_day!(day03_1, args));
    results.push(process_day!(day04_0, args));
    results.push(process_day!(day04_1, args));
    results.push(process_day!(day05_0, args));
    results.push(process_day!(day05_1, args));
    results.push(process_day!(day06_0, args));
    results.push(process_day!(day06_1, args));
    results.push(process_day!(day07_0, args));
    results.push(process_day!(day07_1, args));
    results.push(process_day!(day08_0, args));
    results.push(process_day!(day08_1, args));
    results.push(process_day!(day09_0, args));
    results.push(process_day!(day09_1, args));
    results.push(process_day!(day10_0, args));
    results.push(process_day!(day10_1, args));
    // results.push(process_day!(day11_0, args));
    // results.push(process_day!(day11_1, args));
    // results.push(process_day!(day12_0, args));
    // results.push(process_day!(day12_1, args));
    // results.push(process_day!(day13_0, args));
    // results.push(process_day!(day13_1, args));
    // results.push(process_day!(day14_0, args));
    // results.push(process_day!(day14_1, args));
    // results.push(process_day!(day15_0, args));
    // results.push(process_day!(day15_1, args));
    // results.push(process_day!(day16_0, args));
    // results.push(process_day!(day16_1, args));
    // results.push(process_day!(day17_0, args));
    // results.push(process_day!(day17_1, args));
    // results.push(process_day!(day18_0, args));
    // results.push(process_day!(day18_1, args));
    // results.push(process_day!(day19_0, args));
    // results.push(process_day!(day19_1, args));
    // results.push(process_day!(day20_0, args));
    // results.push(process_day!(day20_1, args));
    // results.push(process_day!(day21_0, args));
    // results.push(process_day!(day21_1, args));
    // results.push(process_day!(day22_0, args));
    // results.push(process_day!(day22_1, args));
    // results.push(process_day!(day23_0, args));
    // results.push(process_day!(day23_1, args));
    // results.push(process_day!(day24_0, args));
    // results.push(process_day!(day24_1, args));
    // results.push(process_day!(day25_0, args));
    // results.push(process_day!(day25_1, args));

    let processed: Vec<_> = results.iter().filter(|r| r.day_ran).collect();
    if processed.is_empty() {
        eprintln!("Target day did not match anything, nothing was done");
        std::process::exit(1);
    }

    if !(args.bench_enable || args.bench_table || args.bench_graph) {
        return;
    }

    if args.bench_table {
        println!(
            "|   Day   | Validated | Average time per iteration | Number of iterations | Execution time |\n\
             | ------- | --------- | -------------------------- | -------------------- | -------------- |"
        );
    } else if args.bench_enable {
        println!("\nBeginning benchmark reports:");
    }

    for day in &processed {
        let Some(ref benchmark) = day.benchmark else {
            unreachable!("Checked that at least one benchmark option was enabled");
        };
        if args.bench_table {
            println!(
                "| {} | {:>9} | {:>26} | {:>20} | {:>14} |",
                day.day_name,
                day.passed_test,
                benchmark.average_formatted,
                benchmark.iterations,
                benchmark.total_formatted,
            );
        } else if args.bench_enable {
            println!(
                "{} benchmark report: \n\t{} average per iteration \n\t{} iterations \n\t{} total \n\ttest passed: {}",
                day.day_name,
                benchmark.average_formatted,
                benchmark.iterations,
                benchmark.total_formatted,
                day.passed_test,
            );
        }
    }

    if args.bench_graph {
        let times: Vec<_> = processed
            .iter()
            .map(|day| {
                let day_name_bytes = day.day_name.as_bytes();
                // Count by 10, part 2 adds 5
                let index = atoi::<u16>(&day_name_bytes[3..5]).unwrap() * 10
                    + atoi::<u16>(&day_name_bytes[6..]).unwrap() * 5;
                let index = index as f32;

                // Log scale times
                let mut time = day.benchmark.as_ref().unwrap().average_us;
                time = time.log10();

                (index, time)
            })
            .collect();
        println!("\nLog scale graph of timing results\n```");
        Chart::new(180, 60, 0.0, 255.0)
            .lineplot(&Shape::Bars(&times))
            .nice();
        println!("```");
    }
}

/// Benchmark a given day function
/// The signature of this function reads as follows:
/// `process_day` accepts a string literal, a type `F`, and the program arguments.
/// The type generic type `F` allows anything that is a function which
/// returns a generic type `R`.
/// The generic type `R` must be able to be printed to stdout using the `Display` trait.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn bench_day<F: Fn() -> R, R: std::fmt::Display>(function: F, args: &Args) -> BenchmarkResult {
    let mut total_us = 0;
    let mut iterations = 0;
    for i in 0..args.max_bench_iters {
        let start = Instant::now();
        // `black_box` -> Do not optimize out this function
        let _ = std::hint::black_box(function());
        total_us += start.elapsed().as_micros();

        // Limit total execution time
        iterations = i + 1;
        if total_us > args.max_bench_ms * 1_000 {
            break;
        }
    }

    // We shouldn't overflow these unless one of the days is very very slow
    let average_us = total_us as f32 / iterations as f32;
    let total_us = total_us as f32;

    // Metric prefixes
    let average_time = match average_us {
        ..1e3 => format!("{average_us:0.3}us"),
        1e3..1e6 => format!("{:0.3}ms", average_us / 1e3),
        _ => format!("{:0.3}s", average_us / 1e6),
    };
    let total_time = match total_us {
        ..1e3 => format!("{total_us:0.3}us"),
        1e3..1e6 => format!("{:0.3}ms", total_us / 1e3),
        _ => format!("{:0.3}s", total_us / 1e6),
    };

    BenchmarkResult {
        iterations,
        average_us,
        average_formatted: average_time,
        total_formatted: total_time,
    }
}
