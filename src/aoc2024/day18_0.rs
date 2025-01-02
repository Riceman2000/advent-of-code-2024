use atoi::atoi;
use pathfinding::directed::bfs;

const INPUT: &[u8] = include_bytes!("../../input/2024/day18.txt");
aoc_assert::aoc_assert!(370);

const START_POS: (usize, usize) = (0, 0);
const MAP_MAX: usize = 70;
const EXIT_POS: (usize, usize) = (MAP_MAX, MAP_MAX);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let mut points: Vec<_> = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .take(1024)
        .map(|l| {
            let comma_pos = l.iter().position(|b| *b == b',').unwrap();
            let x = atoi::<usize>(&l[..comma_pos]).unwrap();
            let y = atoi::<usize>(&l[comma_pos + 1..]).unwrap();
            (x, y)
        })
        .collect();
    points.sort_unstable();

    let out = bfs::bfs(
        &(START_POS.0, START_POS.1),
        |&(x, y): &(usize, usize)| {
            [
                (x + 1, y),
                (x, y + 1),
                (x.saturating_sub(1), y),
                (x, y.saturating_sub(1)),
            ]
            .into_iter()
            .filter(|&(x, y)| {
                x <= MAP_MAX && y <= MAP_MAX && points.binary_search(&(x, y)).is_err()
            })
        },
        |&node| node == (EXIT_POS.0, EXIT_POS.1),
    )
    .unwrap();

    out.len() - 1
}
