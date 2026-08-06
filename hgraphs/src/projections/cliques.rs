use hgraphs_core::{
    AllocatableHyperGraph, Capacity, ExtendableHyperGraph, MutableIncidenceHyperGraph,
};
use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers, NodeCount};
use std::collections::{HashMap, TryReserveError};
use std::error::Error;
use std::fmt;
use std::hash::Hash;

#[derive(Debug)]
pub enum CliqueProjectionError {
    Allocation(TryReserveError),
    MissingNodeMapping,
    IncidenceRejected,
}

impl fmt::Display for CliqueProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(error) => error.fmt(formatter),
            Self::MissingNodeMapping => formatter.write_str("clique contains an unmapped node"),
            Self::IncidenceRejected => {
                formatter.write_str("output hypergraph rejected a generated incidence")
            }
        }
    }
}

impl Error for CliqueProjectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Allocation(error) => Some(error),
            Self::MissingNodeMapping | Self::IncidenceRejected => None,
        }
    }
}

impl From<TryReserveError> for CliqueProjectionError {
    fn from(error: TryReserveError) -> Self {
        Self::Allocation(error)
    }
}

/// Creates a hypergraph whose hyperedges are the maximal cliques of `g`.
pub fn hypergraph_from_cliques<G, H>(g: G) -> Result<H, CliqueProjectionError>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors + NodeCount,
    G::NodeId: Eq + Hash,
    H: AllocatableHyperGraph + ExtendableHyperGraph + Default + MutableIncidenceHyperGraph,
{
    let node_count = g.node_count();
    // TODO: Implement maximal-clique enumeration lazily to avoid collecting every clique.
    let cliques = petgraph::algo::maximal_cliques(&g);
    let mut hypergraph = H::with_capacity(Capacity {
        num_nodes: Some(node_count),
        num_hyperedges: Some(cliques.len()),
    })?;

    let mut graph_to_hypergraph_node_ids = HashMap::new();
    graph_to_hypergraph_node_ids.try_reserve(node_count)?;
    graph_to_hypergraph_node_ids
        .extend(g.node_identifiers().zip(hypergraph.add_nodes(node_count)?));

    let hyperedge_iter = hypergraph.add_hyperedges(cliques.len())?;
    for (hyperedge, clique) in hyperedge_iter.zip(cliques.iter()) {
        for graph_node_id in clique {
            let hypergraph_node_id = graph_to_hypergraph_node_ids
                .get(graph_node_id)
                .copied()
                .ok_or(CliqueProjectionError::MissingNodeMapping)?;
            hypergraph
                .add_incidence(hypergraph_node_id, hyperedge)
                .ok_or(CliqueProjectionError::IncidenceRejected)?;
        }
    }
    Ok(hypergraph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::IncidenceMatrix;
    use hgraphs_core::*;
    use petgraph::graph::UnGraph;
    use std::collections::HashSet;
    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    fn clique_projection_suite<H>() -> TestResult
    where
        H: HyperGraph
            + AllocatableHyperGraph
            + ExtendableHyperGraph
            + MutableIncidenceHyperGraph
            + NodeCountable
            + HyperEdgeCountable
            + HyperEdgeList
            + NodeIncidence
            + Default,
    {
        let mut graph = UnGraph::<(), ()>::new_undirected();
        let first = graph.add_node(());
        let second = graph.add_node(());
        let third = graph.add_node(());
        let fourth = graph.add_node(());

        // first ----- second
        //   \         /
        //    \       /
        //      third ----- fourth
        graph.add_edge(first, second, ());
        graph.add_edge(first, third, ());
        graph.add_edge(second, third, ());
        graph.add_edge(third, fourth, ());

        let hypergraph: H = hypergraph_from_cliques(&graph)?;
        assert_eq!(hypergraph.num_nodes(), 4);
        assert_eq!(hypergraph.num_hyperedges(), 2);

        let mut projected_cliques = Vec::new();
        for hyperedge_id in hypergraph.hyperedge_identifiers() {
            let clique = hypergraph
                .incident_nodes(hyperedge_id)
                .ok_or("generated hyperedge is invalid")?
                .collect::<HashSet<_>>();
            projected_cliques.push(clique);
        }
        projected_cliques.sort_unstable_by_key(|clique| clique.len());

        let mut projected_cliques = projected_cliques.into_iter();
        let pair = projected_cliques.next().ok_or("missing two-node clique")?;
        let triangle = projected_cliques
            .next()
            .ok_or("missing three-node clique")?;

        assert_eq!(pair.len(), 2);
        assert_eq!(triangle.len(), 3);
        assert_eq!(pair.intersection(&triangle).count(), 1);
        assert_eq!(pair.union(&triangle).count(), 4);
        assert!(projected_cliques.next().is_none());
        Ok(())
    }

    #[test]
    fn incidence_matrix_projects_maximal_cliques() -> TestResult {
        clique_projection_suite::<IncidenceMatrix>()
    }
}
