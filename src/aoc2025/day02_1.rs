use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(4174379265))]
#[expected_long(Some(17298174201))]
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
    if digs <= 1 {
        return false;
    }

    'mods: for m in 1..=(digs / 2) {
        // Only valid if digs is divisible by our chunk width
        if digs % m != 0 {
            continue;
        }
        let mut id = n;
        let m = 10_usize.pow(m as u32);
        let mut last_d = id % m;
        while id > 0 {
            let d = id % m;
            id /= m;
            if last_d != d {
                continue 'mods;
            }
            last_d = d;
        }
        return true;
    }
    false
}
