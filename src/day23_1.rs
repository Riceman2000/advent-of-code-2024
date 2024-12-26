use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day23.txt");

type Identifier = [u8; 2];
type Nodes = BTreeSet<Identifier>;
type Graph = HashMap<Identifier, Nodes>;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> String {
    let pairs = INPUT
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|l| ([l[0], l[1]], [l[3], l[4]]));

    let mut graph: Graph = Graph::new();
    for pair in pairs {
        graph.entry(pair.0).or_default().insert(pair.1);
        graph.entry(pair.1).or_default().insert(pair.0);
    }

    // Bron ICs
    let r = Nodes::new();
    let p: Nodes = graph.keys().copied().collect();
    let x = Nodes::new();
    let mut cliques = Vec::new();

    bron_kerbosch(&graph, r, p, x, &mut cliques);

    cliques.sort_by_key(Nodes::len);
    cliques
        .pop()
        .unwrap()
        .iter()
        .map(|id| String::from_utf8_lossy(id))
        .join(",")
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch(graph: &Graph, r: Nodes, mut p: Nodes, mut x: Nodes, cliques: &mut Vec<Nodes>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }
    if p.is_empty() {
        return;
    }

    let nodes = p.clone();
    for node in nodes {
        let neighbours = graph.get(&node).unwrap();
        let mut to_add = Nodes::new();
        to_add.insert(node);
        bron_kerbosch(
            graph,
            r.union(&to_add).copied().collect(),
            p.intersection(neighbours).copied().collect(),
            x.intersection(neighbours).copied().collect(),
            cliques,
        );
        p.remove(&node);
        x.insert(node);
    }
}

/// Used lts at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = "ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys";

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        eprintln!("Got {actual} expected {expected}",);
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
