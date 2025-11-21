use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("u64")]
#[expected_short(None)]
#[expected_long(Some(20_441_185_092))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> u64 {
    let nums = input
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|n| atoi::<u64>(n).unwrap());

    let mut sum = 0;
    for mut num in nums {
        for _ in 0..2000 {
            num = psudo_random(num);
        }
        sum += num;
        // println!("num: {num}");
    }
    sum
}

#[inline]
fn psudo_random(mut num: u64) -> u64 {
    num ^= num * 64;
    num %= 16_777_216;
    num ^= num / 32;
    num %= 16_777_216;
    num ^= num * 2048;
    num %= 16_777_216;
    num
}
