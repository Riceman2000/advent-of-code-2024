use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("i32")]
#[expected_short(Some(3))]
#[expected_long(Some(1081))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> i32 {
    let mut out = 0;
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .fold(50, |mut acc, l| unsafe {
            let mut n: i32 = atoi(&l[1..]).unwrap_unchecked();
            if l[0] == b'L' {
                n *= -1;
            }
            acc += n;
            acc %= 100;
            if acc == 0 {
                out += 1;
            }
            acc
        });
    out
}
