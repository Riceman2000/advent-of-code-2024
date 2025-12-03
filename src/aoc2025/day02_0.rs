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
        .map(|ids| unsafe {
            let mid = ids.iter().position(|c| *c == b'-').unwrap_unchecked();
            (
                atoi::<usize>(&ids[..mid]).unwrap_unchecked(),
                atoi::<usize>(&ids[mid + 1..]).unwrap_unchecked(),
            )
        })
        .fold(0, |mut acc, (a, b)| {
            acc += (a..=b).filter(check_num).sum::<usize>();
            acc
        })
}

fn check_num(n: &usize) -> bool {
    let n = *n;
    let digs = {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    };
    if digs % 2 != 0 {
        return false;
    }
    let upper = n / 10_usize.pow(digs / 2);
    let lower = n % 10_usize.pow(digs / 2);
    if upper == lower {
        return true;
    }
    false
}
