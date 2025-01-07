use atoi::atoi;

const INPUT: &[u8] = include_bytes!("../../input/2024/day22.txt");
aoc_macros::aoc_assert!(20_441_185_092);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let nums = INPUT
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
