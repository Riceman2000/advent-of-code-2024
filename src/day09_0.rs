// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day09.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn day() -> usize {
    let mut disk: Vec<_> = INPUT
        .trim_ascii()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, data)| {
            let used = data[0] as usize - 48;

            // Last number is a file that has no free space
            let free = if data.len() == 2 {
                data[1] as usize - 48
            } else {
                0
            };
            let mut file = vec![Some(id); used];
            file.extend(vec![None::<usize>; free]);
            file
        })
        .collect();

    let mut l_search = 0;
    let mut r_search = disk.len() - 1;
    loop {
        let l_pos = disk
            .iter()
            .skip(l_search)
            .position(Option::is_none)
            .unwrap()
            + l_search;
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
        .fold(0, |acc, (pos, id)| acc + pos * id)
}

#[cfg(test)]
mod tests {
    use super::day;

    #[test]
    fn test_day() {
        // Correct value can be put here once it is known
        assert_eq!(6_430_446_922_192, day());
    }
}
