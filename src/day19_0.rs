use lazy_static::lazy_static;
use regex::Regex;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day19.txt");

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

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 226;

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
