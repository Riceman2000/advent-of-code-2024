use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("i32")]
#[expected_short(Some(6))]
#[expected_long(Some(6689))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> i32 {
    let steps: Vec<i32> = input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| unsafe {
            let mut n = atoi(&l[1..]).unwrap_unchecked();
            if l[0] == b'L' {
                n *= -1;
            }
            n
        })
        .collect();

    let mut n = 50;
    let mut out = 0;
    for s in steps {
        if s < 0 {
            for _ in 0..-s {
                n -= 1;
                if n < 0 {
                    n = 99;
                }
                if n == 0 {
                    out += 1;
                }
            }
        } else {
            for _ in 0..s {
                n += 1;
                if n > 99 {
                    n = 0;
                }
                if n == 0 {
                    out += 1;
                }
            }
        }
    }

    out
}
