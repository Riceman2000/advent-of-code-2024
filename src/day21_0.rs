use std::collections::{BinaryHeap, HashMap};

use atoi::atoi;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day21.txt");

const ROBOT_DEPTH: usize = 2;

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let codes: Vec<_> = INPUT.trim_ascii().split(|b| *b == b'\n').collect();

    let mut sum = 0;
    for code in codes {
        let num: usize = atoi(&code[..3]).unwrap();
        let chars = code.iter().map(|c| *c as char).collect();
        // println!("Num: {num}, chars: {chars:?}");
        let len = solve(chars);
        // println!("Len: {len}");
        sum += num * len;
    }
    sum
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn execute(&self, instruction: char, pad: &HashMap<Point, char>) -> (Self, Option<char>) {
        match instruction {
            '^' => (Self::new(self.row - 1, self.col), None),
            '>' => (Self::new(self.row, self.col + 1), None),
            'v' => (Self::new(self.row + 1, self.col), None),
            '<' => (Self::new(self.row, self.col - 1), None),
            'A' => (Self::new(self.row, self.col), Some(pad[self])),
            c => unreachable!("Encountered impossible direction char '{c}'"),
        }
    }
}

fn make_numpad() -> HashMap<Point, char> {
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
        (Point::new(0, 0), '7'),
        (Point::new(0, 1), '8'),
        (Point::new(0, 2), '9'),
        (Point::new(1, 0), '4'),
        (Point::new(1, 1), '5'),
        (Point::new(1, 2), '6'),
        (Point::new(2, 0), '1'),
        (Point::new(2, 1), '2'),
        (Point::new(2, 2), '3'),
        (Point::new(3, 1), '0'),
        (Point::new(3, 2), 'A'),
    ])
}

fn make_dirpad() -> HashMap<Point, char> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    HashMap::from([
        (Point::new(0, 1), '^'),
        (Point::new(0, 2), 'A'),
        (Point::new(1, 0), '<'),
        (Point::new(1, 1), 'v'),
        (Point::new(1, 2), '>'),
    ])
}

#[derive(Clone, PartialEq, Eq)]
struct NumPadNode {
    cost: usize,
    pos: Point,
    instr: char,
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

fn solve(code: Vec<char>) -> usize {
    let numpad = make_numpad();

    let mut cache = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut costmap = HashMap::new();

    pq.push(NumPadNode {
        cost: 0,
        pos: Point::new(3, 2),
        instr: 'A',
        len: 0,
    });

    while let Some(node) = pq.pop() {
        if node.len == code.len() {
            return node.cost;
        }

        if costmap
            .insert((node.pos, node.instr, node.len), node.cost)
            .is_some()
        {
            continue;
        }

        for new_instr in "^A<v>".chars() {
            let (new_pos, output) = node.pos.execute(new_instr, &numpad);

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
    cost: usize,
    pos: Point,
    instr: char,
    output: Option<char>,
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

type Cache = HashMap<(char, char, usize), usize>;
fn calc_cost(goal: char, prev_instr: char, depth: usize, cache: &mut Cache) -> usize {
    if let Some(&cost) = cache.get(&(goal, prev_instr, depth)) {
        return cost;
    }
    // println!("Running calc cost with depth {depth}");

    let point_to_key = make_dirpad();
    let key_to_point: HashMap<_, _> = point_to_key.iter().map(|(pos, c)| (c, pos)).collect();

    if depth == 0 {
        return 1;
    }

    let mut pq = BinaryHeap::new();

    pq.push(DirPadNode {
        cost: 0,
        pos: *key_to_point[&prev_instr],
        instr: 'A',
        output: None,
    });

    while let Some(node) = pq.pop() {
        if node.output.is_some_and(|key| key == goal) {
            cache.insert((goal, prev_instr, depth), node.cost);
            return node.cost;
        }

        for new_instr in "^A<v>".chars() {
            let (new_pos, new_output) = node.pos.execute(new_instr, &point_to_key);

            if !point_to_key.contains_key(&new_pos) {
                continue;
            }
            if new_output.is_some_and(|instr| instr != goal) {
                continue;
            }

            let new_cost = node.cost + calc_cost(new_instr, node.instr, depth - 1, cache);

            pq.push(DirPadNode {
                cost: new_cost,
                pos: new_pos,
                instr: new_instr,
                output: new_output,
            })
        }
    }

    unreachable!();
}
/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 184_716;

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
