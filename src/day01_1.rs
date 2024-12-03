// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day01.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> u32 {
    let (col1, col2): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|l| {
            let p0 = l[..5].parse::<u32>().unwrap();
            let p1 = l[8..].parse::<u32>().unwrap();
            (p0, p1)
        })
        .unzip();

    col1.iter()
        .map(|n1| {
            let occurrences = col2.iter().filter(|n2| n1 == *n2).count();
            occurrences as u32 * n1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(26_674_158, day());
    }
}
