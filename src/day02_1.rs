use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reports: Vec<Vec<isize>> = INPUT
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let mut safe_count = 0;
    for report in &reports {
        if let Err(i) = is_report_safe(report) {
            let start = if i == 0 { 0 } else { i - 1 };
            for i in start..=i + 1 {
                if is_report_safe_skip(report, i) {
                    safe_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
            continue;
        }
    }

    safe_count
}

#[inline]
fn is_report_safe(report: &[isize]) -> Result<(), usize> {
    let mut diffs = report
        .iter()
        .copied()
        .map_windows::<_, _, 2>(|&[l, r]| r - l)
        .peekable();
    let diff_positive = *diffs.peek().unwrap() > 0;
    for (i, diff) in diffs.enumerate() {
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

#[inline]
fn is_report_safe_skip(report: &[isize], skip: usize) -> bool {
    let diffs = report
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, n)| (i != skip).then_some(n))
        .map_windows::<_, _, 2>(|&[l, r]| r - l);
    diffs.clone().map(isize::signum).all_equal()
        && diffs.map(isize::abs).all(|n| (1..=3).contains(&n))
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
