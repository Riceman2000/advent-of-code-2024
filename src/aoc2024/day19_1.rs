use std::collections::HashMap;

use rayon::prelude::*;

const INPUT: &str = include_str!("../../input/2024/day19.txt");
aoc_assert::aoc_assert!(601_201_576_113_503);

type Cache = HashMap<&'static str, usize>;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let mut lines = INPUT.trim().lines();
    let substrings: Vec<_> = lines.next().unwrap().split(", ").collect();
    let patterns: Vec<_> = lines.skip(1).collect();

    // Parrallelize iterator
    patterns
        .par_iter()
        .fold(
            || 0,
            |acc, pat| {
                let mut cache = Cache::new();
                acc + get_count(pat, &substrings, &mut cache)
            },
        )
        .sum()
}

fn get_count(pat: &'static str, substrings: &[&str], cache: &mut Cache) -> usize {
    if pat.is_empty() {
        return 1;
    }
    if let Some(v) = cache.get(pat) {
        return *v;
    }

    let count = substrings
        .iter()
        .filter_map(|ss| pat.strip_prefix(ss))
        .map(|remaining| get_count(remaining, substrings, cache))
        .sum();

    cache.insert(pat, count);
    count
}
