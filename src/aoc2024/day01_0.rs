use atoi::atoi;

const INPUT: &[u8] = include_bytes!("../../input/2024/day01.txt");

aoc_assert::aoc_assert!(1_830_467);

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
