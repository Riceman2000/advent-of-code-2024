// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day01.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|l| {
            let p0 = l[..5].parse::<u32>().unwrap();
            let p1 = l[8..].parse::<u32>().unwrap();
            (p0, p1)
        })
        .unzip();

    col1.sort_unstable();
    col2.sort_unstable();

    col1.iter().zip(col2).map(|(c1, c2)| c1.abs_diff(c2)).sum()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_830_467;

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
