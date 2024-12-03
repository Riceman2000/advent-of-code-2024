// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reports: Vec<Vec<i32>> = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut unsafe_count = 0;
    for report in &reports {
        let diffs: Vec<i32> = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        // println!("report: {report:?} \n\t{diffs:?}");
        let diff_positive = diffs[0] > 0;
        for diff in diffs {
            if (diff_positive && diff < 0)
                || (!diff_positive && diff > 0)
                || diff == 0
                || !(-3..=3).contains(&diff)
            {
                unsafe_count += 1;
                break;
            }
        }
    }

    reports.len() - unsafe_count
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
