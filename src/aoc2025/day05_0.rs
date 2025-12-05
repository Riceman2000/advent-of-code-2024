use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(Some(3))]
#[expected_long(Some(770))]
pub struct Day;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &'static [u8]) -> usize {
    let input: Vec<_> = input.split(|c| *c == b'\n').collect();

    let split = input.iter().position(|l| l.is_empty()).unwrap();
    let pairs: Vec<(usize, usize)> = input[..split]
        .iter()
        .map(|l| {
            let mut l = l.split(|c| *c == b'-');
            let a = atoi(l.next().unwrap()).unwrap();
            let b = atoi(l.next().unwrap()).unwrap();
            (a, b)
        })
        .collect();

    let mut count = 0;
    for l in &input[split + 1..] {
        let Some(n) = atoi(l) else { break };
        if pairs.iter().any(|(a, b)| (*a..=*b).contains(&n)) {
            count += 1;
        }
    }
    count
}
