use itertools::Itertools;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(None)]
#[expected_long(Some(1235))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn day(input: &[u8]) -> usize {
    let lines: Vec<&[u8]> = input.trim_ascii().split(|c| *c == b'\n').collect();

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
