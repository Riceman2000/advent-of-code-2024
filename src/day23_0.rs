use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day23.txt");

type Identifier = [u8; 2];
type Nodes = BTreeSet<Identifier>;
type Graph = HashMap<Identifier, Nodes>;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> usize {
    let pairs = INPUT
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|l| ([l[0], l[1]], [l[3], l[4]]));

    let mut nodes: Graph = Graph::new();
    for pair in pairs {
        nodes.entry(pair.0).or_default().insert(pair.1);
        nodes.entry(pair.1).or_default().insert(pair.0);
    }

    let mut seen = HashSet::new();
    let mut id_cache; // Space to hold a collection of IDs to sort them
    for (id0, first_links) in &nodes {
        if id0[0] != b't' || first_links.len() < 2 {
            continue;
        }
        // Find if the pairs create a triangle
        for pairs in first_links.iter().combinations(2) {
            let id1 = pairs[0];
            let id2 = pairs[1];
            let links1 = &nodes[id1];
            let links2 = &nodes[id2];

            if links1.contains(id2) && links2.contains(id1) {
                id_cache = [(id0[0], id0[1]), (id1[0], id1[1]), (id2[0], id2[1])];
                id_cache.sort_unstable();
                seen.insert(id_cache);
            }
        }
    }
    seen.len()
}

/// Used lts at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 1_151;

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
