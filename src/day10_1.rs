// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day10.txt");

#[must_use]
pub fn day() -> usize {
    let lines: Vec<_> = INPUT.trim_ascii().split(|b| *b == b'\n').collect();

    let mut entry_points = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == b'0' {
                entry_points.push((x, y));
            }
        }
    }

    let mut sum = 0;
    for entry_point in entry_points {
        sum += branch_from_coords(entry_point, &lines);
    }
    sum
}

fn branch_from_coords(current_pos: (usize, usize), lines: &[&[u8]]) -> usize {
    let current_num = lines[current_pos.1][current_pos.0] - 48;
    let height = lines.len();
    let width = lines[0].len();

    let mut sum = 0;
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let new_pos = (
            current_pos.0.wrapping_add_signed(dir.0),
            current_pos.1.wrapping_add_signed(dir.1),
        );
        if !(0..width).contains(&new_pos.0) || !(0..height).contains(&new_pos.1) {
            continue;
        }

        let new_num = lines[new_pos.1][new_pos.0] - 48;
        if new_num != 1 + current_num {
            continue;
        }

        if new_num == 9 {
            sum += 1;
            continue;
        }

        sum += branch_from_coords(new_pos, lines);
    }
    sum
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1324;

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        let sign = if actual > expected { '+' } else { '-' };
        eprintln!(
            "Got {actual} expected {expected}, diff {sign}{}",
            expected.abs_diff(actual)
        );
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that is automatically run by `cargo test`
    #[test]
    fn test_day() {
        assert!(verify_day(true));
    }
}
