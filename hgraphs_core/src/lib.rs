mod bfs;
mod implementations;
mod io;
mod projections;
pub trait NodeCountable {
    fn num_nodes(&self) -> usize;
}

pub trait EdgeCountable {
    fn num_edges(&self) -> usize;
}

trait Directedness {}

struct Directed;
impl Directedness for Directed {}

struct Undirected;
impl Directedness for Undirected {}

pub trait HyperGraph {
    type HyperEdgeId: Copy + PartialEq;
    type NodeId: Copy + PartialEq;
    type Directedness: Directedness;
}

pub trait IncidenceHyperGraph: HyperGraph {
    type EdgeNodeIter: Iterator<Item = Self::NodeId>;
    type NodeEdgeIter: Iterator<Item = Self::HyperEdgeId>;
    fn nodes_of_edge(&self, edge: Self::HyperEdgeId) -> Self::EdgeNodeIter;
    fn edges_of_node(&self, node: Self::NodeId) -> Self::NodeEdgeIter;
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
