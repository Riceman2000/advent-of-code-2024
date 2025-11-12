#![feature(array_windows)]
#![feature(iter_map_windows)]
#![feature(slice_split_once)]
#![feature(iter_array_chunks)]

use std::io::Write;
use std::sync::LazyLock;
use std::time::Instant;

use clap::Parser;

automod::dir!(pub "src/");

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Which day to test using glob syntax
    #[arg(short, long, default_value_t = String::from("*"))]
    pub target_day: String,

    /// Disable the display of day outputs
    #[arg(short, long)]
    pub output_disable: bool,

    /// Print benchmark reports (use criterion for more accurate results)
    #[arg(short, long)]
    pub bench_enable: bool,

    /// Print out a table of benchmark results
    #[arg(short = 'B', long)]
    pub bench_table: bool,

    /// Print out a graph of benchmark results
    #[arg(short = 'g', long)]
    pub bench_graph: bool,

    /// Maximum number of benchmark trials to run
    #[arg(short = 'i', long, default_value_t = 100_000)]
    pub max_bench_iters: usize,

    /// Max ms to benchmark before canceling
    #[arg(short = 'm', long, default_value_t = 1_000)]
    pub max_bench_ms: u128,

    /// Max ms to warmup before benchmarking
    #[arg(short = 'w', long, default_value_t = 500)]
    pub max_warmup_ms: u128,
}

struct DayResult {
    day_name: &'static str,
    day_ran: bool,
    passed_test: bool,
    benchmark: Option<BenchmarkResult>,
}

struct BenchmarkResult {
    samples: usize,
    average_ns: f32,
    total_ns: f32,
}

struct ExpectedValues<T> {
    short: T,
    long: T,
}

struct InputValues<T> {
    short: T,
    long: T,
}

trait AocDay {
    type InputType;
    type OutputType: std::cmp::PartialEq + std::fmt::Display;

    fn day(input: Self::InputType) -> Self::OutputType;
    fn expected_values() -> ExpectedValues<Self::OutputType>;
    fn input_values() -> InputValues<Self::InputType>;

    fn verify_short() -> bool {
        Self::day(Self::input_values().short) == Self::expected_values().short
    }
    fn verify_long() -> bool {
        Self::day(Self::input_values().long) == Self::expected_values().long
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn bench_day() -> BenchmarkResult {
        let args = &ARGS;

        // Warm up for a few samples to prep caches
        let mut warmup_ns = 0;
        loop {
            let start = Instant::now();
            // `black_box` -> Do not optimize out this function
            let _ = std::hint::black_box(Self::day(Self::input_values().long));
            warmup_ns += start.elapsed().as_nanos();

            // Limit total warmup time
            if warmup_ns > args.max_warmup_ms * 10u128.pow(6) {
                break;
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();

        let mut total_ns = 0;
        let mut samples = 0;
        for i in 0..args.max_bench_iters {
            let start = Instant::now();
            // `black_box` -> Do not optimize out this function
            let _ = std::hint::black_box(Self::day(Self::input_values().long));
            total_ns += start.elapsed().as_nanos();

            // Limit total execution time
            samples = i + 1;
            if total_ns > args.max_bench_ms * 10u128.pow(6) {
                break;
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();

        // We shouldn't overflow these unless one of the days is very very slow
        let average_ns = total_ns as f32 / samples as f32;
        let total_ns = total_ns as f32;
        println!(" Done.");

        BenchmarkResult {
            samples,
            average_ns,
            total_ns,
        }
    }
}
