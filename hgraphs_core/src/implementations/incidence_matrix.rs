use crate::{
    AllocatableHyperGraph, Capacity, EdgeCountable, ExtendableHyperGraph, HyperEdgeIndex,
    HyperGraph, IncidenceHyperGraph, MutableIncidenceHyperGraph, NodeCountable, NodeIndex,
    Undirected,
};

use std::collections::{HashSet, TryReserveError};

// Inspired by SimpleHypergraphs.jl
#[derive(Default)]
struct IncidenceMatrix {
    vertex2hyperedge: Vec<HashSet<HyperEdgeIndex<usize>>>,
    hyperedge2vertex: Vec<HashSet<NodeIndex<usize>>>,
}

impl HyperGraph for IncidenceMatrix {
    type Directedness = Undirected;
    type RawNodeId = usize;
    type RawEdgeId = usize;
}

impl AllocatableHyperGraph for IncidenceMatrix {
    fn try_reserve_exact(&mut self, additional: Capacity) -> Result<(), TryReserveError> {
        self.vertex2hyperedge
            .try_reserve_exact(additional.num_nodes.unwrap_or(0))?;
        self.hyperedge2vertex
            .try_reserve_exact(additional.num_hyperedges.unwrap_or(0))?;
        Ok(())
    }
}

impl MutableIncidenceHyperGraph for IncidenceMatrix {
    fn add_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()> {
        let node_edges = self.vertex2hyperedge.get_mut(node.0)?;
        let edge_nodes = self.hyperedge2vertex.get_mut(hyperedge.0)?;
        node_edges.insert(hyperedge);
        edge_nodes.insert(node);
        Some(())
    }
    fn remove_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()> {
        let node_edges = self.vertex2hyperedge.get_mut(node.0)?;
        let edge_nodes = self.hyperedge2vertex.get_mut(hyperedge.0)?;
        node_edges.remove(&hyperedge);
        edge_nodes.remove(&node);
        Some(())
    }
}

impl NodeCountable for IncidenceMatrix {
    fn num_nodes(&self) -> usize {
        self.vertex2hyperedge.len()
    }
}

impl EdgeCountable for IncidenceMatrix {
    fn num_edges(&self) -> usize {
        self.hyperedge2vertex.len()
    }
}

impl IncidenceHyperGraph for IncidenceMatrix {
    fn incident_nodes<'a>(
        &'a self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a> {
        Some(self.hyperedge2vertex.get(edge.0)?.iter().copied())
    }

    unsafe fn incident_nodes_unchecked<'a>(
        &'a self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a {
        unsafe { self.hyperedge2vertex.get_unchecked(edge.0).iter().copied() }
    }
    fn incident_edges<'a>(
        &'a self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a> {
        Some(self.vertex2hyperedge.get(node.0)?.iter().copied())
    }
    unsafe fn incident_edges_unchecked<'a>(
        &'a self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a {
        unsafe { self.vertex2hyperedge.get_unchecked(node.0).iter().copied() }
    }
}

