use std::collections::{HashMap, HashSet};

use atoi::atoi;

const INPUT: &[u8] = include_bytes!("../../input/2024/day22.txt");
aoc_macros::aoc_assert!(2_268);

const ITERATIONS: usize = 2000;

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let vendors: Vec<_> = INPUT
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|n| atoi::<u64>(n).unwrap())
        .collect();

    let mut diff_cache = HashSet::new();
    let mut window_map = HashMap::new();
    for mut num in vendors {
        diff_cache.clear();
        let mut w = [0; 5]; // Window to scan with
        for j in 0..ITERATIONS {
            num = pseudo_random(num);
            let price = (num % 10) as i8;

            w.rotate_left(1);
            w[4] = price;

            // Wait for enough data to be shifted in
            if j < 4 {
                continue;
            }

            let diffs = [w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]];

            // We can only detect a diff once per vendor
            if diff_cache.contains(&diffs) {
                continue;
            }
            diff_cache.insert(diffs);

            *window_map.entry(diffs).or_insert(0) += price as u64;
        }
    }

    let best_window = window_map.iter().max_by_key(|(_k, v)| **v).unwrap();

    *best_window.1
}

#[inline]
fn pseudo_random(mut num: u64) -> u64 {
    num ^= num * 64;
    num %= 16_777_216;
    num ^= num / 32;
    num %= 16_777_216;
    num ^= num * 2048;
    num %= 16_777_216;
    num
}
