// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day16-short.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let mut start_pos = (0, 0);
    let mut exit_pos = (0, 0);
    let mut grid: Vec<_> = INPUT
        .trim_ascii()
        .split(|c| *c == b'\n')
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'S' {
                        start_pos = (x, y);
                        Tile::Empty
                    } else if *b == b'E' {
                        exit_pos = (x, y);
                        Tile::Empty
                    } else {
                        Tile::from_byte(*b)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    pretty_print(&grid, start_pos, exit_pos);
    0
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>], start_pos: (usize, usize), exit_pos: (usize, usize)) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if (x, y) == start_pos {
                if *c == Tile::Empty {
                    print!("S");
                } else {
                    print!("!");
                }
            } else if (x, y) == exit_pos {
                if *c == Tile::Empty {
                    print!("E");
                } else {
                    print!("X");
                }
            } else {
                print!("{}", c.to_char());
            }
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    WallFresh,
    WallVisited(u64),
}

impl Tile {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'.' => Tile::Empty,
            b'#' => Tile::WallFresh,
            b => unreachable!("Hit char '{}'", b as char),
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::WallFresh | Tile::WallVisited(_) => '#',
        }
    }
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_509_780;

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
