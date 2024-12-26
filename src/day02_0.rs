use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day02.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reports = INPUT.lines().map(|l| {
        l.split(' ')
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    });

    let mut safe_count = 0;
    for report in reports {
        let diffs = report
            .iter()
            .copied()
            .map_windows::<_, _, 2>(|&[l, r]| r - l);
        if diffs.clone().map(isize::signum).all_equal()
            && diffs.map(isize::abs).all(|n| (1..=3).contains(&n))
        {
            safe_count += 1;
            continue;
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
