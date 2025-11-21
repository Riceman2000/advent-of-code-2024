use std::sync::LazyLock;

use regex::Regex;

#[derive(aoc_macros::AocDay)]
#[output_type("u32")]
#[expected_short(None)]
#[expected_long(Some(226))]
pub struct Day;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    let substrings: Vec<_> = include_str!("../../input/2024/day19.txt")
        .trim()
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .collect();
    let regex_str = format!("^({})+$", substrings.join("|"));
    Regex::new(&regex_str).unwrap()
});

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u32 {
    let input = unsafe { str::from_utf8_unchecked(input) };
    let patterns: Vec<_> = input.trim().lines().skip(2).collect();

    let mut possible = 0;
    for pat in patterns {
        if RE.find(pat).is_some() {
            possible += 1;
        }
    }

    possible
}
