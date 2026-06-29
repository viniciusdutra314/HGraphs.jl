use hgraphs_core::{HyperEdgeIndex, HyperGraph, IncidenceHyperGraph, NodeIndex, Undirected};

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

impl IncidenceHyperGraph for InfinitePrimesHyperGraph {
    fn incident_nodes<'a>(
        &'a self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a> {
        let HyperEdgeIndex(edge_id) = edge;
        if !is_prime(edge_id) {
            return None;
        }

        let mut multiple = 0;
        Some(std::iter::from_fn(move || {
            multiple += 1;
            Some(NodeIndex(multiple * edge_id))
        }))
    }

    fn incident_edges<'a>(
        &'a self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a> {
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

fn main() {
    let h = InfinitePrimesHyperGraph;
    //Prime divisors of 12
    assert_eq!(
        h.incident_edges(NodeIndex(12)).unwrap().collect::<Vec<_>>(),
        [HyperEdgeIndex(2), HyperEdgeIndex(3)]
    );
    // Multiples of 3
    assert_eq!(
        h.incident_nodes(HyperEdgeIndex(3))
            .unwrap()
            .take(6)
            .collect::<Vec<_>>(),
        [
            NodeIndex(3),
            NodeIndex(6),
            NodeIndex(9),
            NodeIndex(12),
            NodeIndex(15),
            NodeIndex(18)
        ]
    );
    // Trying to access invalid (non prime) edge-id (return None)
    assert!(h.incident_nodes(HyperEdgeIndex(4)).is_none());
    // Trying to access invalid (smaller than 2) vertex-id (return None)
    assert!(h.incident_edges(NodeIndex(1)).is_none());
}
