use atoi::atoi;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day11.txt");

const NUM_BLINKS: usize = 25;
#[must_use]
pub fn day() -> usize {
    let mut nums: Vec<_> = INPUT
        .trim_ascii()
        .split(|b| *b == b' ')
        .map(|n| atoi::<u64>(n).unwrap())
        .collect();

    for _ in 0..NUM_BLINKS {
        let mut i = 0;
        loop {
            let num = nums[i];
            match num {
                0 => nums[i] = 1,
                _ if (num.ilog10() + 1) % 2 == 0 => {
                    let num_digits = num.ilog10() + 1;
                    let l_digits = num / 10u64.pow(num_digits / 2);
                    let r_digits = num.wrapping_sub(l_digits * 10u64.pow(num_digits / 2));

                    nums[i] = r_digits;
                    nums.insert(i, l_digits);

                    i += 1;
                }
                _ => nums[i] *= 2024,
            }

            i += 1;
            if i >= nums.len() {
                break;
            }
        }
    }
    nums.len()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 185_205;

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
