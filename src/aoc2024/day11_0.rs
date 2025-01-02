use atoi::atoi;
use lazy_static::lazy_static;

const INPUT: &[u8] = include_bytes!("../../input/2024/day11.txt");
aoc_assert::aoc_assert!(185_205);

const NUM_BLINKS: usize = 25;

const MAX_CACHE: usize = 1_000;
const CACHE_DEPTH: usize = NUM_BLINKS + 1;

// While this could be in a normal const block, I don't want the very slow compile time to affect
// other days development
// This array is boxed to avoid running off the stack
lazy_static! {
    static ref LUT: Box<[[u64; MAX_CACHE]; CACHE_DEPTH]> = generate_lut();
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let nums: Vec<_> = INPUT
        .trim_ascii()
        .split(|b| *b == b' ')
        .map(|n| atoi::<usize>(n).unwrap())
        .collect();

    nums.iter()
        .map(|n| solve_from_lut(NUM_BLINKS, *n, &LUT))
        .sum()
}

fn generate_lut() -> Box<[[u64; MAX_CACHE]; CACHE_DEPTH]> {
    let mut lut = Box::new([[0; MAX_CACHE]; CACHE_DEPTH]);

    // Zero blinks always yeilds 1 stone
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
                    let multiplier = 10usize.pow((j.ilog10() + 1) / 2);
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

fn solve_from_lut(i: usize, j: usize, lut: &[[u64; MAX_CACHE]; CACHE_DEPTH]) -> u64 {
    if i == 0 {
        return 1;
    }

    match j {
        0 => lut[i - 1][1],
        _ if j < MAX_CACHE => lut[i][j],
        _ if j.ilog10() % 2 == 1 => {
            let multiplier = 10usize.pow((j.ilog10() + 1) / 2);
            solve_from_lut(i - 1, j / multiplier, lut) + solve_from_lut(i - 1, j % multiplier, lut)
        }
        _ => solve_from_lut(i - 1, j * 2024, lut),
    }
}
