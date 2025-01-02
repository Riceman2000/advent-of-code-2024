use divan::black_box;

// Needed to bring in all of the days
#[allow(clippy::wildcard_imports)]
use aoc::*;

#[cfg(feature = "alloc_profile")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

// Generated benchmark framework made in build.rs
include!(concat!(env!("OUT_DIR"), "/divan_day_list.gen.rs"));
