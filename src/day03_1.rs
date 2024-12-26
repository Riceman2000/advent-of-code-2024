use lazy_static::lazy_static;
use regex::Regex;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day03.txt");

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for cap in RE.captures_iter(INPUT) {
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

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 78_683_433;

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
