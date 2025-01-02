use rayon::prelude::*;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day06.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut start_pos = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == b'^') {
            start_pos = (x, y);
            break;
        }
    }

    let mut current_dir = Direction::new();
    let mut current_pos = start_pos;

    let mut visited = vec![current_pos];

    'main: loop {
        current_pos = loop {
            let next_pos = (
                current_pos.0.wrapping_add_signed(current_dir.0),
                current_pos.1.wrapping_add_signed(current_dir.1),
            );
            if next_pos.0 >= lines[0].len() || next_pos.1 >= lines.len() {
                break 'main;
            }
            if lines[next_pos.1][next_pos.0] == b'#' {
                current_dir.next();
            } else {
                break next_pos;
            }
        };

        visited.push(current_pos);
    }

    visited.sort_unstable();
    visited.dedup();

    // Parrallel iterator
    visited
        .par_iter()
        .fold(
            || 0,
            |a, c| {
                if gets_stuck(*c, &lines, start_pos) {
                    a + 1
                } else {
                    a
                }
            },
        )
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Direction(isize, isize);

impl Direction {
    fn new() -> Self {
        Self(0, -1)
    }
    fn next(&mut self) {
        let new_dir = match (self.0, self.1) {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!(),
        };
        *self = Self(new_dir.0, new_dir.1);
    }
}

fn gets_stuck(obstacle_coords: (usize, usize), lines: &[&[u8]], start_pos: (usize, usize)) -> bool {
    let mut current_dir = Direction::new();
    let mut current_pos = start_pos;

    let mut step_count = 0;
    'main: loop {
        step_count += 1;
        if step_count >= 6_000 {
            return true;
        }
        current_pos = loop {
            let next_pos = (
                current_pos.0.wrapping_add_signed(current_dir.0),
                current_pos.1.wrapping_add_signed(current_dir.1),
            );
            if next_pos.0 >= lines[0].len() || next_pos.1 >= lines.len() {
                break 'main;
            }
            if lines[next_pos.1][next_pos.0] == b'#'
                || (next_pos.1 == obstacle_coords.1 && next_pos.0 == obstacle_coords.0)
            {
                current_dir.next();
            } else {
                break next_pos;
            }
        };
    }
    false
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_753;

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
