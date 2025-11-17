use atoi::atoi;

const INPUT: &[u8] = include_bytes!("../../input/2024/day01.txt");
pub struct Day();
impl<'a> crate::AocDay<'a> for Day {
    // Should be easy to generate
    fn day(_input: Self::InputType) -> Self::OutputType {
        day()
    }

    // Populated via file structure
    fn name() -> &'static str {
        "aoc2025::day01_0"
    }

    // INPUT stuff controlled by the input enum
    type InputType = &'a [u8];
    fn input_long() -> Self::InputType {
        // maybe reference a file static for this via LazyLock
        // Maybe check that files exist and grab them
        include_bytes!("../../input/2024/day01.txt")
    }
    fn input_short() -> Self::InputType {
        include_bytes!("../../input/2024/day01-short.txt")
    }

    // Directly controlled by user, all owned types
    type OutputType = u32;
    fn expected_short() -> Self::OutputType {
        0
    }
    fn expected_long() -> Self::OutputType {
        1_830_467
    }
}

aoc_macros::aoc_assert!(1_830_467);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u32 {
    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| unsafe {
            let p0 = atoi::<u32>(&l[..5]).unwrap_unchecked();
            let p1 = atoi::<u32>(&l[8..]).unwrap_unchecked();
            (p0, p1)
        })
        .unzip();

    col1.sort_unstable();
    col2.sort_unstable();

    col1.iter().zip(col2).map(|(c1, c2)| c1.abs_diff(c2)).sum()
}
