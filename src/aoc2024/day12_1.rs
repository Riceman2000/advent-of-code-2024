const INPUT: &[u8] = include_bytes!("../../input/2024/day12.txt");
aoc_macros::aoc_assert!(901_100);

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
const U: usize = 0;
const UR: usize = 1;
const R: usize = 2;
const LR: usize = 3;
const D: usize = 4;
const LL: usize = 5;
const L: usize = 6;
const UL: usize = 7;

#[must_use]
pub fn day() -> u32 {
    let mut lines: Vec<(u8, bool)> = INPUT.trim_ascii().iter().map(|c| (*c, false)).collect();
    let mut lines: Vec<&mut [(u8, bool)]> = lines.split_mut(|b| b.0 == b'\n').collect();

    let mut sum = 0;
    let mut y = 0;
    while y < lines.len() {
        let mut x = 0;
        while x < lines[0].len() {
            // If tile has been seen don't include it
            if lines[y][x].1 {
                x += 1;
                continue;
            }
            let (corners, area) = branch_from_coords((x, y), &mut lines);
            sum += corners * area;
            x += 1;
        }
        y += 1;
    }

    sum
}

fn branch_from_coords(current_pos: (usize, usize), lines: &mut [&mut [(u8, bool)]]) -> (u32, u32) {
    let region = lines[current_pos.1][current_pos.0].0;

    // This should always be false at the start of the function, if it isn't then this is bjorked
    lines[current_pos.1][current_pos.0] = (region, true);

    let mut matches = [false; 8]; // Follows the same directions as `DIRS`

    // Scan all adjcent tiles to see if they exist or match
    for (i, dir) in DIRS.iter().enumerate() {
        let new_pos = (
            current_pos.0.wrapping_add_signed(dir.0),
            current_pos.1.wrapping_add_signed(dir.1),
        );

        // Check if the next squre would be outside of the area
        if !(0..lines.len()).contains(&new_pos.1) || !(0..lines[0].len()).contains(&new_pos.0) {
            continue;
        }

        let new_tile = lines[new_pos.1][new_pos.0];
        let new_region = new_tile.0;
        if new_region != region {
            continue;
        }

        matches[i] = true;
    }

    // Count the corners
    let mut total_corners = 0; // Will be 0 to 4
    if (matches[U] && !matches[UR] && matches[R]) || (!matches[U] && !matches[R]) {
        total_corners += 1;
    }
    if (matches[R] && !matches[LR] && matches[D]) || (!matches[R] && !matches[D]) {
        total_corners += 1;
    }
    if (matches[D] && !matches[LL] && matches[L]) || (!matches[D] && !matches[L]) {
        total_corners += 1;
    }
    if (matches[L] && !matches[UL] && matches[U]) || (!matches[L] && !matches[U]) {
        total_corners += 1;
    }

    // Scan again and recurse in four directions
    let mut total_area = 1; // Count the current tile
    for (dir, matches) in matches.iter().enumerate() {
        if !matches || dir % 2 == 1 {
            continue;
        }
        let dir = DIRS[dir];
        // If it matches it will be in bounds
        let new_pos = (
            current_pos.0.wrapping_add_signed(dir.0),
            current_pos.1.wrapping_add_signed(dir.1),
        );

        let new_tile = lines[new_pos.1][new_pos.0];
        // No need to explore visited tiles
        if new_tile.1 {
            continue;
        }

        let (new_corners, new_area) = branch_from_coords(new_pos, lines);
        total_corners += new_corners;
        total_area += new_area;
    }

    (total_corners, total_area)
}
