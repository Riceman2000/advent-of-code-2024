use pathfinding::directed::dijkstra;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../../input/2024/day16.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let mut start_pos = (0, 0);
    let mut exit_pos = (0, 0);
    let grid: Vec<_> = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'S' {
                        start_pos = (x, y);
                    } else if *b == b'E' {
                        exit_pos = (x, y);
                    }
                    Tile::from_byte(*b)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let out = dijkstra::dijkstra(
        &(start_pos.0, start_pos.1, Direction::E),
        |&(x, y, dir): &(usize, usize, Direction)| {
            let forward_pos = (
                x.wrapping_add_signed(dir.to_coords().0),
                y.wrapping_add_signed(dir.to_coords().1),
            );

            [
                ((x, y, dir.next()), 1000),
                ((forward_pos.0, forward_pos.1, dir), 1),
                ((x, y, dir.prev()), 1000),
            ]
            .into_iter()
            .filter(|&((x, y, _), _)| grid[y][x] != Tile::Wall)
        },
        |&node| node == (exit_pos.0, exit_pos.1, node.2),
    )
    .unwrap();

    // pretty_print(&grid, &out.0);
    out.1
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>], path: &[(usize, usize, Direction)]) {
    let mut grid = grid.to_vec();
    for (x, y, dir) in path {
        grid[*y][*x] = Tile::Path(*dir);
    }
    for line in grid {
        for c in line {
            print!("{}", c.to_char());
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Exit,
    Empty,
    Wall,
    Path(Direction),
}

impl Tile {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'S' => Tile::Start,
            b'E' => Tile::Exit,
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            b => unreachable!("Hit char '{}'", b as char),
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::Exit => 'E',
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Path(Direction::N) => '^',
            Tile::Path(Direction::E) => '>',
            Tile::Path(Direction::S) => 'v',
            Tile::Path(Direction::W) => '<',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn to_coords(self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }

    fn next(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn prev(self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 82_460;

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
