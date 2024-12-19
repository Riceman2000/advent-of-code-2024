use rayon::prelude::*;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day19.txt");

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
                let mut dp = vec![0; pat.len() + 1];
                dp[0] = 1;

                for i in 0..pat.len() {
                    for &ss in &substrings {
                        // If nothing fit last time including single letters
                        if dp[i] == 0 {
                            continue;
                        }

                        // If the substring length would not fit
                        if i + ss.len() > pat.len() {
                            continue;
                        }

                        // If the substring does not match
                        if pat[i..i + ss.len()] != *ss {
                            continue;
                        }

                        // Accumulate matches
                        dp[i + ss.len()] += dp[i];
                    }
                }

                acc + dp[pat.len()]
            },
        )
        .sum()
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
