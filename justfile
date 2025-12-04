default:
  @just --list

all: spell format-fix test

# Testing
alias t := test

test target="":
  cargo test {{target}}

# Spelling
alias s := spell
alias sf := spell-fix
alias sw := spell-watch

spell:
  typos --sort

spell-fix:
  typos -w

spell-watch:
  watchexec "clear && typos --sort"

# Formatting
alias f := format-fix
alias fc := format-check
alias format := format-fix

format-check:
  cargo fmt --check
  
format-fix:
  cargo fmt

# Bench
alias b := bench
alias bf := bench-flame
alias bc := bench-criterion
alias bd := bench-divan
alias bg := bench-graphics

bench target="'*'":
  cargo run --release -- -t {{target}} -b

bench-flame target="'*'" bench-ms="5000":
  cargo flamegraph --profile prof -- -t {{target}} -b -m {{bench-ms}}

bench-criterion target="":
  cargo bench --features criterion {{target}}

bench-divan target="":
  cargo bench --features divan {{target}}

bench-graphics bench-ms="5000":
  cargo run --release --features plotting -- -Bg -m {{bench-ms}}
  git add ./media ./README.md
  git commit -m "Update benchmarks"
