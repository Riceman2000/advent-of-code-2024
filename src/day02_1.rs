// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let mut reports: Vec<Vec<i32>> = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut safe_count = 0;
    for report in &mut reports {
        if let Err(idx) = is_report_safe(report) {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_report_safe(&new_report).is_ok() {
                    safe_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_report_safe(report: &[i32]) -> Result<(), usize> {
    let diffs: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    // println!("report: {report:?} \n\t{diffs:?}");
    let diff_positive = diffs[0] > 0;
    for (i, &diff) in diffs.iter().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(354, day());
    }
}
