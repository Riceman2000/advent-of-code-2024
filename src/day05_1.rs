use itertools::Itertools;
use std::cmp::Ordering;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day05.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let lines: Vec<_> = INPUT.trim_ascii_end().lines().collect();
    let section_split = lines.iter().position(|l| l.is_empty()).unwrap();
    let orders: Vec<_> = lines[0..section_split]
        .iter()
        .map(|r| {
            (
                r[0..2].parse::<u8>().unwrap(),
                r[3..].parse::<u8>().unwrap(),
            )
        })
        .collect();
    let reports: Vec<_> = lines[section_split + 1..]
        .iter()
        .map(|r| {
            r.split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut bad_reports = Vec::new();
    'report: for (report_idx, report) in reports.iter().enumerate() {
        for (num_idx, num) in report.iter().enumerate() {
            for order_rule in orders.iter().filter(|o| o.0 == *num) {
                for test_num in report.iter().take(num_idx) {
                    if *test_num == order_rule.1 {
                        bad_reports.push(report_idx);
                        continue 'report;
                    }
                }
            }
        }
    }

    let mut sum: u32 = 0;
    for i in bad_reports {
        let sorted: Vec<_> = reports[i]
            .iter()
            .sorted_by(|a, b| {
                if orders.contains(&(**a, **b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .collect();
        sum += u32::from(*sorted[sorted.len() / 2]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(5_799, day());
    }
}
