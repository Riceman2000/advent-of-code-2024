#[cfg(feature = "alloc_profile")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

// Generated code made in build.rs makes a list of days
aoc_macros::divan_process_list!();
