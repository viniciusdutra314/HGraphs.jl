use std::collections::VecDeque;

use crate::{IncidenceHyperGraph, PropertyMapError, PropertyMapReadWrite};

#[derive(PartialEq)]
pub enum BfsEvent<HG: IncidenceHyperGraph> {
    NodeFound(HG::NodeId),
    EdgeFound(HG::HyperEdgeId),
}

#[derive(PartialEq)]
pub enum BfsState {
    Visited,
    Processing,
    Unvisited,
}

pub fn bfs<HG, Visitor, NodeMap>(
    hg: &HG,
    start_node: HG::NodeId,
    node_map: &mut NodeMap,
    visitor: &mut Visitor,
) -> Result<(), PropertyMapError>
where
    HG: IncidenceHyperGraph,
    NodeMap: PropertyMapReadWrite<Key = HG::NodeId, Value = BfsState>,
    Visitor: FnMut(BfsEvent<HG>),
{
    let mut queue = VecDeque::new();
    queue.push_back(start_node);
    node_map.set(start_node, BfsState::Processing)?;

    while let Some(node) = queue.pop_front() {
        visitor(BfsEvent::NodeFound(node));
        node_map.set(node, BfsState::Visited)?;
        for edge in hg.edges_of_node(node) {
            visitor(BfsEvent::EdgeFound(edge));
            for neighbor in hg.nodes_of_edge(edge) {
                let is_visited = node_map.get(neighbor).unwrap_or(&BfsState::Unvisited);
                if *is_visited == BfsState::Unvisited {
                    queue.push_back(neighbor);
                    node_map.set(neighbor, BfsState::Processing)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
