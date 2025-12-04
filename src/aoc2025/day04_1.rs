#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(43))]
#[expected_long(Some(8317))]
pub struct Day;

// Constants to help keep directions in order
const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let mut grid: Vec<Vec<u8>> = input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(Vec::from)
        .collect();

    let mut sum = 0;
    loop {
        let to_remove = get_removable(&grid);
        if to_remove.is_empty() {
            break;
        }
        sum += to_remove.len();
        for r in to_remove {
            grid[r.1][r.0] = b'.';
        }
    }
    sum
}

#[inline]
fn get_removable(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let valid_x = 0..grid[0].len();
    let valid_y = 0..grid.len();
    for (y, l) in grid.iter().enumerate() {
        for (x, item) in l.iter().enumerate() {
            if *item == b'.' {
                continue;
            }
            let pos = (x.cast_signed(), y.cast_signed());
            let mut surround = 0;
            for dir in DIRS {
                let check_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if !valid_x.contains(&check_pos.0.cast_unsigned())
                    || !valid_y.contains(&check_pos.1.cast_unsigned())
                {
                    continue;
                }
                let check_pos = (check_pos.0.cast_unsigned(), check_pos.1.cast_unsigned());
                let check_item =
                    unsafe { *grid.get_unchecked(check_pos.1).get_unchecked(check_pos.0) };
                if check_item == b'@' {
                    surround += 1;
                }
            }
            if surround < 4 {
                let pos = (pos.0.cast_unsigned(), pos.1.cast_unsigned());
                out.push(pos);
            }
        }
    }
    out
}
