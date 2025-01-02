use atoi::atoi;
use rayon::prelude::*;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day07.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> u64 {
    let lines: Vec<(u64, Vec<u64>)> = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| {
            let mut l = l.split(|c| *c == b':');

            let target = atoi::<u64>(l.next().unwrap()).unwrap();
            let nums: Vec<_> = l
                .next()
                .unwrap()
                .split(|c| *c == b' ')
                .skip(1)
                .map(|n| atoi::<u64>(n).unwrap())
                .collect();

            (target, nums)
        })
        .collect();

    lines
        .par_iter()
        .fold(
            || 0,
            |acc, line| {
                let target = line.0;
                let nums = &line.1;
                let num_ops = nums.len() as u64 - 1;

                for op_mask in 0..3u64.pow(nums.len() as u32 - 1) {
                    let mut sum = nums[0];
                    for op in 0..num_ops {
                        // Extract base 3 digit
                        let current_op = (op_mask / 3u64.pow(op as u32)) % 3;
                        match current_op {
                            0 => sum += nums[op as usize + 1],
                            1 => sum *= nums[op as usize + 1],
                            2 => {
                                let concat_num = nums[op as usize + 1];
                                let num_digits = concat_num.ilog10() + 1;
                                sum *= 10u64.pow(num_digits);
                                sum += concat_num;
                            }
                            _ => unreachable!(),
                        }

                        if sum > target {
                            break;
                        }
                    }

                    if sum == target {
                        return acc + sum;
                    }
                }
                acc
            },
        )
        .sum()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 275_791_737_999_003;

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
