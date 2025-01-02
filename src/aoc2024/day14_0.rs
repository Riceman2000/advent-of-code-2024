use atoi::atoi;
use nalgebra::Vector2;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day14.txt");

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const SECONDS: i32 = 100;

const VERTICAL: i32 = WIDTH / 2;
const HORIZONTAL: i32 = HEIGHT / 2;

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> u64 {
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

    let mut ul = 0;
    let mut ur = 0;
    let mut lr = 0;
    let mut ll = 0;
    for robot in robots {
        let offset = robot.1 * SECONDS;
        let mut new_pos = robot.0 + offset;

        // Clamp to max bounds
        new_pos[0] %= WIDTH;
        new_pos[1] %= HEIGHT;

        // Negative overflows back into the positive
        if new_pos[0] < 0 {
            new_pos[0] += WIDTH;
        }
        if new_pos[1] < 0 {
            new_pos[1] += HEIGHT;
        }

        // Determine quadrant
        if new_pos[0] < VERTICAL && new_pos[1] < HORIZONTAL {
            ul += 1;
        } else if new_pos[0] > VERTICAL && new_pos[1] < HORIZONTAL {
            ur += 1;
        } else if new_pos[0] > VERTICAL && new_pos[1] > HORIZONTAL {
            lr += 1;
        } else if new_pos[0] < VERTICAL && new_pos[1] > HORIZONTAL {
            ll += 1;
        }
    }

    ul * ur * lr * ll
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 225_552_000;

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
