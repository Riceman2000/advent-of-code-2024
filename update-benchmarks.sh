set -e 

BENCHMARK_MS=5000

cargo run --release -- -Bg -m ${BENCHMARK_MS}
git add ./media
git commit -m "Update benchmarks"
