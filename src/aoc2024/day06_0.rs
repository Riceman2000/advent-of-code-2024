// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day06.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut current_pos = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == b'^') {
            current_pos = (x, y);
            break;
        }
    }

    let mut direction_vectors = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let mut current_dir = direction_vectors.next().unwrap();

    let mut visited = vec![current_pos];
    'main: loop {
        let mut next_pos;
        loop {
            next_pos = (
                current_pos.0.wrapping_add_signed(current_dir.0),
                current_pos.1.wrapping_add_signed(current_dir.1),
            );
            if next_pos.0 >= lines[0].len() || next_pos.1 >= lines.len() {
                break 'main;
            }
            if lines[next_pos.1][next_pos.0] == b'#' {
                current_dir = direction_vectors.next().unwrap();
            } else {
                break;
            }
        }

        current_pos = next_pos;
        visited.push(current_pos);
    }

    visited.sort_unstable();
    visited.dedup();
    visited.len()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 5_239;

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
