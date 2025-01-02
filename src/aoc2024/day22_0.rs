use atoi::atoi;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day22.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let nums = INPUT
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|n| atoi::<u64>(n).unwrap());

    let mut sum = 0;
    for mut num in nums {
        for _ in 0..2000 {
            num = psudo_random(num);
        }
        sum += num;
        // println!("num: {num}");
    }
    sum
}

#[inline]
fn psudo_random(mut num: u64) -> u64 {
    num ^= num * 64;
    num %= 16_777_216;
    num ^= num / 32;
    num %= 16_777_216;
    num ^= num * 2048;
    num %= 16_777_216;
    num
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 20_441_185_092;

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
