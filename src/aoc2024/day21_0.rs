use std::collections::{BinaryHeap, HashMap};

use atoi::atoi;
use lazy_static::lazy_static;

const INPUT: &[u8] = include_bytes!("../../input/2024/day21.txt");
aoc_macros::aoc_assert!(184_716);

const ROBOT_DEPTH: u64 = 2;

lazy_static! {
    static ref NUMPAD: HashMap<Point, u8> = {
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        HashMap::from([
            (Point::new(0, 0), b'7'),
            (Point::new(0, 1), b'8'),
            (Point::new(0, 2), b'9'),
            (Point::new(1, 0), b'4'),
            (Point::new(1, 1), b'5'),
            (Point::new(1, 2), b'6'),
            (Point::new(2, 0), b'1'),
            (Point::new(2, 1), b'2'),
            (Point::new(2, 2), b'3'),
            (Point::new(3, 1), b'0'),
            (Point::new(3, 2), b'A'),
        ])
    };

    static ref DIRPAD: HashMap<Point, u8> = {
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        HashMap::from([
            (Point::new(0, 1), b'^'),
            (Point::new(0, 2), b'A'),
            (Point::new(1, 0), b'<'),
            (Point::new(1, 1), b'v'),
            (Point::new(1, 2), b'>'),
        ])
    };

    static ref DIRPAD_INV: HashMap<u8, Point> = {
        DIRPAD.iter().map(|(pos, c)| (*c, *pos)).collect()
    };
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let codes: Vec<_> = INPUT.trim_ascii().split(|b| *b == b'\n').collect();

    let mut sum = 0;
    for code in codes {
        let num: u64 = atoi(&code[..3]).unwrap();
        let len = get_len(code);
        sum += num * len;
    }
    sum
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i8,
    col: i8,
}

impl Point {
    fn new(row: i8, col: i8) -> Self {
        Self { row, col }
    }

    fn execute(self, instruction: u8, pad: &HashMap<Point, u8>) -> (Self, Option<u8>) {
        match instruction {
            b'^' => (Self::new(self.row - 1, self.col), None),
            b'>' => (Self::new(self.row, self.col + 1), None),
            b'v' => (Self::new(self.row + 1, self.col), None),
            b'<' => (Self::new(self.row, self.col - 1), None),
            b'A' => (Self::new(self.row, self.col), Some(pad[&self])),
            c => unreachable!("Encountered impossible direction char '{c}'"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct NumPadNode {
    cost: u64,
    pos: Point,
    instr: u8,
    len: usize,
}

impl Ord for NumPadNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for NumPadNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_len(code: &[u8]) -> u64 {
    let numpad = &NUMPAD;

    let mut cache = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut costmap = HashMap::new();

    // We always start and end pointed at the A key
    pq.push(NumPadNode {
        cost: 0,
        pos: Point::new(3, 2),
        instr: b'A',
        len: 0,
    });

    while let Some(node) = pq.pop() {
        if node.len == code.len() {
            return node.cost;
        }

        // If this path has been explored stop processing it
        if costmap
            .insert((node.pos, node.instr, node.len), node.cost)
            .is_some()
        {
            continue;
        }

        for &new_instr in b"^A<v>" {
            let (new_pos, output) = node.pos.execute(new_instr, numpad);

            // Never go off of the existing pads
            if !numpad.contains_key(&new_pos) {
                continue;
            }

            let mut new_len = node.len;
            if let Some(instr) = output {
                if instr != code[new_len] {
                    continue;
                }
                new_len += 1;
            }

            let new_cost = node.cost + calc_cost(new_instr, node.instr, ROBOT_DEPTH, &mut cache);
            pq.push(NumPadNode {
                cost: new_cost,
                pos: new_pos,
                instr: new_instr,
                len: new_len,
            });
        }
    }

    unreachable!();
}

#[derive(Clone, PartialEq, Eq)]
struct DirPadNode {
    cost: u64,
    pos: Point,
    instr: u8,
    output: Option<u8>,
}

impl Ord for DirPadNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for DirPadNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Cache = HashMap<(u8, u8, u64), u64>;
fn calc_cost(goal: u8, prev_instr: u8, depth: u64, cache: &mut Cache) -> u64 {
    if let Some(&cost) = cache.get(&(goal, prev_instr, depth)) {
        return cost;
    }

    let dirpad = &DIRPAD;
    let dirpad_inv = &DIRPAD_INV;

    if depth == 0 {
        return 1;
    }

    let mut pq = BinaryHeap::new();

    // Start at the previous instruction
    pq.push(DirPadNode {
        cost: 0,
        pos: dirpad_inv[&prev_instr],
        instr: b'A', // This is never really considered
        output: None,
    });

    while let Some(node) = pq.pop() {
        if node.output.is_some_and(|key| key == goal) {
            cache.insert((goal, prev_instr, depth), node.cost);
            return node.cost;
        }

        for &new_instr in b"^A<v>" {
            let (new_pos, new_output) = node.pos.execute(new_instr, dirpad);

            // Never go off the existing pads
            if !dirpad.contains_key(&new_pos) {
                continue;
            }

            // If the output is not at the goal we are not done yet
            if new_output.is_some_and(|instr| instr != goal) {
                continue;
            }

            // Recurse till we find what the true cost will be
            let new_cost = node.cost + calc_cost(new_instr, node.instr, depth - 1, cache);

            pq.push(DirPadNode {
                cost: new_cost,
                pos: new_pos,
                instr: new_instr,
                output: new_output,
            });
        }
    }

    unreachable!();
}
