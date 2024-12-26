use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let reports: Vec<Vec<i32>> = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut safe_count = 0;
    for report in &reports {
        if let Err(i) = is_report_safe(report) {
            let start = if i == 0 { 0 } else { i - 1 };
            for i in start..=i + 1 {
                if is_report_safe_skip(report, i) {
                    safe_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
            continue;
        }
    }

    safe_count
}

#[inline]
fn is_report_safe(report: &[i32]) -> Result<(), usize> {
    let mut diffs = report
        .iter()
        .copied()
        .map_windows::<_, _, 2>(|&[l, r]| r - l)
        .peekable();
    let diff_positive = *diffs.peek().unwrap() > 0;
    for (i, diff) in diffs.enumerate() {
        if (diff_positive && diff < 0)
            || (!diff_positive && diff > 0)
            || diff == 0
            || !(-3..=3).contains(&diff)
        {
            return Err(i);
        }
    }
    Ok(())
}

#[inline]
fn is_report_safe_skip(report: &[i32], skip: usize) -> bool {
    let diffs = report
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, n)| (i != skip).then_some(n))
        .map_windows::<_, _, 2>(|&[l, r]| r - l);

    diffs.clone().map(i32::signum).all_equal() && diffs.map(i32::abs).all(|n| (1..=3).contains(&n))
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 354;

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
