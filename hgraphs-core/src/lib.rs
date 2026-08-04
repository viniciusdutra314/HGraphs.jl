#![no_std]

//! Standardized generic traits for the hypergraph ecosystem, similar to the [petgraph](https://docs.rs/petgraph/latest/petgraph/)
//! traits but extended to hypergraphs.
//!
//! The traits in this crate describe granular capabilities so that algorithms can
//! operate on generic hypergraph data structures without code change.
//! It can be used in `#![no_std]` environments for embedded or resource-constrained applications (with optional `alloc` support).
//!
//! This crate was created to be used by the [`hgraphs`](https://github.com/HGraphs-jl/HGraphs.jl) library, which is
//! the recommend crate for end-users. But if you reading this chances are that you want to build your own hypergraph
//! data structure or algorithm.
//!
//! # Features
//!
//! - `alloc` enables traits that require allocation, such as [AllocatableHyperGraph],
//!   and is enabled by default.
//!
//! # Complexity model
//!
//! Each trait operation documents a **baseline complexity** for its running
//! time and memory use. Generic algorithms use these operation-level bounds to
//! derive their **reference complexity**. A reference complexity therefore
//! applies when the concrete implementations of all required trait operations
//! meet their documented baseline complexities.
//!
//! Documentation uses `n` for the number of nodes, `m` for the number of
//! hyperedges, `d(v)` for the number of hyperedges incident to node `v`, and
//! `|e|` for the number of nodes incident to hyperedge `e`.
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::collections::TryReserveError;
use core::hash::Hash;

/// Provides the number of nodes in the hypergraph.
pub trait NodeCountable {
    /// Returns the number of nodes in the hypergraph.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    fn num_nodes(&self) -> usize;
}

/// Provides the number of hyperedges in the hypergraph.
pub trait HyperEdgeCountable {
    /// Returns the number of hyperedges in the hypergraph.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    fn num_hyperedges(&self) -> usize;
}

/// Provides iteration over all hyperedges in a hypergraph.
pub trait HyperEdgeList: HyperGraph {
    /// Returns every hyperedge identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time to create the iterator and `O(m)` time to exhaust
    /// it. Auxiliary space is `O(1)` beyond implementation-defined iterator
    /// state.
    fn hyperedge_identifiers(&self) -> impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>>;
}

/// Provides iteration over all nodes in a hypergraph.
pub trait NodeList: HyperGraph {
    /// Returns every currently valid node identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time to create the iterator and `O(n)` time to exhaust
    /// it. Auxiliary space is `O(1)` beyond implementation-defined iterator
    /// state.
    fn node_identifiers(&self) -> impl Iterator<Item = NodeIndex<Self::RawNodeId>>;
}

/// Marker implemented by compile-time directedness categories.
pub trait Directedness {}

/// Selects directed hypergraph semantics.
pub struct Directed;
impl Directedness for Directed {}

/// Selects undirected hypergraph semantics.
pub struct Undirected;
impl Directedness for Undirected {}

/// A strongly typed node identifier.
///
/// The wrapped value is meaningful only to the hypergraph that produced it. This
/// type does not imply that identifiers are contiguous or stable after mutation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeIndex<T>(
    /// The data-structure-defined identifier type.
    pub T,
);

/// A strongly typed hyperedge identifier.
///
/// The wrapped value is meaningful only to the hypergraph that produced it. This
/// type does not imply that identifiers are contiguous or stable after mutation.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HyperEdgeIndex<T>(
    /// The data-structure-defined identifier type.
    pub T,
);

/// Base trait for all hypergraphs, it also defines the identifier and directedness types.
pub trait HyperGraph {
    /// The underlying identifier type wrapped by [`NodeIndex`].
    type RawNodeId: Copy + Eq + Hash;

    /// The underlying identifier type wrapped by [`HyperEdgeIndex`].
    type RawEdgeId: Copy + Eq + Hash;

    /// Whether the hypergraph is directed or undirected.
    type Directedness: Directedness;
}

