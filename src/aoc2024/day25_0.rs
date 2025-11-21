use itertools::Itertools;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(None)]
#[expected_long(Some(3_663))]
pub struct Day;

const KEY_LENGTH: usize = 5;
const KEY_HEIGHT: u8 = 5;
type Identifier = [u8; KEY_LENGTH];

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> usize {
    let lines: Vec<_> = input.trim_ascii().split(|b| *b == b'\n').collect();
    let grids = lines.split(|l| l.is_empty());

    // Super rough estimate for size
    let mut locks = Vec::with_capacity(lines.len() / KEY_HEIGHT as usize);
    let mut keys = Vec::with_capacity(lines.len() / KEY_HEIGHT as usize);
    for grid in grids {
        if grid[0][0] == b'#' {
            locks.push(process_lock(grid));
        } else {
            keys.push(process_key(grid));
        }
    }

    locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            for i in 0..KEY_LENGTH {
                let k = key[i];
                let l = lock[i];
                if k + l > KEY_HEIGHT {
                    return false;
                }
            }
            true
        })
        .count()
}

/// Locks start from the top and extend down
#[inline]
#[allow(clippy::cast_possible_truncation)]
fn process_lock(grid: &[&[u8]]) -> Identifier {
    let mut lock = [0; KEY_LENGTH];

    for (col, val) in lock.iter_mut().enumerate() {
        let mut row = 0;
        *val = loop {
            if grid[row][col] == b'.' {
                break row as u8 - 1;
            }
            row += 1;
        }
    }
    lock
}

/// Keys start from the bottom and extend up
#[inline]
#[allow(clippy::cast_possible_truncation)]
fn process_key(grid: &[&[u8]]) -> Identifier {
    let mut key = [0; KEY_LENGTH];

    for (col, val) in key.iter_mut().enumerate() {
        let mut row = 1 + KEY_HEIGHT as usize;
        *val = loop {
            if grid[row][col] == b'.' {
                break KEY_HEIGHT - row as u8;
            }
            row -= 1;
        }
    }
    key
}
