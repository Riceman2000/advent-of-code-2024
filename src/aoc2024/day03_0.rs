use std::sync::LazyLock;

use regex::Regex;

const INPUT: &str = include_str!("../../input/2024/day03.txt");
aoc_macros::aoc_assert!(170_068_701);

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let mut sum = 0;
    for cap in RE.captures_iter(INPUT) {
        let l: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let r: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        sum += l * r;
    }

    sum
}
