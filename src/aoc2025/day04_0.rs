#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(13))]
#[expected_long(Some(1445))]
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
    let grid: Vec<Vec<u8>> = input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(Vec::from)
        .collect();

    let mut sum = 0;
    for (y, l) in grid.iter().enumerate() {
        for (x, item) in l.iter().enumerate() {
            if *item == b'.' {
                continue;
            }
            let pos = (x.cast_signed(), y.cast_signed());
            let mut surround = 0;
            for dir in DIRS {
                let check_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if check_pos.0 < 0
                    || check_pos.0 >= l.len().cast_signed()
                    || check_pos.1 < 0
                    || check_pos.1 >= grid.len().cast_signed()
                {
                    continue;
                }
                let check_pos = (check_pos.0.cast_unsigned(), check_pos.1.cast_unsigned());
                let check_item = grid[check_pos.1][check_pos.0];
                if check_item == b'@' {
                    surround += 1;
                }
                // println!(
                //     "{}, {pos:?}, {check_pos:?}, {}",
                //     *item as char, check_item as char
                // );
            }
            if surround < 4 {
                // println!("{pos:?} valid");
                sum += 1;
            }
        }
    }

    sum
}
