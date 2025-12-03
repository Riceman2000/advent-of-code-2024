use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(4_174_379_265))]
#[expected_long(Some(17_298_174_201))]
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

    // Iterate over all possible lenghts of invalid numbers, no odds
    let mut sum = 0;
    for digs in digs_low..=digs_high {
        // For each "width" of repeating values e.g. 3 for 123123123 we save our sum for dedupe later
        let mut width_counts = vec![0; digs as usize / 2 + 1];

        // Iterate over all possible widths of repeating parts, only even multiples
        for width in (1..=digs / 2).filter(|w| digs % w == 0) {
            // Number of reps for a given width
            let rep = digs / width;

            let rep_low = 10usize.pow((digs / rep) - 1);
            let rep_high = 10 * rep_low - 1;

            // Smallest invalid number "part" that is >= a, has length digs, and repeats properly
            // If digs_low > digs then our min a power of 10
            let min = if digs > digs_low {
                rep_low
            } else {
                let mut min = a / (rep_low * 10).pow(rep - 1);
                // Check this value and inc if it is off by one
                let mut tmp = min;
                for _ in 0..(rep - 1) {
                    tmp = tmp * (10 * rep_low) + min;
                }
                if tmp < a {
                    min += 1;
                }
                min
            };

            // Biggest invalid number "part" that is <= b, has length digs and repeats properly
            let max = if digs < digs_high {
                rep_high
            } else {
                let mut max = b / (rep_low * 10).pow(rep - 1);
                // Check this value and dec if it is off by one
                let mut tmp = max;
                for _ in 0..(rep - 1) {
                    tmp = tmp * (10 * rep_low) + max;
                }
                if tmp > b {
                    max -= 1;
                }
                max
            };

            // Construct invalid number from its parts
            let part = (min..=max).sum::<usize>();
            let mut invalid = part;
            for _ in 0..(rep - 1) {
                invalid = invalid * (10 * rep_low) + part;
            }
            width_counts[width as usize] = invalid;
        }

        // Dedupe
        let mut dig_sum = 0;
        for l in (1..=digs / 2).filter(|l| digs % l == 0) {
            dig_sum += width_counts[l as usize];
            for i in (1..l).filter(|i| l % i == 0) {
                dig_sum -= width_counts[i as usize];
            }
        }

        sum += dig_sum;
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
