mod implementations;
mod io;
mod projections;
mod views;
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

pub trait PropertyMap {
    type Key;
    type Value;
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    unsafe fn get_unsafe(&self, key: &Self::Key) -> &Self::Value;
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyMapError {
    KeyNotFound,
}

pub trait MutablePropertyMap: PropertyMap {
    fn set(
        &mut self,
        key: &Self::Key,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, PropertyMapError>;

    unsafe fn set_unsafe(&mut self, key: &Self::Key, value: Self::Value) -> Option<Self::Value>;
    fn remove(&mut self, key: &Self::Key) -> Result<Option<Self::Value>, PropertyMapError>;
    unsafe fn remove_unsafe(&mut self, key: &Self::Key) -> Option<Self::Value>;
}
