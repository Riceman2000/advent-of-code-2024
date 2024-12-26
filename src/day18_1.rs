use atoi::atoi;
use pathfinding::directed::dijkstra;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day18.txt");

const START_POS: (usize, usize) = (0, 0);
const MAP_MAX: usize = 70;
const SCAN_START: usize = 1024; // P1 means the first 1024 will never block

const EXIT_POS: (usize, usize) = (MAP_MAX, MAP_MAX);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> String {
    let points: Vec<_> = INPUT
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
    let mut pivot = (r_bound + l_bound) / 2;
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

        pivot = (l_bound + r_bound) / 2;
    }
    format!("{:?}", points[pivot])
}

fn path_exists(points: &[(usize, usize)]) -> bool {
    // Expensive but worth it to be able to binary search
    let mut points = points.to_vec();
    points.sort_unstable();

    dijkstra::dijkstra(
        &(START_POS.0, START_POS.1),
        |&(x, y): &(usize, usize)| {
            [
                ((x + 1, y), 1),
                ((x, y + 1), 1),
                ((x.saturating_sub(1), y), 1),
                ((x, y.saturating_sub(1)), 1),
            ]
            .into_iter()
            .filter(|&((x, y), _)| {
                x <= MAP_MAX && y <= MAP_MAX && points.binary_search(&(x, y)).is_err()
            })
        },
        |&node| node == (EXIT_POS.0, EXIT_POS.1),
    )
    .is_some()
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = "(65, 6)";

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        eprintln!("Got {actual:?} expected {expected:?}");
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that is automatically run by `cargo test`
    #[test]
    fn test_day() {
        assert!(verify_day(true));
    }
}
