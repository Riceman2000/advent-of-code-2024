use pathfinding::directed::bfs;
use rayon::prelude::*;

const INPUT: &[u8] = include_bytes!("../../input/2024/day20.txt");
aoc_assert::aoc_assert!(977_665);

const CHEAT_RADIUS: isize = 20;
const SHORTCUT_THRESHOLD: usize = 100;

// Calculate offset vectors in a const context
#[allow(clippy::cast_sign_loss)]
const CHEAT_RADIUS_BUFFER_SIZE: usize =
    (2 * CHEAT_RADIUS.pow(2) + 2 * CHEAT_RADIUS + 1) as usize - 9;

#[allow(clippy::cast_sign_loss)]
#[allow(long_running_const_eval)]
const RADIUS_OFFSETS: [((isize, isize), usize); CHEAT_RADIUS_BUFFER_SIZE] = {
    let mut radius_offsets = [((0, 0), 0); CHEAT_RADIUS_BUFFER_SIZE];
    let mut index = 0;

    // Diamond shape
    let mut i = -CHEAT_RADIUS;
    while i <= CHEAT_RADIUS {
        let j_lim = CHEAT_RADIUS - i.abs();
        let mut j = -j_lim;
        while j <= j_lim {
            // Reject center 9 tiles
            if i.abs() <= 1 && j.abs() <= 1 {
                j += 1;
                continue;
            }

            let dist = i.abs() + j.abs();
            radius_offsets[index] = ((i, j), dist as usize);
            index += 1;
            j += 1;
        }
        i += 1;
    }

    radius_offsets
};

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
                    } else if *b == b'E' {
                        exit_pos = (x, y);
                    }
                    Tile::from_byte(*b)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let path = bfs::bfs(
        &(start_pos.0, start_pos.1),
        |&(x, y): &(usize, usize)| {
            [
                (x + 1, y),
                (x, y + 1),
                (x.saturating_sub(1), y),
                (x, y.saturating_sub(1)),
            ]
            .into_iter()
            .filter(|&(x, y)| x <= grid[0].len() && y <= grid.len() && grid[y][x] != Tile::Wall)
        },
        |&node| node == (exit_pos.0, exit_pos.1),
    )
    .unwrap();

    for (i, (x, y)) in path.iter().enumerate() {
        grid[*y][*x] = Tile::Path(i);
    }

    // pretty_print(&grid);

    path.par_iter()
        .enumerate()
        .fold(
            || 0,
            |acc, (i, (node_x, node_y))| {
                let steps_remaining = path.len() - i;
                acc + RADIUS_OFFSETS
                    .iter()
                    .filter(|((off_x, off_y), dist)| {
                        // Calculate offset points from start location
                        let x = node_x.wrapping_add_signed(*off_x);
                        let y = node_y.wrapping_add_signed(*off_y);

                        // Extract position to end
                        if x >= grid[0].len() || y >= grid.len() {
                            return false;
                        }
                        let tile = grid[y][x];
                        let Tile::Path(shortcut_pos) = tile else {
                            return false;
                        };

                        // Determine if shortcut is valid
                        let shortcut_steps_remaning = dist + path.len() - shortcut_pos;
                        let time_saved = steps_remaining.saturating_sub(shortcut_steps_remaning);
                        time_saved >= SHORTCUT_THRESHOLD
                    })
                    .count()
            },
        )
        .sum()
}

#[allow(dead_code)]
#[allow(clippy::match_on_vec_items)]
fn pretty_print(grid: &[Vec<Tile>]) {
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
    Path(usize),
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
            Tile::Path(_) => '.',
        }
    }
}
