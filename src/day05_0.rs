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

    let mut sum: u32 = 0;
    for report in reports {
        let is_good = report
            .array_windows()
            .all(|&[a, b]| orders.contains(&(a, b)));

        if is_good {
            sum += u32::from(report[report.len() / 2]);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(5_268, day());
    }
}
