use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("../../input/2024/day24.txt");
aoc_macros::aoc_assert!("gwh,jct,rcb,wbw,wgb,z09,z21,z39");

type Identifier = [u8; 3];
type Nodes = HashMap<Identifier, Option<bool>>;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> String {
    let lines: Vec<_> = INPUT.trim_ascii().split(|b| *b == b'\n').collect();
    let (initial_conditions, connections_raw) = lines.split_once(|l| l.is_empty()).unwrap();

    let mut nodes = Nodes::new();
    for ic in initial_conditions {
        let id: Identifier = ic[..3].try_into().unwrap();
        let value = match ic[5] {
            b'1' => Some(true),
            b'0' => Some(false),
            c => unreachable!("Found character '{}' in ICs", c as char),
        };
        nodes.insert(id, value);
    }

    let mut connections = VecDeque::new();
    for connection in connections_raw {
        let gate = match connection[4] {
            b'A' => Gate::And,
            b'O' => Gate::Or,
            b'X' => Gate::Xor,
            c => unreachable!("Found character '{}' in connections", c as char),
        };
        let id0: Identifier = connection[..3].try_into().unwrap();

        // If the gate type is OR the shorter string shifts everything over
        let id1: Identifier = if gate == Gate::Or {
            connection[7..10].try_into().unwrap()
        } else {
            connection[8..11].try_into().unwrap()
        };
        let output: Identifier = if gate == Gate::Or {
            connection[14..].try_into().unwrap()
        } else {
            connection[15..].try_into().unwrap()
        };

        nodes.entry(id0).or_insert(None);
        nodes.entry(id1).or_insert(None);

        connections.push_front(Connection {
            id0,
            id1,
            output,
            gate,
        });
    }

    let adder_len = nodes.iter().filter(|n| n.0[0] == b'x').count();
    let last_z: Identifier = format!("z{adder_len:>02}").as_bytes().try_into().unwrap();

    // This algo is very focused on the structure of this specific data set and knowing that it
    // will always follow the pattern of a ripple adder, the thought process behind these checks
    // can be seen in this post:
    // https://old.reddit.com/r/adventofcode/comments/1hla5ql/2024_day_24_part_2_a_guide_on_the_idea_behind_the/
    let mut must_swap = Vec::new();
    for c in &connections {
        // 1. if the output of a gate is z??,
        // then the op must be xor unless last bit
        if c.output[0] == b'z' && c.output != last_z && c.gate != Gate::Xor {
            must_swap.push(c.output);
            continue;
        }

        // 2. if the output of a gate is not z??, and the inputs are not both x?? and y??,
        // the op must not be xor
        if !(c.output[0] == b'z'
            || c.id0[0] == b'x' && c.id1[0] == b'y'
            || c.id0[0] == b'y' && c.id1[0] == b'x')
            && c.gate == Gate::Xor
        {
            must_swap.push(c.output);
            continue;
        }

        // 3. if we are doing id0 XOR id1, and ids are x?? and y??,
        // the output must be XOR'ed with something else later
        if (c.id0[0] == b'x' && c.id1[0] == b'y' || c.id0[0] == b'y' && c.id1[0] == b'x')
            && c.output != *b"z00"
            && c.gate == Gate::Xor
            && !connections.iter().any(|other_c| {
                other_c.gate == Gate::Xor && (other_c.id0 == c.output || other_c.id1 == c.output)
            })
        {
            must_swap.push(c.output);
            continue;
        }

        // 4. similarly, if we are doing id0 AND id1,
        // the output must be OR'ed with something else later
        // -- does not apply for x00 and y00 --
        if c.gate == Gate::And
            && c.id0 != *b"y00"
            && c.id1 != *b"y00"
            && c.id0 != *b"x00"
            && c.id1 != *b"x00"
            && !connections.iter().any(|other_c| {
                other_c.gate == Gate::Or && (other_c.id0 == c.output || other_c.id1 == c.output)
            })
        {
            must_swap.push(c.output);
            continue;
        }
    }

    must_swap.sort_unstable();
    must_swap
        .iter()
        .map(|id| String::from_utf8_lossy(id))
        .join(",")
}

#[derive(Debug, PartialEq, Eq)]
struct Connection {
    id0: Identifier,
    id1: Identifier,
    output: Identifier,
    gate: Gate,
}

#[derive(Debug, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}
