use std::collections::HashMap;

use rayon::prelude::*;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../../input/2024/day19.txt");

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

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 601_201_576_113_503;

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        let sign = if actual > expected { '+' } else { '-' };
        eprintln!(
            "Got {actual} expected {expected}, diff {sign}{}",
            expected.abs_diff(actual)
        );
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that is automatically run by `cargo test`
    #[test]
    fn test_day() {
        assert!(verify_day(true));
    }
}