impl ExtendableHyperGraph for IncidenceMatrix {
    fn try_add_nodes<'a>(
        &mut self,
        num_nodes: usize,
    ) -> Result<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a, TryReserveError> {
        let current_len = self.vertex2hyperedge.len();
        let current_cap = self.vertex2hyperedge.capacity();
        let target_len = current_len + num_nodes;
        if target_len > current_cap {
            let missing_capacity = target_len - current_cap;
            self.vertex2hyperedge.try_reserve_exact(missing_capacity)?;
        }
        self.vertex2hyperedge
            .extend(std::iter::repeat_with(HashSet::new).take(num_nodes));
        let end = self.vertex2hyperedge.len();
        Ok((current_len..end).map(NodeIndex))
    }

    fn try_add_hyperedges<'a>(
        &mut self,
        num_hyperedges: usize,
    ) -> Result<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a, TryReserveError> {
        let current_len = self.hyperedge2vertex.len();
        let current_capacity = self.hyperedge2vertex.capacity();
        let target_len = current_len + num_hyperedges;
        if current_capacity < target_len {
            let missing_capacity = target_len - current_capacity;
            self.hyperedge2vertex.try_reserve_exact(missing_capacity)?;
        }
        self.hyperedge2vertex
            .extend(std::iter::repeat_with(HashSet::new).take(num_hyperedges));
        let end = self.hyperedge2vertex.len();
        Ok((current_len..end).map(HyperEdgeIndex))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_allocation_and_initialization() -> Result<(), &'static str> {
        let mut graph = IncidenceMatrix::try_with_capacity(Capacity {
            num_nodes: Some(5),
            num_hyperedges: Some(10),
        })
        .map_err(|_| "failed to initialize graph")?;
        assert_eq!(graph.num_nodes(), 0);
        assert_eq!(graph.num_edges(), 0);
        for node_id in graph.try_add_nodes(5).map_err(|_| "failed to add nodes")? {
            assert_eq!(graph.incident_edges(node_id).unwrap().count(), 0);
        }
        assert_eq!(graph.num_nodes(), 5);
        assert_eq!(graph.num_edges(), 0);
        for hyperedge_id in graph
            .try_add_hyperedges(10)
            .map_err(|_| "failed to add edges")?
        {
            assert_eq!(graph.incident_nodes(hyperedge_id).unwrap().count(), 0);
        }
        assert_eq!(graph.num_nodes(), 5);
        assert_eq!(graph.num_edges(), 10);
        Ok(())
    }

    // #[test]
    // fn test_set_and_get_incidence() {
    //     let mut graph = IncidenceMatrix::new(3, 3);
    //     let old_val = graph.set_incidence(NodeIndex(0), HyperEdgeIndex(2), true);
    //     assert_eq!(old_val, Some(false));

    //     assert_eq!(
    //         graph.get_incidence(NodeIndex(0), HyperEdgeIndex(2)),
    //         Some(true)
    //     );
    //     assert_eq!(
    //         graph.get_incidence(NodeIndex(0), HyperEdgeIndex(1)),
    //         Some(false)
    //     );

    //     let old_val_again = graph.set_incidence(NodeIndex(0), HyperEdgeIndex(2), true);
    //     assert_eq!(old_val_again, Some(true));
    // }

    // #[test]
    // fn test_remove_incidence() {
    //     let mut graph = IncidenceMatrix::new(4, 4);

    //     graph.set_incidence(2, 1, true);
    //     assert_eq!(graph.get_incidence(2, 1), Some(true));

    //     let old_val = graph.set_incidence(2, 1, false);
    //     assert_eq!(old_val, Some(true));
    //     assert_eq!(graph.get_incidence(2, 1), Some(false));
    //     assert_eq!(graph.incident_nodes(1).unwrap().count(), 0);
    //     assert_eq!(graph.incident_edges(2).unwrap().count(), 0);

    //     let old_val_absent = graph.set_incidence(2, 1, false);
    //     assert_eq!(old_val_absent, Some(false));
    // }

    // #[test]
    // fn test_bounds_protection() {
    //     let mut graph = IncidenceMatrix::new(2, 2);

    //     assert_eq!(graph.set_incidence(5, 1, true), None);
    //     assert_eq!(graph.set_incidence(1, 5, true), None);
    //     assert_eq!(graph.set_incidence(5, 5, true), None);
    //     assert_eq!(graph.get_incidence(5, 1), None);
    //     assert_eq!(graph.get_incidence(1, 5), None);
    // }

    // #[test]
    // fn test_iterator_contents() {
    //     let mut graph = IncidenceMatrix::new(5, 5);

    //     graph.set_incidence(0, 1, true);
    //     graph.set_incidence(2, 1, true);
    //     graph.set_incidence(4, 1, true);

    //     let mut nodes: Vec<usize> = graph.incident_nodes(1).unwrap().collect();
    //     nodes.sort_unstable();
    //     assert_eq!(nodes, vec![0, 2, 4]);
    //     assert!(graph.incident_edges(2).unwrap().any(|edge| edge == 1));
    // }

    // #[test]
    // fn test_unchecked_variants_correctness() {
    //     let mut graph = IncidenceMatrix::new(3, 3);
    //     graph.set_incidence(0, 0, true);
    //     graph.set_incidence(0, 1, true);
    //     graph.set_incidence(1, 1, true);
    //     graph.set_incidence(2, 2, true);

    //     unsafe {
    //         let mut edge_1_nodes: Vec<usize> = graph.incident_nodes_unchecked(1).collect();
    //         edge_1_nodes.sort_unstable();
    //         assert_eq!(edge_1_nodes, vec![0, 1]);

    //         let edge_2_nodes: Vec<usize> = graph.incident_nodes_unchecked(2).collect();
    //         assert_eq!(edge_2_nodes, vec![2]);

    //         let edge_0_nodes: Vec<usize> = graph.incident_nodes_unchecked(0).collect();
    //         assert_eq!(edge_0_nodes, vec![0]);

    //         let mut node_0_edges: Vec<usize> = graph.incident_edges_unchecked(0).collect();
    //         node_0_edges.sort_unstable();
    //         assert_eq!(node_0_edges, vec![0, 1]);

    //         let node_2_edges: Vec<usize> = graph.incident_edges_unchecked(2).collect();
    //         assert_eq!(node_2_edges, vec![2]);
    //     }
    // }

    // #[test]
    // fn test_unchecked_empty_structures() {
    //     let graph = IncidenceMatrix::new(1, 1);

    //     unsafe {
    //         let mut node_iter = graph.incident_nodes_unchecked(0);
    //         assert_eq!(node_iter.next(), None);
    //         assert_eq!(node_iter.count(), 0);

    //         let mut edge_iter = graph.incident_edges_unchecked(0);
    //         assert_eq!(edge_iter.next(), None);
    //         assert_eq!(edge_iter.count(), 0);
    //     }
    // }
}
