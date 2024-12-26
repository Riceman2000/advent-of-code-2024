# Advent of Code 2024

## Mindset

This framework is meant to be easy to expand and iterate on. I hope to foster a collaborative atmosphere for solving these problems and make this a public repository for basic Rust.

## Notes

- You will have to install the nightly toolchain if prompted to run the latest Rust has to offer
  - That is done via `rustup toolchain add [toolchain-name-from-error-message]`
- When adding a day you must:
  - Copy an existing day 
  - Edit the new days `Cargo.toml` so that the crate name matches the name of the day file
  - Uncomment lines in the `runner` crate
    - in `runner/Cargo.toml`
    - in `runner/src/lib.rs`
  - Check success with the `./checks -b` command

## Usage
### Run benchmarks
``` bash
./checks -b
```

### Use the `checks` script to do various useful things
``` bash
./checks -h
```

### Run individual days
``` bash
cd day01_0
cargo +nightly run --release
```

### Run all days in parallel
``` bash
cd runner
cargo +nightly run --release --bin runner-par
```

## Credit
- This video from [ThePrimegan](https://youtu.be/U16RnpV48KQ)
- Benchmarking code/inspiration from [timvisee](https://github.com/timvisee/advent-of-code-2022)
