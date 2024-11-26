use lazy_static::lazy_static;
use regex::Regex;

fn day() -> u32 {
    // Pull this file's contents into the binary as a string literal
    let input = include_str!("../input.txt");

    let input_vec: Vec<&str> = input.lines().collect();
    1
}

// Constructing regex searches is EXPENSIVE so it is very worth it to make this static
lazy_static! {
    static ref RE_SYMBOL_BEFORE: Regex = Regex::new(r"[\D--[.]](\d{1,})").unwrap();
    static ref RE_SYMBOL_AFTER: Regex = Regex::new(r"(\d{1,})[\D--[.]]").unwrap();
}

fn part_nums(line: &str) -> u32 {
    1
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
        assert_eq!(65371, day());
    }
}
