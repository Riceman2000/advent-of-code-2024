use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(1_227_775_554))]
#[expected_long(Some(12_586_854_255))]
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
            acc += sum_invalids(a, b);
            acc
        })
}

#[inline]
fn sum_invalids(a: usize, b: usize) -> usize {
    let digs_low = count_digits(a);
    let digs_high = count_digits(b);

    // Iterate over all possible lengths of invalid numbers, no odds
    let mut sum = 0;
    for digs in (digs_low..=digs_high).filter(|d| d % 2 == 0) {
        let half_low = 10usize.pow((digs / 2) - 1);
        let half_high = 10 * half_low - 1;

        // Smallest invalid number "part" that is >= a and has length digs
        // If digs_low > digs then our min a power of 10
        let min = if digs > digs_low {
            half_low
        } else {
            let mut min = a / (10 * half_low);
            if min * (10 * half_low + 1) < a {
                min += 1;
            }
            min
        };

        // Biggest invalid number "part" that is <= b and has length digs
        // If digs_high < digs then our max is repeating 9s e.g 9, 99, 999, ...
        let max = if digs < digs_high {
            half_high
        } else {
            let mut max = b / (10 * half_low);
            if max * (10 * half_low + 1) > b {
                max -= 1;
            }
            max
        };

        sum += (min..=max).sum::<usize>() * (10 * half_low + 1);
    }

    sum
}

#[inline]
fn count_digits(mut a: usize) -> u32 {
    let mut count = 0;
    while a > 0 {
        a /= 10;
        count += 1;
    }
    count
}
