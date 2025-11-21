use std::sync::LazyLock;

use regex::Regex;

#[derive(aoc_macros::AocDay)]
#[output_type("u32")]
#[expected_short(None)]
#[expected_long(Some(170_068_701))]
pub struct Day;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u32 {
    let input = unsafe { str::from_utf8_unchecked(input) };
    let mut sum = 0;
    for cap in RE.captures_iter(input) {
        let l: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let r: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        sum += l * r;
    }

    sum
}
