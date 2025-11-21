#[derive(aoc_macros::AocDay)]
#[output_type("u64")]
#[expected_short(None)]
#[expected_long(Some(6_430_446_922_192))]
pub struct Day;

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_lossless)]
pub fn day(input: &[u8]) -> u64 {
    let mut disk = vec![None; 100_000];
    let mut cursor = 0;
    for (id, entry) in input.trim_ascii().chunks(2).enumerate() {
        let used = entry[0] as usize - 48;

        // Last number is a file that has no free space
        let free = if entry.len() == 2 {
            entry[1] as usize - 48
        } else {
            0
        };

        disk[cursor..cursor + used].fill(Some(id as u16));
        cursor += used + free; // Free stays None
    }

    let mut l_search = 0;
    let mut r_search = disk.len() - 1;
    loop {
        // Search for empty space from left
        let mut l_pos = l_search;
        loop {
            if disk[l_pos].is_none() {
                break;
            }
            l_pos += 1;
        }

        // Search for a used space from right
        let mut r_pos = r_search;
        loop {
            if disk[r_pos].is_some() {
                break;
            }
            r_pos -= 1;
        }
        if l_pos > r_pos {
            break;
        }
        disk.swap(l_pos, r_pos);

        l_search = l_pos + 1;
        r_search = r_pos - 1;
    }

    disk.iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (pos, id)| acc + (pos as u64) * (*id as u64))
}
