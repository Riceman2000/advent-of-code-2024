use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("i32")]
#[expected_short(Some(6))]
#[expected_long(Some(6689))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> i32 {
    let mut out = 0;
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .fold(50, |acc, l| unsafe {
            let mut n: i32 = atoi(&l[1..]).unwrap_unchecked();
            if l[0] == b'L' {
                n *= -1;
            }

            out += n.abs() / 100;
            if acc == 0 || acc + n % 100 < 0 || acc + n % 100 > 100 {
                out += 1;
            }

            (acc + n).rem_euclid(100)
        });
    out
}
