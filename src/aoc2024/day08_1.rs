use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day08.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn day() -> usize {
    let lines: Vec<&[u8]> = INPUT.trim_ascii().split(|c| *c == b'\n').collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antennas: Vec<(&u8, Vec<(i32, i32)>)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != b'.' {
                match antennas.binary_search_by_key(c, |a| *a.0) {
                    Ok(i) => antennas[i].1.push((x as i32, y as i32)),
                    Err(i) => {
                        antennas.insert(i, (c, vec![(x as i32, y as i32)]));
                    }
                }
            }
        }
    }

    let mut antinodes = Vec::new();
    for (_frequency, coords) in antennas {
        let pairs: Vec<_> = coords.iter().combinations(2).collect();
        for pair in &pairs {
            let p1 = *pair[0];
            let p2 = *pair[1];

            let distance_x = p1.0 - p2.0;
            let distance_y = p1.1 - p2.1;

            let mut anti1 = p1;
            while (0..width).contains(&anti1.0) && (0..height).contains(&anti1.1) {
                antinodes.push(anti1);
                anti1 = (anti1.0 + distance_x, anti1.1 + distance_y);
            }

            let mut anti2 = p2;
            while (0..width).contains(&anti2.0) && (0..height).contains(&anti2.1) {
                antinodes.push(anti2);
                anti2 = (anti2.0 - distance_x, anti2.1 - distance_y);
            }
        }
    }

    antinodes.sort_unstable();
    antinodes.dedup();
    antinodes.len()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1235;

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
