use criterion::{criterion_group, criterion_main, Criterion};

criterion_main!(benches);

criterion_group! {
    name=benches;
    config=Criterion::default();
    targets=day_benches
}

fn day_benches(_c: &mut Criterion) {
    aoc_macros::criterion_process_list!();
}
