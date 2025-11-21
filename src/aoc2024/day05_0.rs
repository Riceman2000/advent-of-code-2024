use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("u32")]
#[expected_short(None)]
#[expected_long(Some(5_268))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u32 {
    let lines: Vec<_> = input.trim_ascii_end().split(|c| *c == b'\n').collect();
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
    for report in reports {
        let is_good = report
            .array_windows()
            .all(|&[a, b]| orders[a as usize].contains(&b));

        if is_good {
            sum += u32::from(report[report.len() / 2]);
        }
    }

    sum
}
