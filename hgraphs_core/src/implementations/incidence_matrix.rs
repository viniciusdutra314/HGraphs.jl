use crate::{EdgeCountable, HyperGraph, IncidenceHyperGraph, NodeCountable, Undirected};
use std::collections::HashSet;

// Inspired by SimpleHypergraphs.jl
#[derive(Default)]
struct IncidenceMatrix {
    vertex2hyperedge: Vec<HashSet<usize>>,
    hyperedge2vertex: Vec<HashSet<usize>>,
}

impl IncidenceMatrix {
    pub fn new(num_nodes: usize, num_edges: usize) -> Self {
        IncidenceMatrix {
            vertex2hyperedge: (0..num_nodes).map(|_| HashSet::new()).collect(),
            hyperedge2vertex: (0..num_edges).map(|_| HashSet::new()).collect(),
        }
    }
    pub fn set_incidence(
        &mut self,
        node: usize,
        hyperedge: usize,
        is_incident: bool,
    ) -> Option<bool> {
        let node_edges = self.vertex2hyperedge.get_mut(node)?;
        let edge_nodes = self.hyperedge2vertex.get_mut(hyperedge)?;
        let was_incident = node_edges.contains(&hyperedge);
        match is_incident {
            true => {
                node_edges.insert(hyperedge);
                edge_nodes.insert(node);
            }
            false => {
                node_edges.remove(&hyperedge);
                edge_nodes.remove(&node);
            }
        }
        Some(was_incident)
    }
    pub fn get_incidence(&self, node: usize, hyperedge: usize) -> Option<bool> {
        self.hyperedge2vertex.get(hyperedge)?;
        Some(self.vertex2hyperedge.get(node)?.contains(&hyperedge))
    }
}

impl HyperGraph for IncidenceMatrix {
    type Directedness = Undirected;
    type NodeId = usize;
    type HyperEdgeId = usize;
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
    type EdgeNodeIter<'a> = std::iter::Copied<std::collections::hash_set::Iter<'a, usize>>;
    type NodeEdgeIter<'a> = std::iter::Copied<std::collections::hash_set::Iter<'a, usize>>;
    fn nodes_of_edge<'a>(&'a self, edge: Self::HyperEdgeId) -> Option<Self::EdgeNodeIter<'a>> {
        Some(self.hyperedge2vertex.get(edge)?.iter().copied())
    }
    unsafe fn nodes_of_edge_unchecked<'a>(
        &'a self,
        edge: Self::HyperEdgeId,
    ) -> Self::EdgeNodeIter<'a> {
        unsafe { self.hyperedge2vertex.get_unchecked(edge).iter().copied() }
    }
    fn edges_of_node<'a>(&'a self, node: Self::NodeId) -> Option<Self::NodeEdgeIter<'a>> {
        Some(self.vertex2hyperedge.get(node)?.iter().copied())
    }

    unsafe fn edges_of_node_unchecked<'a>(&'a self, node: Self::NodeId) -> Self::NodeEdgeIter<'a> {
        unsafe { self.vertex2hyperedge.get_unchecked(node).iter().copied() }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_initialization() {
        let graph = IncidenceMatrix::new(5, 10);
        assert_eq!(graph.num_nodes(), 5);
        assert_eq!(graph.num_edges(), 10);
        for i in 0..5 {
            assert_eq!(graph.edges_of_node(i).unwrap().count(), 0);
        }
        for j in 0..10 {
            assert_eq!(graph.nodes_of_edge(j).unwrap().count(), 0);
        }
    }

    #[test]
    fn test_set_and_get_incidence() {
        let mut graph = IncidenceMatrix::new(3, 3);
        let old_val = graph.set_incidence(0, 2, true);
        assert_eq!(old_val, Some(false));

        assert_eq!(graph.get_incidence(0, 2), Some(true));
        assert_eq!(graph.get_incidence(0, 1), Some(false));

        let old_val_again = graph.set_incidence(0, 2, true);
        assert_eq!(old_val_again, Some(true));
    }

    #[test]
    fn test_remove_incidence() {
        let mut graph = IncidenceMatrix::new(4, 4);

        graph.set_incidence(2, 1, true);
        assert_eq!(graph.get_incidence(2, 1), Some(true));

        let old_val = graph.set_incidence(2, 1, false);
        assert_eq!(old_val, Some(true)); // Removals return true if it *was* present

        assert_eq!(graph.get_incidence(2, 1), Some(false));
        assert_eq!(graph.nodes_of_edge(1).unwrap().count(), 0);
        assert_eq!(graph.edges_of_node(2).unwrap().count(), 0);

        // Remove a non-existent incidence
        let old_val_absent = graph.set_incidence(2, 1, false);
        assert_eq!(old_val_absent, Some(false));
    }

    #[test]
    fn test_bounds_protection() {
        let mut graph = IncidenceMatrix::new(2, 2);

        assert_eq!(graph.set_incidence(5, 1, true), None);
        assert_eq!(graph.set_incidence(1, 5, true), None);
        assert_eq!(graph.set_incidence(5, 5, true), None);
        assert_eq!(graph.get_incidence(5, 1), None);
        assert_eq!(graph.get_incidence(1, 5), None);
    }

    #[test]
    fn test_iterator_contents() {
        let mut graph = IncidenceMatrix::new(5, 5);

        graph.set_incidence(0, 1, true);
        graph.set_incidence(2, 1, true);
        graph.set_incidence(4, 1, true);

        let mut nodes: Vec<usize> = graph.nodes_of_edge(1).unwrap().collect();
        nodes.sort_unstable();
        assert_eq!(nodes, vec![0, 2, 4]);
        assert!(graph.edges_of_node(2).unwrap().any(|edge| edge == 1));
    }

    #[test]
    fn test_unchecked_variants_correctness() {
        let mut graph = IncidenceMatrix::new(3, 3);
        graph.set_incidence(0, 0, true);
        graph.set_incidence(0, 1, true);
        graph.set_incidence(1, 1, true);
        graph.set_incidence(2, 2, true);

        unsafe {
            let mut edge_1_nodes: Vec<usize> = graph.nodes_of_edge_unchecked(1).collect();
            edge_1_nodes.sort_unstable();
            assert_eq!(edge_1_nodes, vec![0, 1]);

            let edge_2_nodes: Vec<usize> = graph.nodes_of_edge_unchecked(2).collect();
            assert_eq!(edge_2_nodes, vec![2]);

            let edge_0_nodes: Vec<usize> = graph.nodes_of_edge_unchecked(0).collect();
            assert_eq!(edge_0_nodes, vec![0]);

            let mut node_0_edges: Vec<usize> = graph.edges_of_node_unchecked(0).collect();
            node_0_edges.sort_unstable();
            assert_eq!(node_0_edges, vec![0, 1]);

            let node_2_edges: Vec<usize> = graph.edges_of_node_unchecked(2).collect();
            assert_eq!(node_2_edges, vec![2]);
        }
    }

    #[test]
    fn test_unchecked_empty_structures() {
        let graph = IncidenceMatrix::new(1, 1);

        unsafe {
            let mut node_iter = graph.nodes_of_edge_unchecked(0);
            assert_eq!(node_iter.next(), None);
            assert_eq!(node_iter.count(), 0);

            let mut edge_iter = graph.edges_of_node_unchecked(0);
            assert_eq!(edge_iter.next(), None);
            assert_eq!(edge_iter.count(), 0);
        }
    }
}
