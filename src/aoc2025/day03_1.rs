#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(3_121_910_778_619))]
#[expected_long(Some(173_685_428_989_126))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .fold(0, |mut acc, l| {
            let mut max = 0;
            let mut pow = 10usize.pow(11);
            let mut start = 0;
            for remaining in (0..12).rev() {
                let end = l.len() - remaining;
                let (i, &b) = l[start..end]
                    .iter()
                    .enumerate()
                    .max_by_key(|(i, b)| (**b, usize::MAX - i))
                    .unwrap();
                start += i + 1;

                max += (b - 48) as usize * pow;
                pow /= 10;
            }
            acc += max;
            acc
        })
}
