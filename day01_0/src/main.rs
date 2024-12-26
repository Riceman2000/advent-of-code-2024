fn day() -> u32 {
    // Pull this file's contents into the binary as a string literal
    let input = include_str!("../input.txt");

    return input
        .lines()
        .map(|l| {
            let line_vec: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            line_vec[0] * 10 + line_vec[line_vec.len() - 1]
        })
        .sum();
}

pub fn main() {
    // This format is important because the benchmark strips out all lines starting with "o: "
    // Do not print anything else out from within the `day` function
    println!("o: {}", day());
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
