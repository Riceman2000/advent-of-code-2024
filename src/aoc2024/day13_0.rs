use atoi::atoi;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

#[derive(aoc_macros::AocDay)]
#[output_type("u64")]
#[expected_short(None)]
#[expected_long(Some(35_574))]
pub struct Day;

// Real epsilon is too small
const BIG_EPSILON: f64 = 1e-3;

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn day(input: &[u8]) -> u64 {
    let games: Vec<_> = input
        .trim_ascii()
        .split(|b| *b == b'\n')
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let button_a = chunk.next().unwrap();
            let ax = atoi::<u64>(&button_a[12..=13]).unwrap();
            let ay = atoi::<u64>(&button_a[18..]).unwrap();

            let button_b = chunk.next().unwrap();
            let bx = atoi::<u64>(&button_b[12..=13]).unwrap();
            let by = atoi::<u64>(&button_b[18..]).unwrap();

            let prize = chunk.next().unwrap();
            let x_pos = prize.iter().position(|c| *c == b'X').unwrap();
            let y_pos = prize.iter().position(|c| *c == b'Y').unwrap();
            let px = atoi::<u64>(&prize[x_pos + 2..y_pos - 2]).unwrap();
            let py = atoi::<u64>(&prize[y_pos + 2..]).unwrap();

            Game {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            }
        })
        .collect();

    let mut sum = 0.0;
    for game in games {
        let a = Matrix2::new(
            game.ax as f64,
            game.bx as f64,
            game.ay as f64,
            game.by as f64,
        );
        let b = Vector2::new(game.px as f64, game.py as f64);
        let ai = a.qr();
        let x = ai.solve(&b).unwrap();

        // Determine if this is a real solution or an approximation
        let a_diff = (x[0].round() - x[0]).abs();
        let b_diff = (x[1].round() - x[1]).abs();
        if a_diff > BIG_EPSILON || b_diff > BIG_EPSILON {
            continue;
        }

        let a_pressed = x[0].round();
        let b_pressed = x[1].round();

        sum += a_pressed * 3.0 + b_pressed;
    }

    sum as u64
}

#[derive(Debug)]
struct Game {
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    px: u64,
    py: u64,
}
