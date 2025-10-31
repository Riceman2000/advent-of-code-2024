use pathfinding::directed::bfs;

const INPUT: &[u8] = include_bytes!("../../input/2024/day20.txt");
aoc_macros::aoc_assert!(1_296);

const SHORTCUT_THRESHOLD: usize = 100;

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

    // Offsets of possible cheat points
    let radius_offsets = [
        (0, -2),
        (-1, -1),
        (0, -1),
        (1, -1),
        (-2, 0),
        (-1, 0),
        (1, 0),
        (2, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (0, 2),
    ];

    let mut sum = 0;
    for (i, (node_x, node_y)) in path.iter().enumerate() {
        let steps_remaining = path.len() - i;

        // Find number of viable shortcuts
        sum += radius_offsets
            .iter()
            .filter_map(|(off_x, off_y)| {
                // Calculate offset points from start location
                let x = node_x.wrapping_add_signed(*off_x);
                let y = node_y.wrapping_add_signed(*off_y);
                if x >= grid[0].len() || y >= grid.len() {
                    None
                } else if let Tile::Path(pos) = grid[y][x] {
                    Some(pos)
                } else {
                    None
                }
            })
            .filter(|shortcut_pos| {
                // Knowing that we traveled two steps during shortcut, determine time saved
                let shortcut_steps_remaning = 2 + path.len() - shortcut_pos;
                let time_saved = steps_remaining.saturating_sub(shortcut_steps_remaning);
                time_saved >= SHORTCUT_THRESHOLD
            })
            .count();
    }
    sum
}

#[allow(dead_code)]
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
