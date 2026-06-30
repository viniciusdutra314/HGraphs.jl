use std::{collections::TryReserveError, hash::Hash};

pub trait NodeCountable {
    fn num_nodes(&self) -> usize;
}

pub trait HyperEdgeCountable {
    fn num_hyperedges(&self) -> usize;
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

pub trait HyperEdgeIncidence: HyperGraph {
    fn incident_edges(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>>>;

    unsafe fn incident_edges_unchecked(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> {
        unsafe { self.incident_edges(node).unwrap_unchecked() }
    }
}

pub trait NodeIncidence: HyperGraph {
    fn incident_nodes(
        &self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>>>;

    unsafe fn incident_nodes_unchecked(
        &self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> impl Iterator<Item = NodeIndex<Self::RawNodeId>> {
        unsafe { self.incident_nodes(edge).unwrap_unchecked() }
    }
}

pub trait ContiguousHyperEdgeIncidence: HyperGraph {
    fn incident_edges_slice(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<&[HyperEdgeIndex<Self::RawEdgeId>]>;
    unsafe fn incident_edges_slice_unchecked(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> &[HyperEdgeIndex<Self::RawEdgeId>] {
        unsafe { self.incident_edges_slice(node).unwrap_unchecked() }
    }
}

pub trait ContiguousNodeIncidence: HyperGraph {
    fn incident_nodes_slice(
        &self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<&[NodeIndex<Self::RawNodeId>]>;

    unsafe fn incident_nodes_slice_unchecked(
        &self,
        edge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> &[NodeIndex<Self::RawNodeId>] {
        unsafe { self.incident_nodes_slice(edge).unwrap_unchecked() }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Capacity {
    pub num_nodes: Option<usize>,
    pub num_hyperedges: Option<usize>,
}

pub trait AllocatableHyperGraph: HyperGraph {
    fn try_reserve_exact(&mut self, additional: Capacity) -> Result<(), TryReserveError>;

    fn try_with_capacity(capacity: Capacity) -> Result<Self, TryReserveError>
    where
        Self: Default,
    {
        let mut graph = Self::default();
        graph.try_reserve_exact(capacity)?;
        Ok(graph)
    }
}

pub trait ExtendableHyperGraph: HyperGraph {
    fn try_add_nodes<'a>(
        &mut self,
        num_nodes: usize,
    ) -> Result<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a, TryReserveError>;

    fn try_add_hyperedges<'a>(
        &mut self,
        num_hyperedges: usize,
    ) -> Result<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a, TryReserveError>;
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
