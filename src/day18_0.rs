use atoi::atoi;
use pathfinding::directed::dijkstra;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day18.txt");

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

    let out = dijkstra::dijkstra(
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
    .unwrap();

    out.1
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 370;

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        let sign = if actual > expected { '+' } else { '-' };
        eprintln!(
            "Got {actual} expected {expected}, diff {sign}{}",
            expected.abs_diff(actual)
        );
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
