use std::sync::LazyLock;

use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("u64")]
#[expected_short(None)]
#[expected_long(Some(185_205))]
pub struct Day;

const NUM_BLINKS: usize = 25;

const MAX_CACHE: usize = 1_000;
const CACHE_DEPTH: usize = NUM_BLINKS + 1;

// While this could be in a normal const block, I don't want the very slow compile time to affect
// other days development
// This array is a vec to avoid running off the stack
static LUT: LazyLock<Vec<[u64; MAX_CACHE]>> = LazyLock::new(generate_lut);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u64 {
    let nums: Vec<_> = input
        .trim_ascii()
        .split(|b| *b == b' ')
        .map(|n| atoi::<usize>(n).unwrap())
        .collect();

    nums.iter()
        .map(|n| solve_from_lut(NUM_BLINKS, *n, &LUT))
        .sum()
}

fn generate_lut() -> Vec<[u64; MAX_CACHE]> {
    let mut lut = vec![[0; MAX_CACHE]; CACHE_DEPTH];

    // Zero blinks always yields 1 stone
    lut[0].fill(1);

    // For each possible stone look back at the last solution for the previous number of blinks, if
    // that solution is not cached recurse until we find one that is (this is very likely to happen)
    let mut i = 1;
    while i < CACHE_DEPTH {
        let mut j = 0;
        while j < MAX_CACHE {
            lut[i][j] = match j {
                0 => lut[i - 1][1],
                _ if j.ilog10() % 2 == 1 => {
                    let multiplier = 10usize.pow(j.ilog10().div_ceil(2));
                    lut[i - 1][j / multiplier] + lut[i - 1][j % multiplier]
                }
                _ => solve_from_lut(i - 1, j * 2024, &lut),
            };
            j += 1;
        }
        i += 1;
    }

    lut
}

fn solve_from_lut(i: usize, j: usize, lut: &[[u64; MAX_CACHE]]) -> u64 {
    if i == 0 {
        return 1;
    }

    match j {
        0 => lut[i - 1][1],
        _ if j < MAX_CACHE => lut[i][j],
        _ if j.ilog10() % 2 == 1 => {
            let multiplier = 10usize.pow(j.ilog10().div_ceil(2));
            solve_from_lut(i - 1, j / multiplier, lut) + solve_from_lut(i - 1, j % multiplier, lut)
        }
        _ => solve_from_lut(i - 1, j * 2024, lut),
    }
}
