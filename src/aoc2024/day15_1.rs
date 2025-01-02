const INPUT: &[u8] = include_bytes!("../../input/2024/day15.txt");
aoc_assert::aoc_assert!(1_509_780);

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let lines: Vec<_> = INPUT.trim_ascii_end().split(|c| *c == b'\n').collect();
    let section_split = lines.iter().position(|l| l.is_empty()).unwrap();

    let mut current_pos = (0, 0);
    let mut grid: Vec<_> = lines[..section_split]
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .flat_map(|(x, b)| {
                    if *b == b'@' {
                        current_pos = (x * 2, y);
                        [Tile::Empty, Tile::Empty]
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
        // pretty_print(&grid, current_pos);
        // println!("direction: {direction:?}");

        let check = flood_check(current_pos, direction, &grid);
        if check {
            let offset = direction.to_coords();
            current_pos = (
                current_pos.0.wrapping_add_signed(offset.0),
                current_pos.1.wrapping_add_signed(offset.1),
            );
            let preshift_tile = grid[current_pos.1][current_pos.0];
            flood_shift(current_pos, direction, &mut grid);
            if direction == Direction::Up || direction == Direction::Down {
                match preshift_tile {
                    Tile::BoxLeft => {
                        flood_shift((current_pos.0 + 1, current_pos.1), direction, &mut grid);
                    }
                    Tile::BoxRight => {
                        flood_shift((current_pos.0 - 1, current_pos.1), direction, &mut grid);
                    }
                    _ => (),
                }
            }
        }
    }

    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == Tile::BoxLeft {
                sum += 100 * y + x;
            }
        }
    }
    sum
}

fn flood_check(pos: (usize, usize), dir: Direction, grid: &Vec<Vec<Tile>>) -> bool {
    let offset = dir.to_coords();
    let new_pos = (
        pos.0.wrapping_add_signed(offset.0),
        pos.1.wrapping_add_signed(offset.1),
    );
    let new_pos_tile = grid[new_pos.1][new_pos.0];

    match new_pos_tile {
        Tile::Wall => return false,
        Tile::Empty => return true,
        _ => (),
    }
    match dir {
        Direction::Left | Direction::Right => flood_check(new_pos, dir, grid),
        Direction::Up | Direction::Down => match new_pos_tile {
            Tile::BoxLeft => {
                flood_check(new_pos, dir, grid)
                    && flood_check((new_pos.0 + 1, new_pos.1), dir, grid)
            }
            Tile::BoxRight => {
                flood_check(new_pos, dir, grid)
                    && flood_check((new_pos.0 - 1, new_pos.1), dir, grid)
            }
            _ => unreachable!(),
        },
    }
}

fn flood_shift(pos: (usize, usize), dir: Direction, grid: &mut Vec<Vec<Tile>>) {
    let offset = dir.to_coords();
    let new_pos = (
        pos.0.wrapping_add_signed(offset.0),
        pos.1.wrapping_add_signed(offset.1),
    );
    let new_pos_tile = grid[new_pos.1][new_pos.0];
    let pos_tile = grid[pos.1][pos.0];

    match pos_tile {
        Tile::Wall => unreachable!(),
        Tile::Empty => return,
        _ => (),
    }
    match dir {
        Direction::Left | Direction::Right => flood_shift(new_pos, dir, grid),
        Direction::Up | Direction::Down => match new_pos_tile {
            Tile::BoxLeft => {
                flood_shift(new_pos, dir, grid);
                flood_shift((new_pos.0 + 1, new_pos.1), dir, grid);
            }
            Tile::BoxRight => {
                flood_shift(new_pos, dir, grid);
                flood_shift((new_pos.0 - 1, new_pos.1), dir, grid);
            }
            Tile::Empty => (),
            Tile::Wall => unreachable!("new_pos is a wall"),
        },
    };
    grid[new_pos.1][new_pos.0] = pos_tile;
    grid[pos.1][pos.0] = Tile::Empty;
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>], player_pos: (usize, usize)) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if (x, y) == player_pos {
                if *c == Tile::Empty {
                    print!("@");
                } else {
                    print!("!");
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
    Wall,
    BoxLeft,
    BoxRight,
}

impl Tile {
    fn from_byte(byte: u8) -> [Self; 2] {
        match byte {
            b'.' => [Tile::Empty, Tile::Empty],
            b'#' => [Tile::Wall, Tile::Wall],
            b'O' => [Tile::BoxLeft, Tile::BoxRight],
            _ => unreachable!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
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
