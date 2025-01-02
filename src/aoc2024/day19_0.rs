use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/2024/day19.txt");
aoc_assert::aoc_assert!(226);

lazy_static! {
    static ref RE: Regex = {
        let substrings: Vec<_> = INPUT.trim().lines().next().unwrap().split(", ").collect();
        let regex_str = format!("^({})+$", substrings.join("|"));
        Regex::new(&regex_str).unwrap()
    };
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let patterns: Vec<_> = INPUT.trim().lines().skip(2).collect();

    let mut possible = 0;
    for pat in patterns {
        if RE.find(pat).is_some() {
            possible += 1;
        }
    }

    possible
}
