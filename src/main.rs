use std::io::Write;
use std::{fs, path::PathBuf, time::Instant};

use charming::{component::Axis, element::AxisType, series::Bar, Chart};
use charming::{theme::Theme, ImageRenderer};
use glob_match::glob_match;
use regex::Regex;
use table_to_html::HtmlTable;

// Avoids lints when leaving out years
#[allow(clippy::wildcard_imports)]
use aoc::*;

const GRAPH_SAVE_LOCATION: &str = "./media/benchmark-graph.svg";
const README_LOCATION: &str = "./README.md";


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

impl BenchmarkResult {
    // Metric prefixes
    fn average_formatted(&self) -> String {
        let average_ns = self.average_ns;
        match average_ns {
            ..1e3 => format!("{average_ns:0.3}ns"),
            1e3..1e6 => format!("{:0.3}us", average_ns / 1e3),
            1e6..1e9 => format!("{:0.3}ms", average_ns / 1e6),
            _ => format!("{:0.3}s", average_ns / 1e9),
        }
    }
    fn iterations_formatted(&self) -> String {
        let samples = self.samples;
        match samples {
            ..1_000 => format!("{samples:0.1}"),
            1_000..1_000_000 => format!("{:0.3}k", samples / 1_000),
            1_000_000..1_000_000_000 => format!("{:0.3}M", samples / 1_000_000),
            _ => format!("{:0.3}G", samples / 1_000_000_000),
        }
    }
    fn total_formatted(&self) -> String {
        let total_ns = self.total_ns;
        match total_ns {
            ..1e3 => format!("{total_ns:0.3}ns"),
            1e3..1e6 => format!("{:0.3}us", total_ns / 1e3),
            1e6..1e9 => format!("{:0.3}ms", total_ns / 1e6),
            _ => format!("{:0.3}s", total_ns / 1e9),
        }
    }
}

#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_precision_loss)]
fn main() {
    let args = &aoc::ARGS;

    if args.output_disable && !args.bench_enable {
        eprintln!("No output mode selected, nothing to do");
        return;
    }

    let mut results = Vec::new();

    // Generates list of days based on file structure
    aoc_macros::day_process_list!();

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
                benchmark.average_formatted(),
                benchmark.samples,
                benchmark.total_formatted(),
                day.passed_test,
            );
        }
    }

    if args.bench_table {
        println!("Generating benchmark table");
        generate_bench_table(&processed);
        println!("Benchmark table saved to README");
    }

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

    let chart = Chart::new()
        .y_axis(Axis::new().type_(AxisType::Category).data(ids))
        .x_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(times));
    let mut renderer = ImageRenderer::new(600, 450).theme(Theme::Westeros);
    renderer
        .save(&chart, GRAPH_SAVE_LOCATION)
        .expect("Failed to render graph");

    // let mut plot = Plot::new();
    // let trace = Bar::new(ids, times);
    // let layout = Layout::new()
    //     .x_axis(Axis::new().title(Title::from("Day")))
    //     .y_axis(Axis::new().title(Title::from("Runtime in log10 nanoseconds")))
    //     .title(Title::from("Benchmark results logscale"));
    // plot.add_trace(trace);
    // plot.set_layout(layout);
    // plot.write_html(GRAPH_SAVE_LOCATION);
}

fn generate_bench_table(processed: &[&DayResult]) {
    let mut table_rows = Vec::new();
    table_rows.push(vec![
        "Day".to_string(),
        "Validated".to_string(),
        "Average time".to_string(),
        "Samples".to_string(),
        "Total time".to_string(),
    ]);

    let mut total_bench_results = BenchmarkResult {
        samples: 0,
        total_ns: 0.0,
        average_ns: 0.0,
    };
    let mut all_validated = true;

    for day in processed {
        let Some(ref benchmark) = day.benchmark else {
            unreachable!("Checked that at least one benchmark option was enabled");
        };

        // Collect totals
        total_bench_results.samples += benchmark.samples;
        total_bench_results.average_ns += benchmark.average_ns;
        total_bench_results.total_ns += benchmark.total_ns;
        if !day.passed_test {
            all_validated = false;
        }

        // Log current
        table_rows.push(vec![
            day.day_name.to_string(),
            day.passed_test.to_string(),
            benchmark.average_formatted(),
            benchmark.iterations_formatted(),
            benchmark.total_formatted(),
        ]);
    }
    table_rows.push(vec![
        "totals".to_string(),
        all_validated.to_string(),
        total_bench_results.average_formatted(),
        total_bench_results.iterations_formatted(),
        total_bench_results.total_formatted(),
    ]);
    let html_table = HtmlTable::with_header(table_rows);

    // Convert to string without newlines
    let table_str: String = html_table
        .to_string()
        .chars()
        .filter(|c| *c != '\n')
        .collect();

    // Find and replace tags in README
    let re =
        Regex::new(r"(<!-- Table insert start -->\n)[\s\S]*(\n<!-- Table insert end -->)").unwrap();
    let readme = PathBuf::from(README_LOCATION);
    assert!(readme.is_file(), "README not present");
    let readme_content = fs::read(&readme).expect("Failed to read README content");
    let mut readme_content = String::from_utf8(readme_content).expect("Invalid README");
    readme_content = re
        .replace(&readme_content, format!("$1{table_str}$2"))
        .to_string();
    fs::write(&readme, readme_content).expect("Failed to write README");
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
    // Warm up for a few samples to prep caches
    let mut warmup_ns = 0;
    loop {
        let start = Instant::now();
        // `black_box` -> Do not optimize out this function
        let _ = std::hint::black_box(function());
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
        let _ = std::hint::black_box(function());
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
