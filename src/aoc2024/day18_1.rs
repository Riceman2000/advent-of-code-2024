use atoi::atoi;
use pathfinding::directed::bfs;

#[derive(aoc_macros::AocDay)]
#[output_type("String")]
#[expected_short(None)]
#[expected_long(Some("(65, 6)".to_string()))]
pub struct Day;

const START_POS: (usize, usize) = (0, 0);
const MAP_MAX: usize = 70;
const SCAN_START: usize = 1024; // P1 means the first 1024 will never block

const EXIT_POS: (usize, usize) = (MAP_MAX, MAP_MAX);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day(input: &[u8]) -> String {
    let points: Vec<_> = input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .map(|l| {
            let comma_pos = l.iter().position(|b| *b == b',').unwrap();
            let x = atoi::<usize>(&l[..comma_pos]).unwrap();
            let y = atoi::<usize>(&l[comma_pos + 1..]).unwrap();
            (x, y)
        })
        .collect();

    let mut r_bound = points.len() - 1;
    let mut l_bound = SCAN_START;
    let mut pivot = usize::midpoint(l_bound, r_bound);
    while l_bound <= r_bound {
        let l_result = path_exists(&points[..pivot]);
        let r_result = path_exists(&points[..=pivot]);
        if l_result && !r_result {
            break;
        }
        if l_result && r_result {
            l_bound = pivot + 1;
        } else {
            r_bound = pivot - 1;
        }

        pivot = usize::midpoint(l_bound, r_bound);
    }
    format!("{:?}", points[pivot])
}

fn path_exists(points: &[(usize, usize)]) -> bool {
    // Expensive but worth it to be able to binary search
    let mut points = points.to_vec();
    points.sort_unstable();

    bfs::bfs(
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
    .is_some()
}
