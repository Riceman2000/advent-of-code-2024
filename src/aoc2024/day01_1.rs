use atoi::atoi;

const INPUT: &[u8] = include_bytes!("../../input/2024/day01.txt");
aoc_macros::aoc_assert!(26_674_158);

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> u32 {
    let (col1, col2): (Vec<u32>, Vec<u32>) = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| unsafe {
            let p0 = atoi::<u32>(&l[..5]).unwrap_unchecked();
            let p1 = atoi::<u32>(&l[8..]).unwrap_unchecked();
            (p0, p1)
        })
        .unzip();

    col1.iter()
        .map(|n1| {
            let occurrences = col2.iter().filter(|n2| n1 == *n2).count();
            occurrences as u32 * n1
        })
        .sum()
}
