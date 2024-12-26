// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day06.txt");

#[must_use]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn day() -> usize {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut current_pos = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == b'^') {
            current_pos = (x, y);
            break;
        }
    }

    let mut direction_vectors = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let mut current_dir = direction_vectors.next().unwrap();

    let mut visited = vec![current_pos];
    'main: loop {
        let mut next_pos;
        loop {
            next_pos = (
                current_pos.0.wrapping_add_signed(current_dir.0),
                current_pos.1.wrapping_add_signed(current_dir.1),
            );
            if next_pos.0 >= lines[0].len() || next_pos.1 >= lines.len() {
                break 'main;
            }
            if lines[next_pos.1][next_pos.0] == b'#' {
                current_dir = direction_vectors.next().unwrap();
            } else {
                break;
            }
        }

        current_pos = next_pos;
        visited.push(current_pos);
    }

    visited.sort_unstable();
    visited.dedup();

    let mut out = 0;
    for (x, y) in visited {
        if gets_stuck((x, y)) {
            out += 1;
        }
    }
    out
}

fn gets_stuck(obstacle_coords: (usize, usize)) -> bool {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut current_pos = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.iter().position(|c| *c == b'^') {
            current_pos = (x, y);
            break;
        }
    }

    let mut direction_vectors = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let mut current_dir = direction_vectors.next().unwrap();

    let mut step_count = 0;
    'main: loop {
        step_count += 1;
        if step_count >= 6_000 {
            return true;
        }
        let mut next_pos;
        loop {
            next_pos = (
                current_pos.0.wrapping_add_signed(current_dir.0),
                current_pos.1.wrapping_add_signed(current_dir.1),
            );
            if next_pos.0 >= lines[0].len() || next_pos.1 >= lines.len() {
                break 'main;
            }
            if lines[next_pos.1][next_pos.0] == b'#'
                || (next_pos.1 == obstacle_coords.1 && next_pos.0 == obstacle_coords.0)
            {
                current_dir = direction_vectors.next().unwrap();
            } else {
                break;
            }
        }

        current_pos = next_pos;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(1_753, day());
    }
}
