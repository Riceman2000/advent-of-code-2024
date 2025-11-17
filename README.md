# Advent of Code 2024

## Mindset

I did not agree with many of the overbearing design decisions of the existing AOC frameworks so I made my own with even more overbearing design decisions that make it basically unusable for anyone else besides me, use it at your peril.

## Usage
### Run benchmarks with graphics and stats
``` bash
cargo install cargo-criterion
cargo bench --features criterion
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

## Current benchmarks

These benchmarks were done without any true care for accuracy or attempting to control external variables so take them with a grain of salt:

Log scale benchmark results:

![](./media/benchmark-graph.svg)

Results table:

<!-- Table insert start -->
<table>    <thead>        <tr>            <th>                <div>                    <p>                        Day                    </p>                </div>            </th>            <th>                <div>                    <p>                        Validated                    </p>                </div>            </th>            <th>                <div>                    <p>                        Average time                    </p>                </div>            </th>            <th>                <div>                    <p>                        Samples                    </p>                </div>            </th>            <th>                <div>                    <p>                        Total time                    </p>                </div>            </th>        </tr>    </thead>    <tbody>        <tr>            <td>                <div>                    <p>                        aoc2025::day01_0                    </p>                </div>            </td>            <td>                <div>                    <p>                        true                    </p>                </div>            </td>            <td>                <div>                    <p>                        24.705us                    </p>                </div>            </td>            <td>                <div>                    <p>                        2k                    </p>                </div>            </td>            <td>                <div>                    <p>                        50.004ms                    </p>                </div>            </td>        </tr>        <tr>            <td>                <div>                    <p>                        totals                    </p>                </div>            </td>            <td>                <div>                    <p>                        true                    </p>                </div>            </td>            <td>                <div>                    <p>                        24.705us                    </p>                </div>            </td>            <td>                <div>                    <p>                        2k                    </p>                </div>            </td>            <td>                <div>                    <p>                        50.004ms                    </p>                </div>            </td>        </tr>    </tbody></table>
<!-- Table insert end -->
