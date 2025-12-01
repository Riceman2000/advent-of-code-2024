use std::{fs, path::PathBuf};

use charming::{component::Axis, element::AxisType, series::Bar, Chart};
use charming::{theme::Theme, ImageRenderer};
use regex::Regex;
use table_to_html::HtmlTable;

// Avoids lints when leaving out years
#[allow(clippy::wildcard_imports)]
use aoc::*;

const GRAPH_SAVE_LOCATION: &str = "./media/benchmark-graph.svg";
const README_LOCATION: &str = "./README.md";

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
    let ids: Vec<_> = processed.iter().map(|day| day.day_name.clone()).collect();

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
    let mut renderer = ImageRenderer::new(1920, 1080).theme(Theme::Dark);
    renderer
        .save(&chart, GRAPH_SAVE_LOCATION)
        .expect("Failed to render graph");
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
            day.day_name.clone(),
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
