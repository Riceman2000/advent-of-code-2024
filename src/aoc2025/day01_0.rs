// use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("u32")]
#[expected_short(Some(1))]
#[expected_long(Some(1))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u32 {
    // let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = input.trim_ascii().split(|c| *c == b'\n');
    let out = input.len() + 1;

    out as u32
}
