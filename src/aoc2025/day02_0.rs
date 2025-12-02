use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(1227775554))]
#[expected_long(Some(12586854255))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    input
        .trim_ascii()
        .split(|c| *c == b',')
        .map(|ids| {
            let mut ids = ids.split(|c| *c == b'-').map(|n| atoi::<usize>(n).unwrap());
            let a = ids.next().unwrap();
            let b = ids.next().unwrap();
            (a, b)
        })
        .fold(0, |mut acc, (a, b)| unsafe {
            acc += (a..=b).filter(check_num).sum::<usize>();

            acc
        })
}

fn check_num(n: &usize) -> bool {
    let n = *n;
    let digs = (n as f64).log10().floor() as usize + 1;
    if digs % 2 != 0 {
        return false;
    }
    let upper = n / 10_usize.pow(digs as u32 / 2);
    let lower = n - upper * 10_usize.pow(digs as u32 / 2);
    if upper == lower {
        return true;
    }
    false
}
