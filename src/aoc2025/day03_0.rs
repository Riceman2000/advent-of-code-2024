#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(357))]
#[expected_long(Some(17493))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .fold(0, |mut acc, l| {
            let (i, &b1) = l[..l.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|(i, b)| (**b, usize::MAX - i))
                .unwrap();
            let (_i, &b2) = l[i + 1..]
                .iter()
                .enumerate()
                .max_by_key(|(i, b)| (**b, usize::MAX - i))
                .unwrap();
            acc += (b1 - 48) as usize * 10 + (b2 - 48) as usize;
            acc
        })
}
