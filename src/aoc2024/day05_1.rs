use atoi::atoi;
use std::cmp::Ordering;

const INPUT: &[u8] = include_bytes!("../../input/2024/day05.txt");
aoc_assert::aoc_assert!(5_799);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let lines: Vec<_> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let section_split = lines.iter().position(|l| l.is_empty()).unwrap();
    let mut orders = [const { Vec::new() }; 100]; // Indexes 0-99
    for order_line in &lines[0..section_split] {
        let index = atoi::<usize>(&order_line[0..2]).unwrap();
        let page = atoi::<u8>(&order_line[3..]).unwrap();
        orders[index].push(page);
    }
    let reports: Vec<_> = lines[section_split + 1..]
        .iter()
        .map(|r| {
            r.split(|c| *c == b',')
                .map(|n| unsafe { atoi::<u8>(n).unwrap_unchecked() })
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut sum: u32 = 0;
    for mut report in reports {
        if report
            .array_windows()
            .all(|&[a, b]| orders[a as usize].contains(&b))
        {
            continue;
        }

        report.sort_by(|a, b| {
            if orders[*a as usize].contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sum += u32::from(report[report.len() / 2]);
    }

    sum
}
