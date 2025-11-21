#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(None)]
#[expected_long(Some(1_324))]
pub struct Day;

#[must_use]
pub fn day(input: &[u8]) -> usize {
    let lines: Vec<_> = input.trim_ascii().split(|b| *b == b'\n').collect();

    let mut entry_points = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == b'0' {
                entry_points.push((x, y));
            }
        }
    }

    let mut sum = 0;
    for entry_point in entry_points {
        sum += branch_from_coords(entry_point, &lines);
    }
    sum
}

fn branch_from_coords(current_pos: (usize, usize), lines: &[&[u8]]) -> usize {
    let current_num = lines[current_pos.1][current_pos.0] - 48;
    let height = lines.len();
    let width = lines[0].len();

    let mut sum = 0;
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let new_pos = (
            current_pos.0.wrapping_add_signed(dir.0),
            current_pos.1.wrapping_add_signed(dir.1),
        );
        if !(0..width).contains(&new_pos.0) || !(0..height).contains(&new_pos.1) {
            continue;
        }

        let new_num = lines[new_pos.1][new_pos.0] - 48;
        if new_num != 1 + current_num {
            continue;
        }

        if new_num == 9 {
            sum += 1;
            continue;
        }

        sum += branch_from_coords(new_pos, lines);
    }
    sum
}