/// Provides the hyperedges incident to a node.
pub trait HyperEdgeIncidence: HyperGraph {
    /// Returns the hyperedges incident to `node`.
    ///
    /// Returns `None` when `node` is not currently valid. For a valid node, the
    /// iterator yields each incident hyperedge identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time to create the iterator and `O(d(node))` time to
    /// exhaust it. Auxiliary space is `O(1)` beyond implementation-defined
    /// iterator state.
    fn incident_edges(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>>>;

    /// Returns the hyperedges incident to `node` without validating it.
    ///
    /// This has the same iteration semantics as [`Self::incident_edges`].
    ///
    /// # Baseline complexity
    ///
    /// The same as [`Self::incident_edges`].
    /// # Safety
    ///
    /// `node` must be a currently valid identifier for `self`.
    unsafe fn incident_edges_unchecked(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> {
        unsafe { self.incident_edges(node).unwrap_unchecked() }
    }
}

/// Provides the nodes incident to a hyperedge.
pub trait NodeIncidence: HyperGraph {
    /// Returns the nodes incident to `hyperedge`.
    ///
    /// Returns `None` when `hyperedge` is not currently valid. For a valid
    /// hyperedge, the iterator yields each incident node identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time to create the iterator and `O(|e|)` time to
    /// exhaust it. Auxiliary space is `O(1)` beyond implementation-defined
    /// iterator state.
    fn incident_nodes(
        &self,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<impl Iterator<Item = NodeIndex<Self::RawNodeId>>>;

    /// Returns the nodes incident to `hyperedge` without validating it.
    ///
    /// This has the same iteration semantics as [`Self::incident_nodes`].
    ///
    /// # Baseline complexity
    ///
    /// The same as [`Self::incident_nodes`].
    ///
    /// # Safety
    ///
    /// `hyperedge` must be a currently valid identifier for `self`.
    unsafe fn incident_nodes_unchecked(
        &self,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> impl Iterator<Item = NodeIndex<Self::RawNodeId>> {
        unsafe { self.incident_nodes(hyperedge).unwrap_unchecked() }
    }
}

/// Provides node-to-hyperedge incidences as borrowed contiguous slices.
///
/// The returned slice is a view of hypergraph-owned memory.
pub trait ContiguousHyperEdgeIncidence: HyperGraph {
    /// Returns a contiguous slice of the hyperedges incident to `node`.
    ///
    /// Returns `None` when `node` is not currently valid. For a valid node, the
    /// slice contains each incident hyperedge identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    fn incident_edges_slice(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> Option<&[HyperEdgeIndex<Self::RawEdgeId>]>;

    /// Returns a contiguous incidence slice without validating `node`.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    ///
    /// # Safety
    ///
    /// `node` must be a currently valid identifier for `self`.
    unsafe fn incident_edges_slice_unchecked(
        &self,
        node: NodeIndex<Self::RawNodeId>,
    ) -> &[HyperEdgeIndex<Self::RawEdgeId>] {
        unsafe { self.incident_edges_slice(node).unwrap_unchecked() }
    }
}

/// Provides hyperedge-to-node incidences as borrowed contiguous slices.
///
/// The returned slice is a view of hypergraph-owned memory.
pub trait ContiguousNodeIncidence: HyperGraph {
    /// Returns a contiguous slice of the nodes incident to `hyperedge`.
    ///
    /// Returns `None` when `hyperedge` is not currently valid. For a valid
    /// hyperedge, the slice contains each incident node identifier exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    fn incident_nodes_slice(
        &self,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<&[NodeIndex<Self::RawNodeId>]>;

    /// Returns a contiguous incidence slice without validating `hyperedge`.
    ///
    /// # Baseline complexity
    ///
    /// `O(1)` time and `O(1)` auxiliary space.
    ///
    /// # Safety
    ///
    /// `hyperedge` must be a currently valid identifier for `self`.
    unsafe fn incident_nodes_slice_unchecked(
        &self,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> &[NodeIndex<Self::RawNodeId>] {
        unsafe { self.incident_nodes_slice(hyperedge).unwrap_unchecked() }
    }
}

/// Additional capacity for a hypergraph data structure.
///
/// This struct is intended to be used with the methods of the trait
/// [`AllocatableHyperGraph`].
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Capacity {
    /// Additional node capacity to request.
    pub num_nodes: Option<usize>,

    /// Additional hyperedge capacity to request.
    pub num_hyperedges: Option<usize>,
}

/// Provides fallible reservation of memory for a hypergraph data structure.
///
/// Reservation changes capacity only; it must not add elements, remove
/// elements, change incidences, or invalidate identifiers.
#[cfg(feature = "alloc")]
pub trait AllocatableHyperGraph: HyperGraph {
    /// Tries to reserve capacity for at least `additional` more elements,
    /// returns an error if a requested allocation failed.
    ///
    /// # Baseline complexity
    ///
    /// TODO!
    fn try_reserve_exact(&mut self, additional: Capacity) -> Result<(), TryReserveError>;

    fn try_with_capacity(capacity: Capacity) -> Result<Self, TryReserveError>
    where
        Self: Default,
    {
        let mut hypergraph = Self::default();
        hypergraph.try_reserve_exact(capacity)?;
        Ok(hypergraph)
    }
}

/// Provides fallible insertion of nodes and hyperedges.
///
/// Successful insertion returns the identifiers created by that operation.
#[cfg(feature = "alloc")]
pub trait ExtendableHyperGraph: HyperGraph {
    /// Tries to add `num_nodes` isolated nodes.
    ///
    /// On success, the iterator yields every newly created node identifier
    /// exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(num_nodes)` total time and `O(num_nodes)` retained space.
    /// Auxiliary space is `O(1)` beyond implementation-defined iterator state.
    fn try_add_nodes<'a>(
        &mut self,
        num_nodes: usize,
    ) -> Result<impl Iterator<Item = NodeIndex<Self::RawNodeId>> + 'a, TryReserveError>;

    /// Tries to add `num_hyperedges` empty hyperedges.
    ///
    /// On success, the iterator yields every newly created hyperedge identifier
    /// exactly once.
    ///
    /// # Baseline complexity
    ///
    /// `O(num_hyperedges)` total time and `O(num_hyperedges)` retained
    /// space. Auxiliary space is `O(1)` beyond implementation-defined iterator
    /// state.
    fn try_add_hyperedges<'a>(
        &mut self,
        num_hyperedges: usize,
    ) -> Result<impl Iterator<Item = HyperEdgeIndex<Self::RawEdgeId>> + 'a, TryReserveError>;
}

/// Provides mutation of the node-hyperedge incidence relation.
///
/// Both operations are idempotent: adding an existing incidence or removing an
/// absent incidence succeeds without changing the hypergraph. Implementations
/// must keep node-to-hyperedge and hyperedge-to-node views consistent.
pub trait MutableIncidenceHyperGraph: HyperGraph {
    /// Ensures that `node` is incident to `hyperedge`.
    ///
    /// Returns `None` if either identifier is invalid. In that case, the
    /// hypergraph must remain logically unchanged. Returns `Some(())` after the
    /// incidence exists, whether it was newly inserted or already present.
    ///
    /// # Baseline complexity
    ///
    /// TODO
    fn add_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()>;

    /// Ensures that `node` is not incident to `hyperedge`.
    ///
    /// Returns `None` if either identifier is invalid. In that case, the
    /// hypergraph must remain logically unchanged. Returns `Some(())` after the
    /// incidence is absent, whether it was removed or already absent.
    ///
    /// # Baseline complexity
    ///
    /// TODO
    fn remove_incidence(
        &mut self,
        node: NodeIndex<Self::RawNodeId>,
        hyperedge: HyperEdgeIndex<Self::RawEdgeId>,
    ) -> Option<()>;
}

/// Defines the key and value types of a property map.
pub trait PropertyMapBase {
    /// The type used to access properties.
    type Key: Clone + PartialEq;

    /// The property value associated with a key.
    type Value;
}

/// Provides checked and unchecked reads from a property map.
pub trait PropertyMapReadable: PropertyMapBase {
    /// Returns the value associated with `key`, or `None` if none is present.
    ///
    /// # Baseline complexity
    ///
    /// O(1) in runtime and O(1) in auxilary space
    fn get(&self, key: Self::Key) -> Option<&Self::Value>;

    /// Returns the value associated with `key` without checking its presence.
    ///
    /// # Baseline complexity
    ///
    /// O(1) in runtime and O(1) in auxilary space
    ///
    /// # Safety
    ///
    /// [`Self::get`] must return `Some` for `key` at the time of this call.
    unsafe fn get_unsafe(&self, key: Self::Key) -> &Self::Value {
        unsafe { self.get(key).unwrap_unchecked() }
    }
}

/// Errors produced by checked property-map mutation.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyMapError {
    /// The key is outside the property map's supported key domain.
    KeyNotFound,
}

/// Provides checked and unchecked mutation of a property map.
pub trait PropertyMapWritable: PropertyMapBase {
    /// Associates `value` with `key`.
    ///
    /// Returns the previous value, or `None` if the valid key had no value.
    /// Returns [`PropertyMapError::KeyNotFound`] when `key` is outside the map's
    /// supported key domain; on error, the map must remain unchanged.
    ///
    /// # Baseline complexity
    ///
    /// TODO!
    fn set(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Result<Option<Self::Value>, PropertyMapError>;

    /// Removes and returns the value associated with `key`.
    ///
    /// Returns `Ok(None)` when the valid key has no value. Returns
    /// [`PropertyMapError::KeyNotFound`] when `key` is outside the map's
    /// supported key domain; on error, the map must remain unchanged.
    ///
    /// # Baseline complexity
    ///
    /// TODO
    fn remove(&mut self, key: Self::Key) -> Result<Option<Self::Value>, PropertyMapError>;

    /// Associates a value with `key` without validating its key domain.
    ///
    /// # Baseline complexity
    ///
    /// The same as [`Self::set`].
    ///
    /// # Safety
    ///
    /// `key` must belong to the property map's supported key domain, so that
    /// [`Self::set`] would return `Ok`.
    unsafe fn set_unsafe(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        unsafe { self.set(key, value).unwrap_unchecked() }
    }

    /// Removes a value without validating `key`.
    ///
    /// # Baseline complexity
    ///
    /// The same as [`Self::remove`].
    ///
    /// # Safety
    ///
    /// `key` must belong to the property map's supported key domain, so that
    /// [`Self::remove`] would return `Ok`.
    unsafe fn remove_unsafe(&mut self, key: Self::Key) -> Option<Self::Value> {
        unsafe { self.remove(key).unwrap_unchecked() }
    }
}

/// A property map supporting both reads and writes.
pub trait PropertyMapReadWrite: PropertyMapReadable + PropertyMapWritable {}

impl<T> PropertyMapReadWrite for T where T: PropertyMapReadable + PropertyMapWritable {}
