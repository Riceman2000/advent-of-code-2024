// Pull this file's contents into the binary as a string literal
const INPUT: &str = include_str!("../input/day01.txt");

#[must_use]
pub fn day() -> u32 {
    return INPUT
        .lines()
        .map(|l| {
            let line_vec: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            line_vec[0] * 10 + line_vec[line_vec.len() - 1]
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(54708, day());
    }
}
