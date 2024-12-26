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

    let mut safe_count = 0u32;
    for report in reports {
        let mut idx = 0;
        let diff_positive = report[1] - report[0] > 0;
        let safe = loop {
            idx = if idx >= report.len() - 1 {
                break true;
            } else {
                idx + 1
            };

            let diff = report[idx] - report[idx - 1];
            if (diff_positive && diff < 0) || (!diff_positive && diff > 0) {
                break false;
            }
            if diff == 0 || !(-3..=3).contains(&diff) {
                break false;
            }
        };
        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(287, day());
    }
}
