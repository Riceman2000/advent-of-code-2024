# Advent of Code 2024

## Mindset

This framework is meant to be easy to expand and iterate on. I hope to foster a collaborative atmosphere for solving these problems and make this a public repository for basic Rust.

## Current benchmarks

These benchmarks were done without any true care for accuracy or attempting to control external variables so take them with a grain of salt:

|   Day   | Average time per iteration | Number of iterations | Execution time |
| ------- | -------------------------- | -------------------- | -------------- |
| day01_0 |                   23.899us |               100000 |         2.390s |
| day01_1 |                   42.167us |               100000 |         4.217s |
| day02_0 |                   98.020us |                51010 |         5.000s |
| day02_1 |                  112.217us |                44557 |         5.000s |
| day03_0 |                   78.491us |                63702 |         5.000s |
| day03_1 |                  130.440us |                38332 |         5.000s |
| day04_0 |                  369.325us |                13539 |         5.000s |
| day04_1 |                  100.575us |                49714 |         5.000s |
| day05_0 |                   45.681us |               100000 |         4.568s |
| day05_1 |                  122.679us |                40757 |         5.000s |
| day06_0 |                   82.313us |                60745 |         5.000s |
| day06_1 |                    3.918ms |                 1277 |         5.003s |
| day07_0 |                    1.150ms |                 4349 |         5.001s |
| day07_1 |                   80.848ms |                   62 |         5.013s |
| day08_0 |                   18.916us |               100000 |         1.892s |
| day08_1 |                   39.633us |               100000 |         3.963s |
| day09_0 |                  259.799us |                19246 |         5.000s |
| day09_1 |                   35.998ms |                  139 |         5.004s |

## Notes

- When adding a day you must:
  - Copy an existing day file into the `src` directory
    - Ensure the numbering scheme is followed
  - Uncomment the newly added day file in the following places
    - `src/main.rs` -> to allow you to run the day using `cargo run`
    - `src/lib.rs` -> to include the new code in the binary
    - `benches/days.rs` -> to allow for benchmarking using `cargo criterion`

When developing solutions you will likely want to run using `cargo run` without the `--release` flag for faster compile times. For benchmarking however you should use the `--release` flag to get the highest level of optimization. Criterion will automatically use a high level of optimization.

## Usage
### Run benchmarks with graphics and stats
``` bash
cargo install cargo-criterion
cargo criterion
```
output will be in `target/criterion/index.html`

### See the CLI help menu
``` bash
cargo run -- --help
```

### Run individual days
``` bash
cargo run -- --target-day day01_0
```

### Run days based on a glob
``` bash
cargo run -- --target-day "day01_*"
```

### Run all days with their outputs hidden
``` bash
cargo run --release -- -o
```

### Run all days without running benchmarks
``` bash
cargo run --release -- -b
```

### Generate the benchmark results table
``` bash
cargo run --release -- -B
```
