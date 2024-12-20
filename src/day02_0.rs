use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reports = INPUT.lines().map(|l| {
        l.split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe_count = 0;
    for report in reports {
        let diffs = report
            .iter()
            .copied()
            .map_windows::<_, _, 2>(|&[l, r]| r - l);
        if diffs.clone().map(i32::signum).all_equal()
            && diffs.map(i32::abs).all(|n| (1..=3).contains(&n))
        {
            safe_count += 1;
            continue;
        }
    }

    safe_count
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 287;

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
