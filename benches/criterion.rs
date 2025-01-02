use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "criterion")]
#[allow(clippy::wildcard_imports)]
use aoc::*;

criterion_main!(benches);

criterion_group! {
    name=benches;
    config=Criterion::default();
    targets=day_benches
}

fn day_benches(_c: &mut Criterion) {
    #[cfg(feature = "criterion")]
    // Generated code made in build.rs makes a list of days
    include!(concat!(env!("OUT_DIR"), "/criterion_day_list.gen.rs"));
}
