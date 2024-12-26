// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day01.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|l| {
            let p0 = l[..5].parse::<u32>().unwrap();
            let p1 = l[8..].parse::<u32>().unwrap();
            (p0, p1)
        })
        .unzip();

    col1.sort_unstable();
    col2.sort_unstable();

    col1.iter().zip(col2).map(|(c1, c2)| c1.abs_diff(c2)).sum()
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(1_830_467, day());
    }
}
