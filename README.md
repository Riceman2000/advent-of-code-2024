# Advent of Code 2024

## Mindset

This framework is meant to be easy to expand and iterate on. I hope to foster a collaborative atmosphere for solving these problems and make this a public repository for basic Rust.

## Current benchmarks

These benchmarks were done without any true care for accuracy or attempting to control external variables so take them with a grain of salt:

|   Day   | Validated | Average time per iteration | Number of iterations | Execution time |
| ------- | --------- | -------------------------- | -------------------- | -------------- |
| day01_0 |      true |                   25.153us |               100000 |         2.515s |
| day01_1 |      true |                   49.027us |               100000 |         4.903s |
| day02_0 |      true |                   97.979us |                51032 |         5.000s |
| day02_1 |      true |                  111.489us |                44848 |         5.000s |
| day03_0 |      true |                   79.985us |                62512 |         5.000s |
| day03_1 |      true |                  130.751us |                38241 |         5.000s |
| day04_0 |      true |                  355.807us |                14053 |         5.000s |
| day04_1 |      true |                   99.384us |                50310 |         5.000s |
| day05_0 |      true |                   42.939us |               100000 |         4.294s |
| day05_1 |      true |                  121.522us |                41145 |         5.000s |
| day06_0 |      true |                   83.447us |                59919 |         5.000s |
| day06_1 |      true |                    4.205ms |                 1190 |         5.003s |
| day07_0 |      true |                    1.219ms |                 4103 |         5.002s |
| day07_1 |      true |                   86.118ms |                   59 |         5.081s |
| day08_0 |      true |                   18.392us |               100000 |         1.839s |
| day08_1 |      true |                   39.368us |               100000 |         3.937s |
| day09_0 |      true |                  257.941us |                19385 |         5.000s |
| day09_1 |      true |                   36.315ms |                  138 |         5.011s |
| day10_0 |      true |                  102.058us |                48992 |         5.000s |
| day10_1 |      true |                   21.083us |               100000 |         2.108s |

Log scale benchmark results:

![](./media/benchmark-graph.png)

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

### Enable benchmarks
``` bash
cargo run --release -- -b
```

### Generate the benchmark results table
``` bash
cargo run --release -- -B
```

### Generate the benchmark results graph
``` bash
cargo run --release -- -g
```
