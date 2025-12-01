set -e 

BENCHMARK_MS=5000

cargo run --release --features "plotting" -- -Bg -m ${BENCHMARK_MS}
git add ./media ./README.md
git commit -m "Update benchmarks"
