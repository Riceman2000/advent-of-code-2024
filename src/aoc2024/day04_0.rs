const INPUT: &[u8] = include_bytes!("../../input/2024/day04.txt");
aoc_assert::aoc_assert!(2_390);

#[must_use]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn day() -> u32 {
    let lines: Vec<&[u8]> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let mut seen = 0;

    let offset_vectors = [
        [0i32, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
        [-1, -1],
        [-1, 0],
        [-1, 1],
    ];

    let expected = b"XMAS";

    for y in 0..lines.len() as i32 {
        for x in 0..lines[0].len() as i32 {
            'offset: for [ox, oy] in offset_vectors {
                for (expected_char, off_mag) in expected.iter().zip(0..) {
                    let x_pos = (x + ox * off_mag) as usize;
                    let y_pos = (y + oy * off_mag) as usize;
                    // Relying on short circuit
                    if y_pos >= lines.len() || x_pos >= lines[0].len() {
                        continue 'offset;
                    }
                    let test_char = lines[y_pos][x_pos];
                    if test_char != *expected_char {
                        continue 'offset;
                    }
                }
                seen += 1;
            }
        }
    }

    seen
}
