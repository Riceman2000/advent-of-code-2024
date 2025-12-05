use std::ops::RangeInclusive;

use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(3))]
#[expected_long(Some(770))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let mut input = input.split(|c| *c == b'\n');

    let ranges: Vec<RangeInclusive<usize>> = input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| unsafe {
            let mut l = l.split(|c| *c == b'-');
            let a = atoi(l.next().unwrap_unchecked()).unwrap_unchecked();
            let b = atoi(l.next().unwrap_unchecked()).unwrap_unchecked();
            a..=b
        })
        .collect();

    input
        .filter(|l| {
            let Some(n) = atoi(l) else { return false };
            ranges.iter().any(|r| r.contains(&n))
        })
        .count()
}
