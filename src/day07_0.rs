use atoi::atoi;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day07.txt");

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

    let mut out = 0;
    for line in lines {
        let target = line.0;
        let nums = line.1;
        let num_ops = nums.len() - 1;

        // println!("target: {target}, nums {nums:?}, opmask {op_mask:b}");

        for op_mask in 0..2u64.pow(nums.len() as u32 - 1) {
            let mut sum = nums[0];
            for op in 0..num_ops {
                let mask = 1 << op;
                let op_masked = (mask & op_mask) >> op;
                // println!(
                //     "target: {target}, nums {nums:?}, opmask {op_mask:b}, opmasked: {op_masked:b}"
                // );
                if op_masked == 1 {
                    // println!("{} + {}", sum, nums[op + 1]);
                    sum += nums[op + 1];
                } else {
                    // println!("{} * {}", sum, nums[op + 1]);
                    sum *= nums[op + 1];
                }
                // std::thread::sleep(std::time::Duration::from_secs(1));
            }

            if sum == target {
                out += sum;
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(1_830_467, day());
    }
}
