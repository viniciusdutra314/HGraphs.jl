use std::hash::Hash;

use petgraph::{acyclic::AcyclicEdgeError::SelfLoop, data::Element::Edge};

mod bfs;
mod generators;
mod implementations;
mod io;
mod projections;
pub trait NodeCountable {
    fn num_nodes(&self) -> usize;
}

pub trait EdgeCountable {
    fn num_edges(&self) -> usize;
}

pub trait Directedness {}

pub struct Directed;
impl Directedness for Directed {}

pub struct Undirected;
impl Directedness for Undirected {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeIndex<T>(pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HyperEdgeIndex<T>(pub T);

pub trait HyperGraph {
    type RawNodeId: Copy + Eq + Hash;
    type RawEdgeId: Copy + Eq + Hash;
    type Directedness: Directedness;
}

pub trait IncidenceHyperGraph: HyperGraph {
    fn incident_nodes<'a>(
        &'a self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a>;
    fn incident_edges<'a>(
        &'a self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a>;

    unsafe fn incident_nodes_unchecked<'a>(
        &'a self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a {
        unsafe { self.incident_nodes(edge).unwrap_unchecked() }
    }

    unsafe fn incident_edges_unchecked<'a>(
        &'a self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a {
        unsafe { self.incident_edges(node).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Capacity {
    additional_num_nodes: Option<usize>,
    additional_num_hyperedges: Option<usize>,
}

pub trait AllocatableHyperGraph: HyperGraph {
    fn reserve(&mut self, capacity: Capacity);

    fn with_capacity(capacity: Capacity) -> Self
    where
        Self: Default,
    {
        let mut graph = Self::default();
        graph.reserve(capacity);
        graph
    }
}

pub trait MutableIncidenceHyperGraph: HyperGraph {
    fn add_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()>;
    fn remove_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()>;
}

pub trait PropertyMapBase {
    type Key: Clone + PartialEq;
    type Value;
}

pub trait PropertyMapReadable: PropertyMapBase {
    fn get(&self, key: Self::Key) -> Option<&Self::Value>;
    unsafe fn get_unsafe(&self, key: Self::Key) -> &Self::Value {
        unsafe { self.get(key).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyMapError {
    KeyNotFound,
}

pub trait PropertyMapWritable: PropertyMapBase {
    fn set(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, PropertyMapError>;
    fn remove(&mut self, key: Self::Key) -> Result<Option<Self::Value>, PropertyMapError>;
    unsafe fn set_unsafe(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        unsafe { self.set(key, value).unwrap_unchecked() }
    }
    unsafe fn remove_unsafe(&mut self, key: Self::Key) -> Option<Self::Value> {
        unsafe { self.remove(key).unwrap_unchecked() }
    }
}

pub trait PropertyMapReadWrite: PropertyMapReadable + PropertyMapWritable {}

impl<T> PropertyMapReadWrite for T where T: PropertyMapReadable + PropertyMapWritable {}
