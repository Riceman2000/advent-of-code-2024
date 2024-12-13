// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day12.txt");

#[must_use]
pub fn day() -> u32 {
    let mut lines: Vec<(u8, bool)> = INPUT.trim_ascii().iter().map(|c| (*c, false)).collect();
    let mut lines: Vec<&mut [(u8, bool)]> = lines.split_mut(|b| b.0 == b'\n').collect();

    let mut sum = 0;
    let mut y = 0;
    while y < lines.len() {
        let mut x = 0;
        while x < lines[0].len() {
            if lines[y][x].1 {
                x += 1;
                continue;
            }
            let (perim, area) = branch_from_coords((x, y), &mut lines);
            sum += perim * area;
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

    let mut total_area = 1; // Count the current tile
    let mut total_perim = 0;
    let mut current_perim = 0; // Will be 0 to 4
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let new_pos = (
            current_pos.0.wrapping_add_signed(dir.0),
            current_pos.1.wrapping_add_signed(dir.1),
        );

        // Check if the next squre would be outside of the area
        if !(0..lines.len()).contains(&new_pos.1) || !(0..lines[0].len()).contains(&new_pos.0) {
            current_perim += 1; // Count the void
            continue;
        }

        let new_tile = lines[new_pos.1][new_pos.0];
        let new_region = new_tile.0;
        if new_region != region {
            current_perim += 1;
            continue;
        }

        // No need to explore visited tiles
        if new_tile.1 {
            continue;
        }

        let (new_perim, new_area) = branch_from_coords(new_pos, lines);
        total_perim += new_perim;
        total_area += new_area;
    }
    (total_perim + current_perim, total_area)
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_473_276;

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
