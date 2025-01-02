// Needed to bring in all of the days
#[cfg(feature = "divian")]
#[allow(clippy::wildcard_imports)]
use aoc::*;

#[cfg(feature = "alloc_profile")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

// Generated code made in build.rs makes a list of days
#[cfg(feature = "divian")]
include!(concat!(env!("OUT_DIR"), "/divan_day_list.gen.rs"));
