use std::time::Instant;

use clap::Parser;
use glob_match::glob_match;
use plotly::{
    common::Title,
    layout::{Axis, Margin},
    traces::table::{Cells, Header},
    Bar, ImageFormat, Layout, Plot, Table,
};

// Needed to bring in all of the days
#[allow(clippy::wildcard_imports)]
use aoc::*;

const GRAPH_SAVE_LOCATION: &str = "./media/benchmark-graph.png";
const TABLE_SAVE_LOCATION: &str = "./media/benchmark-table.png";

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
    average_ns: f32,
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
    results.push(process_day!(day11_0, args));
    results.push(process_day!(day11_1, args));
    results.push(process_day!(day12_0, args));
    results.push(process_day!(day12_1, args));
    results.push(process_day!(day13_0, args));
    results.push(process_day!(day13_1, args));
    results.push(process_day!(day14_0, args));
    results.push(process_day!(day14_1, args));
    results.push(process_day!(day15_0, args));
    results.push(process_day!(day15_1, args));
    results.push(process_day!(day16_0, args));
    results.push(process_day!(day16_1, args));
    results.push(process_day!(day17_0, args));
    results.push(process_day!(day17_1, args));
    results.push(process_day!(day18_0, args));
    results.push(process_day!(day18_1, args));
    results.push(process_day!(day19_0, args));
    results.push(process_day!(day19_1, args));
    results.push(process_day!(day20_0, args));
    results.push(process_day!(day20_1, args));
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

    // Benchmarks text reports
    if args.bench_enable {
        for day in &processed {
            let Some(ref benchmark) = day.benchmark else {
                unreachable!("Checked that at least one benchmark option was enabled");
            };
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

    if args.bench_table {
        println!("Generating benchmark table");
        generate_bench_table(&processed);
        println!("Benchmark table saved to {TABLE_SAVE_LOCATION}");
    }

    // Benchmarks graph
    if args.bench_graph {
        println!("Generating benchmark graph");
        generate_bench_graph(&processed);
        println!("Benchmark plot saved to {GRAPH_SAVE_LOCATION}");
    }
}

fn generate_bench_graph(processed: &[&DayResult]) {
    let ids: Vec<_> = processed.iter().map(|day| day.day_name).collect();

    let times: Vec<_> = processed
        .iter()
        .map(|day| {
            // Log scale times
            let mut time = day.benchmark.as_ref().unwrap().average_ns;
            time = time.log10();
            time
        })
        .collect();

    let mut plot = Plot::new();
    let trace = Bar::new(ids, times);
    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::from("Day")))
        .y_axis(Axis::new().title(Title::from("Runtime in log10 nanoseconds")))
        .title(Title::from("Benchmark results logscale"));
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.write_image(GRAPH_SAVE_LOCATION, ImageFormat::PNG, 800, 600, 1.0);
}

fn generate_bench_table(processed: &[&DayResult]) {
    let mut table_rows = Vec::new();
    for day in processed {
        let Some(ref benchmark) = day.benchmark else {
            unreachable!("Checked that at least one benchmark option was enabled");
        };
        table_rows.push(vec![
            day.day_name.to_string(),
            day.passed_test.to_string(),
            benchmark.average_formatted.to_string(),
            benchmark.iterations.to_string(),
            benchmark.total_formatted.to_string(),
        ]);
    }
    // Table rows must be transposed
    let total_rows = table_rows.len();
    table_rows = (0..table_rows[0].len())
        .map(|i| {
            table_rows
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<_>>()
        })
        .collect();

    // Form plot
    let mut plot = Plot::new();
    let cells = Cells::new(table_rows);
    let header = Header::new(vec![
        "Day",
        "Validated",
        "Iteration time",
        "Iterations",
        "Total time",
    ]);
    let trace = Table::new(header, cells);
    let margin = Margin::new().left(5).right(5).top(50).bottom(5);
    let layout = Layout::new()
        .title(Title::from("Benchmark results table"))
        .margin(margin);
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.write_image(
        TABLE_SAVE_LOCATION,
        ImageFormat::PNG,
        800,
        85 + 20 * total_rows,
        1.0,
    );
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
    let mut total_ns = 0;
    let mut iterations = 0;
    for i in 0..args.max_bench_iters {
        let start = Instant::now();
        // `black_box` -> Do not optimize out this function
        let _ = std::hint::black_box(function());
        total_ns += start.elapsed().as_nanos();

        // Limit total execution time
        iterations = i + 1;
        if total_ns > args.max_bench_ms * 10u128.pow(6) {
            break;
        }
    }

    // We shouldn't overflow these unless one of the days is very very slow
    let average_ns = total_ns as f32 / iterations as f32;
    let total_ns = total_ns as f32;

    // Metric prefixes
    let average_formatted = match average_ns {
        ..1e3 => format!("{average_ns:0.3}ns"),
        1e3..1e6 => format!("{:0.3}us", average_ns / 1e3),
        1e6..1e9 => format!("{:0.3}ms", average_ns / 1e6),
        _ => format!("{:0.3}s", average_ns / 1e9),
    };
    let total_formatted = match total_ns {
        ..1e3 => format!("{total_ns:0.3}ns"),
        1e3..1e6 => format!("{:0.3}us", total_ns / 1e3),
        1e6..1e9 => format!("{:0.3}ms", total_ns / 1e6),
        _ => format!("{:0.3}s", total_ns / 1e9),
    };

    BenchmarkResult {
        iterations,
        average_ns,
        average_formatted,
        total_formatted,
    }
}
