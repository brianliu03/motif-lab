use crate::core::{Motif, Pitch};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionGraph {
    pub nodes: BTreeSet<Pitch>,
    pub edges: BTreeMap<Pitch, BTreeMap<Pitch, usize>>,
}

pub fn transition_graph(motif: &Motif) -> TransitionGraph {
    let mut nodes = BTreeSet::new();
    let mut edges: BTreeMap<Pitch, BTreeMap<Pitch, usize>> = BTreeMap::new();

    for note in &motif.notes {
        nodes.insert(note.pitch);
    }

    for pair in motif.notes.windows(2) {
        let from = pair[0].pitch;
        let to = pair[1].pitch;
        *edges.entry(from).or_default().entry(to).or_default() += 1;
    }

    TransitionGraph { nodes, edges }
}

pub fn weighted_walk(
    graph: &TransitionGraph,
    start: Option<Pitch>,
    steps: usize,
    seed: u64,
) -> Vec<Pitch> {
    if steps == 0 || graph.edges.is_empty() {
        return Vec::new();
    }

    let mut rng = DeterministicRng::new(seed);
    let mut current = start
        .filter(|pitch| graph.edges.contains_key(pitch))
        .or_else(|| graph.first_source());
    let mut walk = Vec::new();

    while walk.len() < steps {
        let Some(source) = current else {
            break;
        };

        let Some(targets) = graph.edges.get(&source) else {
            current = graph.first_source();
            continue;
        };

        let Some(next) = choose_weighted(targets, &mut rng) else {
            current = graph.first_source();
            continue;
        };

        walk.push(next);
        current = Some(next);
    }

    walk
}

impl TransitionGraph {
    fn first_source(&self) -> Option<Pitch> {
        self.edges.keys().next().copied()
    }
}

fn choose_weighted(
    targets: &BTreeMap<Pitch, usize>,
    rng: &mut DeterministicRng,
) -> Option<Pitch> {
    let total_weight = targets.values().sum::<usize>();
    if total_weight == 0 {
        return None;
    }

    let mut roll = rng.next_usize(total_weight);
    for (pitch, weight) in targets {
        if roll < *weight {
            return Some(*pitch);
        }
        roll -= weight;
    }

    None
}

#[derive(Debug, Clone)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn next_usize(&mut self, upper_bound: usize) -> usize {
        (self.next_u64() % upper_bound as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::parse::parse_motif;

    #[test]
    fn builds_weighted_pitch_transition_graph() {
        let motif = parse_motif("C4:1 D4:1 E4:1 D4:1 E4:1").unwrap();
        let graph = transition_graph(&motif);
        let c4 = "C4".parse::<Pitch>().unwrap();
        let d4 = "D4".parse::<Pitch>().unwrap();
        let e4 = "E4".parse::<Pitch>().unwrap();

        assert_eq!(graph.edges[&c4][&d4], 1);
        assert_eq!(graph.edges[&d4][&e4], 2);
        assert_eq!(graph.edges[&e4][&d4], 1);
    }

    #[test]
    fn weighted_walk_is_deterministic_for_a_seed() {
        let motif = parse_motif("C4:1 D4:1 E4:1 D4:1 F4:1 D4:1 E4:1").unwrap();
        let graph = transition_graph(&motif);
        let start = motif.notes.last().map(|note| note.pitch);

        let first = weighted_walk(&graph, start, 8, 42);
        let second = weighted_walk(&graph, start, 8, 42);

        assert_eq!(first, second);
        assert_eq!(first.len(), 8);
    }
}
