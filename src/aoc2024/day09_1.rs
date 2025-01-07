const INPUT: &[u8] = include_bytes!("../../input/2024/day09.txt");
aoc_macros::aoc_assert!(6_460_170_593_016);

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn day() -> usize {
    let mut disk: Vec<_> = INPUT
        .trim_ascii()
        .chunks(2)
        .enumerate()
        .map(|(id, entry)| {
            let used = entry[0] as usize - 48;

            // Last number is a file that has no free space
            let free = if entry.len() == 2 {
                entry[1] as usize - 48
            } else {
                0
            };

            Block { id, used, free }
        })
        .collect();

    let mut r_pos = disk.len() - 1;
    loop {
        // Grapical print of disk
        // let disk_graphical = disk.iter().fold(String::new(), |acc, b| {
        //     let used = format!("{}", char::from(b.id as u8 + 48)).repeat(b.used);
        //     let free = ".".repeat(b.free);
        //     acc + &used + &free
        // });
        // println!("Disk {}: {disk_graphical:?}", disk.len());

        // Get right block to be moved
        let r_block = disk[r_pos];

        // Search for empty space from left
        let mut l_pos = 0;
        loop {
            if disk[l_pos].free >= r_block.used {
                break;
            }

            // Unlikely to happen
            if l_pos >= disk.len() - 1 {
                println!("l_pos out of bounds");
                break;
            }
            l_pos += 1;
        }

        if l_pos >= r_pos {
            if r_pos == 0 {
                break;
            }
            r_pos -= 1;
            continue;
        }

        disk[r_pos - 1].free += r_block.used + r_block.free;
        disk.remove(r_pos);
        disk.insert(
            l_pos + 1,
            Block {
                free: disk[l_pos].free - r_block.used,
                ..r_block
            },
        );
        disk[l_pos].free = 0;
    }

    let mut pos = 0;
    let mut sum = 0;
    for block in disk {
        sum += std::iter::repeat(block.id)
            .take(block.used)
            .zip(pos..)
            .fold(0, |acc, (id, pos)| acc + id * pos);
        pos += block.used + block.free;
    }
    sum
}

#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    used: usize,
    free: usize,
}
