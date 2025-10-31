use pathfinding::directed::astar;
use pathfinding::prelude::*;

const INPUT: &[u8] = include_bytes!("../../input/2024/day16.txt");
aoc_macros::aoc_assert!(590);

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

    let out = astar::astar_bag(
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
        |_| 0,
        |&node| node == (exit_pos.0, exit_pos.1, node.2),
    )
    .unwrap();

    // pretty_print(&grid, &out.0);

    let mut path_coords: Vec<_> = out.0.flatten().map(|(x, y, _)| (x, y)).collect();
    path_coords.sort_unstable();
    path_coords.dedup();
    path_coords.len()
}

#[allow(dead_code)]
fn pretty_print(grid: &[Vec<Tile>], paths: &AstarSolution<(usize, usize, Direction)>) {
    let mut grid = grid.to_vec();
    for path in paths.clone() {
        for (x, y, _) in path {
            grid[y][x] = match grid[y][x] {
                Tile::Path(n) => Tile::Path(n + 1),
                Tile::Empty => Tile::Path(1),
                Tile::Start => Tile::Start,
                Tile::Exit => Tile::Exit,
                Tile::Wall => unreachable!(),
            };
        }
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
    Path(u32),
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

    #[allow(clippy::cast_possible_truncation)]
    fn to_char(self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::Exit => 'E',
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Path(n) => {
                if n >= 9 {
                    '*'
                } else {
                    (n as u8 + b'0') as char
                }
            }
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
