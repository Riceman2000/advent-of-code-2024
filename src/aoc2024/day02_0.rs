use itertools::Itertools;

const INPUT: &str = include_str!("../../input/2024/day02.txt");
aoc_assert::aoc_assert!(287);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reports = INPUT.lines().map(|l| {
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
            continue;
        }
    }

    safe_count
}
