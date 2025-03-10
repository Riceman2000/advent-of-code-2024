use atoi::atoi;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("../../input/2024/day07.txt");
aoc_macros::aoc_assert!(1_399_219_271_639);

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
                let num_ops = nums.len() - 1;

                for op_mask in 0..2u64.pow(nums.len() as u32 - 1) {
                    let mut sum = nums[0];
                    for op in 0..num_ops {
                        let mask = 1 << op;
                        let op_masked = (mask & op_mask) >> op;
                        if op_masked == 1 {
                            sum += nums[op + 1];
                        } else {
                            sum *= nums[op + 1];
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
