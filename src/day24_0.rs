use std::collections::{HashMap, VecDeque};

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day24.txt");

type Identifier = [u8; 3];
type Nodes = HashMap<Identifier, Option<bool>>;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let lines: Vec<_> = INPUT.trim_ascii().split(|b| *b == b'\n').collect();
    let (initial_conditions, connections) = lines.split_once(|l| l.is_empty()).unwrap();

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

    let mut connection_list = VecDeque::new();
    for connection in connections {
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

        connection_list.push_front(Connection {
            id0,
            id1,
            output,
            gate,
        });
    }

    while !connection_list.is_empty() {
        let connection = connection_list.pop_front().unwrap();

        let Some(node_0) = nodes[&connection.id0] else {
            connection_list.push_back(connection);
            continue;
        };
        let Some(node_1) = nodes[&connection.id1] else {
            connection_list.push_back(connection);
            continue;
        };

        let new_state = match connection.gate {
            Gate::And => node_0 && node_1,
            Gate::Or => node_0 || node_1,
            Gate::Xor => node_0 != node_1,
        };

        *nodes.entry(connection.output).or_insert(None) = Some(new_state);
    }

    let mut nodes: Vec<_> = nodes
        .iter()
        .filter_map(|(k, v)| {
            if k[0] == b'z' {
                Some((k, v.unwrap()))
            } else {
                None
            }
        })
        .collect();
    nodes.sort_unstable_by(|a, b| b.0.cmp(a.0));

    nodes.iter().fold(
        0,
        |acc, (_id, v)| {
            if *v {
                (acc << 1) | 1
            } else {
                acc << 1
            }
        },
    )
}

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

/// Used lts at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 57_270_694_330_992;

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
