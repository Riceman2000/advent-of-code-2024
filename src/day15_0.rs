// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day15.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let lines: Vec<_> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let section_split = lines.iter().position(|l| l.is_empty()).unwrap();

    let mut player_location = (0, 0);
    let mut grid: Vec<_> = lines[..section_split]
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, b)| {
                    if *b == b'@' {
                        player_location = (x, y);
                        Tile::Empty
                    } else {
                        Tile::from_byte(*b)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let directions: Vec<_> = lines[section_split + 1..]
        .iter()
        .flat_map(|l| l.iter().map(|b| Direction::from_byte(*b)))
        .collect();

    for direction in directions {
        // pretty_print(&grid);
        // println!("direction: {direction:?}");

        let offset = direction.to_coords();
        let new_pos = (
            player_location.0.wrapping_add_signed(offset.0),
            player_location.1.wrapping_add_signed(offset.1),
        );
        let new_pos_tile = grid[new_pos.1][new_pos.0];

        player_location = match new_pos_tile {
            Tile::Empty => new_pos,
            Tile::Wall => player_location,
            Tile::Box => {
                let mut scan_offset: (isize, isize) = (0, 0);
                loop {
                    scan_offset = (scan_offset.0 + offset.0, scan_offset.1 + offset.1);
                    let scan_pos = (
                        player_location.0.wrapping_add_signed(scan_offset.0),
                        player_location.1.wrapping_add_signed(scan_offset.1),
                    );
                    let scan_pos_tile = grid[scan_pos.1][scan_pos.0];

                    match scan_pos_tile {
                        Tile::Empty => {
                            grid[scan_pos.1][scan_pos.0] = Tile::Box;
                            grid[new_pos.1][new_pos.0] = Tile::Empty;
                            break new_pos;
                        }
                        Tile::Wall => {
                            break player_location;
                        }
                        Tile::Box => {
                            continue;
                        }
                    }
                }
            }
        };
    }

    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == Tile::Box {
                sum += 100 * y + x;
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>]) {
    for l in grid {
        for c in l {
            print!("{}", c.to_char());
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
}

impl Tile {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            b'O' => Tile::Box,
            _ => unreachable!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Box => 'O',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => unreachable!(),
        }
    }

    fn to_coords(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_479_679;

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
