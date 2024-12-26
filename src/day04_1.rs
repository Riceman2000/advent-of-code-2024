// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day04.txt");

#[must_use]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn day() -> u32 {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut seen = 0;

    // Edges will never have valid X
    for y in 1..lines.len() - 1 {
        for x in 1..lines[0].len() - 1 {
            let test_char = lines[y][x];
            if test_char != b'A' {
                continue;
            }

            // ur -> upper right, ll -> lower left, etc
            let ur = lines[y - 1][x + 1];
            let ll = lines[y + 1][x - 1];
            if !((ur == b'M' && ll == b'S') || (ur == b'S' && ll == b'M')) {
                continue;
            }

            let ul = lines[y - 1][x - 1];
            let lr = lines[y + 1][x + 1];
            if !((ul == b'M' && lr == b'S') || (ul == b'S' && lr == b'M')) {
                continue;
            }

            seen += 1;
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(1_809, day());
    }
}
