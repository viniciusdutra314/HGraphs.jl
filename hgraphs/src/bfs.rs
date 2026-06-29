use std::collections::VecDeque;

use crate::{
    HyperEdgeIndex, IncidenceHyperGraph, NodeIndex, PropertyMapError, PropertyMapReadWrite,
};

#[derive(PartialEq)]
pub enum BfsEvent<HG: IncidenceHyperGraph> {
    NodeFound(NodeIndex<HG::RawNodeId>),
    EdgeFound(HyperEdgeIndex<HG::RawEdgeId>),
}

#[derive(PartialEq)]
pub enum BfsState {
    Visited,
    Processing,
    Unvisited,
}

pub fn bfs<HG, Visitor, NodeMap>(
    hg: &HG,
    start_node: NodeIndex<HG::RawNodeId>,
    node_map: &mut NodeMap,
    visitor: &mut Visitor,
) -> Result<(), PropertyMapError>
where
    HG: IncidenceHyperGraph,
    NodeMap: PropertyMapReadWrite<Key = NodeIndex<HG::RawNodeId>, Value = BfsState>,
    Visitor: FnMut(BfsEvent<HG>),
{
    let mut queue = VecDeque::new();
    queue.push_back(start_node);
    node_map.set(start_node, BfsState::Processing)?;

    while let Some(node) = queue.pop_front() {
        visitor(BfsEvent::NodeFound(node));
        node_map.set(node, BfsState::Visited)?;
        unsafe {
            for edge in hg.incident_edges_unchecked(node) {
                visitor(BfsEvent::EdgeFound(edge));
                for neighbor in hg.incident_nodes_unchecked(edge) {
                    let is_visited = node_map.get(neighbor).unwrap_or(&BfsState::Unvisited);
                    if *is_visited == BfsState::Unvisited {
                        queue.push_back(neighbor);
                        node_map.set(neighbor, BfsState::Processing)?;
                    }
                }
            }
        }
    }
    Ok(())
}
