use lazy_static::lazy_static;
use regex::Regex;

// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day03.txt");

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for cap in RE.captures_iter(INPUT) {
        match cap.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ if enabled => {
                let l: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let r: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
                sum += l * r;
            }
            _ => (),
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
        assert_eq!(78_683_433, day());
    }
}
