use hgraphs_core::{
    HyperEdgeIncidence, HyperEdgeIndex, HyperGraph, NodeIncidence, NodeIndex, Undirected,
};

struct InfinitePrimesHyperGraph;

impl HyperGraph for InfinitePrimesHyperGraph {
    type RawNodeId = usize;
    type RawEdgeId = usize;
    type Directedness = Undirected;
}

fn is_prime(x: usize) -> bool {
    if x <= 1 {
        return false;
    }
    for i in 2..=x.isqrt() {
        if x % i == 0 {
            return false;
        }
    }
    true
}

impl NodeIncidence for InfinitePrimesHyperGraph {
    fn incident_nodes(
        &self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>>> {
        let HyperEdgeIndex(edge_id) = edge;
        if !is_prime(edge_id) {
            return None;
        }

        let mut multiple: usize = 0;
        Some(std::iter::from_fn(move || {
            multiple += 1;
            Some(NodeIndex(multiple * edge_id))
        }))
    }
}

impl HyperEdgeIncidence for InfinitePrimesHyperGraph {
    fn incident_edges(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>>> {
        let NodeIndex(node_id) = node;
        if node_id < 2 {
            return None;
        }
        let mut current = 1;
        Some(std::iter::from_fn(move || {
            while current <= node_id {
                current += 1;
                if node_id % current == 0 && is_prime(current) {
                    return Some(HyperEdgeIndex(current));
                }
            }
            None
        }))
    }
}

fn main() -> Result<(), &'static str> {
    let h = InfinitePrimesHyperGraph;
    // Prime divisors of 12.
    let prime_divisors = h
        .incident_edges(NodeIndex(12))
        .ok_or("node 12 should be valid")?
        .collect::<Vec<_>>();
    assert_eq!(prime_divisors, [HyperEdgeIndex(2), HyperEdgeIndex(3)]);
    // Multiples of 3
    let multiples = h
        .incident_nodes(HyperEdgeIndex(3))
        .ok_or("hyperedge 3 should be valid")?
        .take(6)
        .collect::<Vec<_>>();
    assert_eq!(
        multiples,
        [
            NodeIndex(3),
            NodeIndex(6),
            NodeIndex(9),
            NodeIndex(12),
            NodeIndex(15),
            NodeIndex(18),
        ],
    );
    // Invalid, non-prime edge identifier.
    assert!(h.incident_nodes(HyperEdgeIndex(4)).is_none());
    // Invalid node identifier smaller than 2.
    assert!(h.incident_edges(NodeIndex(1)).is_none());
    Ok(())
}
