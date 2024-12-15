use atoi::atoi;
use itertools::Itertools;
use nalgebra::Vector2;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day14.txt");

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> i32 {
    let robots: Vec<_> = INPUT
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|line| {
            // Reference points for parsing
            let comma1 = line.iter().position(|c| *c == b',').unwrap();
            let space = line.iter().skip(comma1).position(|c| *c == b' ').unwrap() + comma1;
            let comma2 = line.iter().skip(space).position(|c| *c == b',').unwrap() + space;

            let px = atoi::<i32>(&line[2..comma1]).unwrap();
            let py = atoi::<i32>(&line[comma1 + 1..space]).unwrap();
            let position = Vector2::new(px, py);

            let vx = atoi::<i32>(&line[space + 3..comma2]).unwrap();
            let vy = atoi::<i32>(&line[comma2 + 1..]).unwrap();
            let velocity = Vector2::new(vx, vy);

            (position, velocity)
        })
        .collect();

    let mut positions = vec![Vector2::new(0, 0); robots.len()];
    for seconds in 0..100_000 {
        for (i, robot) in robots.iter().enumerate() {
            let offset = robot.1 * seconds;
            let mut new_pos = robot.0 + offset;

            // Clamp to max bounds
            new_pos[0] = new_pos[0].rem_euclid(WIDTH);
            new_pos[1] = new_pos[1].rem_euclid(HEIGHT);

            positions[i] = new_pos;
        }
        // return seconds;

        // When the image is showing, there are no bots overlapping
        if positions.iter().all_unique() {
            return seconds;
        }
    }
    0
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 7_371;

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
