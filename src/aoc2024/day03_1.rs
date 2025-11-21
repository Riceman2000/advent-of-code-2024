use std::sync::LazyLock;

use regex::Regex;

#[derive(aoc_macros::AocDay)]
#[output_type("u32")]
#[expected_short(None)]
#[expected_long(Some(78_683_433))]
pub struct Day;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap());

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u32 {
    let input = unsafe { str::from_utf8_unchecked(input) };
    let mut sum = 0;
    let mut enabled = true;
    for cap in RE.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ if enabled => {
                let l: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let r: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
                sum += l * r;
            }
            _ => (),
        }
    }

    sum
}
