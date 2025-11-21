use itertools::Itertools;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(None)]
#[expected_long(Some(287))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> usize {
    let input = unsafe { str::from_utf8_unchecked(input) };
    let reports = input.lines().map(|l| {
        l.split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let mut safe_count = 0;
    for report in reports {
        let diffs = report
            .iter()
            .copied()
            .map_windows::<_, _, 2>(|&[l, r]| r - l);
        if diffs.clone().map(i32::signum).all_equal()
            && diffs.map(i32::abs).all(|n| (1..=3).contains(&n))
        {
            safe_count += 1;
        }
    }

    safe_count
}
