# Advent of Code 2024

## Mindset

This framework is meant to be easy to expand and iterate on. I hope to foster a collaborative atmosphere for solving these problems and make this a public repository for basic Rust.

## Current benchmarks

These benchmarks were done without any true care for accuracy or attempting to control external variables so take them with a grain of salt:

|   Day   | Average time per iteration | Number of iterations | Execution time |
| ------- | -------------------------- | -------------------- | -------------- |
| day01_0 |                   24.908us |               100000 |         2.491s |
| day01_1 |                   50.587us |                98841 |         5.000s |
| day02_0 |                   98.932us |                50540 |         5.000s |
| day02_1 |                  120.958us |                41337 |         5.000s |
| day03_0 |                   82.764us |                60413 |         5.000s |
| day03_1 |                  135.290us |                36958 |         5.000s |
| day04_0 |                  362.843us |                13781 |         5.000s |
| day04_1 |                  101.029us |                49491 |         5.000s |
| day05_0 |                   43.073us |               100000 |         4.307s |
| day05_1 |                  123.428us |                40510 |         5.000s |
| day06_0 |                   84.885us |                58904 |         5.000s |
| day06_1 |                   63.558ms |                   79 |         5.021s |

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
