# Advent of Code 2024

## Mindset

This framework is meant to be easy to expand and iterate on. I hope to foster a collaborative atmosphere for solving these problems and make this a public repository for basic Rust.

## Current benchmarks

These benchmarks were done without any true care for accuracy or attempting to control external variables so take them with a grain of salt:

|   Day   | Average time per iteration | Number of iterations | Execution time |
| ------- | -------------------------- | -------------------- | -------------- |
| day01_0 |                   24.782us |               100000 |         2.478s |
| day01_1 |                   43.359us |               100000 |         4.336s |
| day02_0 |                  102.141us |                48952 |         5.000s |
| day02_1 |                  110.065us |                45428 |         5.000s |
| day03_0 |                   79.984us |                62513 |         5.000s |
| day03_1 |                  132.943us |                37611 |         5.000s |
| day04_0 |                  371.265us |                13468 |         5.000s |
| day04_1 |                   80.382us |                62203 |         5.000s |
| day05_0 |                   42.943us |               100000 |         4.294s |
| day05_1 |                  123.319us |                40546 |         5.000s |
| day06_0 |                   82.619us |                60519 |         5.000s |
| day06_1 |                    4.476ms |                 1118 |         5.004s |
| day07_0 |                    1.257ms |                 3979 |         5.000s |
| day07_1 |                   86.745ms |                   58 |         5.031s |
| day08_0 |                   18.853us |               100000 |         1.885s |
| day08_1 |                   40.227us |               100000 |         4.023s |

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
