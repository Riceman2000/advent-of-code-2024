use std::ops::RangeInclusive;

use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(14))]
#[expected_long(Some(357_674_099_117_260))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let mut ranges: Vec<RangeInclusive<usize>> = input
        .split(|c| *c == b'\n')
        .take_while(|l| !l.is_empty())
        .map(|l| unsafe {
            let mut l = l.split(|c| *c == b'-');
            let a = atoi(l.next().unwrap_unchecked()).unwrap_unchecked();
            let b = atoi(l.next().unwrap_unchecked()).unwrap_unchecked();
            a..=b
        })
        .collect();

    let mut dedupe_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    ranges.sort_unstable_by_key(|r| *r.start());
    for r in ranges {
        if let Some(loc) = dedupe_ranges
            .iter()
            .position(|dr| dr.contains(r.start()) || dr.contains(r.end()))
        {
            let mod_dr = &mut dedupe_ranges[loc];
            *mod_dr = *mod_dr.start().min(r.start())..=*mod_dr.end().max(r.end());
        } else {
            dedupe_ranges.push(r);
        }
    }

    dedupe_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}
