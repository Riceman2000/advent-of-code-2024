use atoi::atoi;
use pathfinding::directed::bfs;

#[derive(aoc_macros::AocDay)]
#[output_type("usize")]
#[expected_short(None)]
#[expected_long(Some(370))]
pub struct Day;

const START_POS: (usize, usize) = (0, 0);
const MAP_MAX: usize = 70;
const EXIT_POS: (usize, usize) = (MAP_MAX, MAP_MAX);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> usize {
    let mut points: Vec<_> = input
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
