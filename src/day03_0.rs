use regex::Regex;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day03.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for cap in reg.captures_iter(INPUT) {
        let l: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let r: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        sum += l * r;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(170_068_701, day());
    }
}