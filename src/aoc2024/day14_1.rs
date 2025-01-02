use atoi::atoi;
use itertools::Itertools;
use nalgebra::Vector2;

const INPUT: &[u8] = include_bytes!("../../input/2024/day14.txt");
aoc_assert::aoc_assert!(7_371);

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

const VERTICAL: i32 = WIDTH / 2;
const HORIZONTAL: i32 = HEIGHT / 2;

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_wrap)]
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

    // Isolate best X
    let best_x = (0..WIDTH)
        .map(|seconds| {
            robots
                .iter()
                .map(move |robot| {
                    let new_x = robot.0[0] + (robot.1[0] * seconds);
                    new_x.rem_euclid(WIDTH).abs_diff(VERTICAL)
                })
                .sum::<u32>()
        })
        .position_min()
        .unwrap() as i32;

    // Isolate best Y
    let best_y = (0..WIDTH)
        .map(|seconds| {
            robots
                .iter()
                .map(move |robot| {
                    let new_x = robot.0[1] + (robot.1[1] * seconds);
                    new_x.rem_euclid(HEIGHT).abs_diff(HORIZONTAL)
                })
                .sum::<u32>()
        })
        .position_min()
        .unwrap() as i32;

    // Chinese remainder theorem -> https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let mut sum = best_x * mod_inv(HEIGHT, WIDTH).unwrap() * HEIGHT;
    sum += best_y * mod_inv(WIDTH, HEIGHT).unwrap() * WIDTH;
    sum % (HEIGHT * WIDTH)
}

// Euclidian GCD
#[allow(clippy::many_single_char_names)]
fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Inverse mod
fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
