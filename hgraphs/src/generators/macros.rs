use std::collections::TryReserveError;
use std::error::Error;
use std::fmt;

/// An error encountered while constructing a hypergraph with [`crate::try_create_hypergraph!`].
#[derive(Debug)]
pub enum HypergraphBuildError {
    Allocation(TryReserveError),
    MissingNodeIdentifier,
    UnexpectedNodeIdentifier,
    MissingHyperedgeIdentifier,
    UnexpectedHyperedgeIdentifier,
    DuplicateNodeName(&'static str),
    DuplicateHyperedgeName(&'static str),
    IncidenceRejected,
}

impl fmt::Display for HypergraphBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation(error) => error.fmt(formatter),
            Self::MissingNodeIdentifier => {
                formatter.write_str("hypergraph returned fewer node identifiers than requested")
            }
            Self::UnexpectedNodeIdentifier => {
                formatter.write_str("hypergraph returned more node identifiers than requested")
            }
            Self::MissingHyperedgeIdentifier => formatter
                .write_str("hypergraph returned fewer hyperedge identifiers than requested"),
            Self::UnexpectedHyperedgeIdentifier => {
                formatter.write_str("hypergraph returned more hyperedge identifiers than requested")
            }
            Self::DuplicateNodeName(name) => {
                write!(formatter, "node name `{name}` appears more than once")
            }
            Self::DuplicateHyperedgeName(name) => {
                write!(formatter, "hyperedge name `{name}` appears more than once")
            }
            Self::IncidenceRejected => {
                formatter.write_str("hypergraph rejected an incidence between fresh identifiers")
            }
        }
    }
}

impl Error for HypergraphBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Allocation(error) => Some(error),
            Self::MissingNodeIdentifier
            | Self::UnexpectedNodeIdentifier
            | Self::MissingHyperedgeIdentifier
            | Self::UnexpectedHyperedgeIdentifier
            | Self::DuplicateNodeName(_)
            | Self::DuplicateHyperedgeName(_)
            | Self::IncidenceRejected => None,
        }
    }
}

impl From<TryReserveError> for HypergraphBuildError {
    fn from(error: TryReserveError) -> Self {
        Self::Allocation(error)
    }
}

/// Constructs a hypergraph and maps its declared names to fresh identifiers.
///
/// # Examples
///
/// ```
/// let (hypergraph, nodes_map, hyperedges_map) = try_create_hypergraph!(
///     IncidenceMatrix {
///         V = { alice, bob, carol, diana, eric },
///         HE = {
///             project_alpha = { alice, bob, carol },
///             review_group = { alice, carol, diana, eric },
///         },
///     }
/// )?;
/// # Ok::<(), hgraphs::HypergraphBuildError>(())
/// ```
/// `V = { ... }` declares the node set. `HE = { ... }` declares named
/// hyperedges, with each inner set containing the nodes incident to that
/// hyperedge. `IncidenceMatrix` is just an example, any hypergraph type that
/// implements traits for construction can be used.
///
/// The macro returns a `Result` type with a tuple containing hypergraph, nodes_map, and hyperedges_map.
/// The hash maps are from `'static str` to `NodeIndex`/`HyperEdgeIndex`, and the error type is [`HypergraphBuildError`].
///
#[macro_export]
macro_rules! try_create_hypergraph {
    // part of the recursive counting trick
    (@unit $_name:ident) => {
        ()
    };
    // trick to recursively count the number of nodes/hyperedges,
    // hopefully the rust team will make this simpler in a new version
    (@count $($name:ident),* $(,)?) => {
        <[()]>::len(&[$($crate::try_create_hypergraph!(@unit $name)),*])
    };

    (
        $storage:ty {
            V = { $($node:ident),* $(,)? },
            HE = {
                $($hyperedge:ident = { $($incident_node:ident),* $(,)? }),* $(,)?
            }
            $(,)?
        }
    ) => {{
        // A immeditly invoked closure is used so `?` can be used to propagate errors.
        (|| -> ::core::result::Result<_, $crate::HypergraphBuildError> {
            // Count the number of nodes and hyperedges.
            let node_count = $crate::try_create_hypergraph!(@count $($node),*);
            let hyperedge_count = $crate::try_create_hypergraph!(@count $($hyperedge),*);
            let mut hypergraph = <$storage as ::core::default::Default>::default();

            // Reserve space for the maps variables names to nodes and hyperedges.
            let mut nodes_map: ::std::collections::HashMap<
                &'static str,
                $crate::core::NodeIndex<
                    <$storage as $crate::core::HyperGraph>::RawNodeId,
                >,
            > = ::std::collections::HashMap::new();
            nodes_map.try_reserve(node_count)?;

            let mut hyperedges_map: ::std::collections::HashMap<
                &'static str,
                $crate::core::HyperEdgeIndex<
                    <$storage as $crate::core::HyperGraph>::RawEdgeId,
                >,
            > = ::std::collections::HashMap::new();
            hyperedges_map.try_reserve(hyperedge_count)?;
            // populate the nodes map and hyperedges map with the node and hyperedge identifiers.
            // maybe unused for an empty hypergraph
            let mut _node_identifiers =
                $crate::core::ExtendableHyperGraph::try_add_nodes(
                    &mut hypergraph,
                    node_count,
                )?;
            $(
                let $node = _node_identifiers
                    .next()
                    .ok_or($crate::HypergraphBuildError::MissingNodeIdentifier)?;
                if nodes_map.insert(::core::stringify!($node), $node).is_some() {
                    return Err($crate::HypergraphBuildError::DuplicateNodeName(
                        ::core::stringify!($node),
                    ));
                }
            )*
            //maybe unused for an empty hypergraph
            let mut _hyperedge_identifiers =
                $crate::core::ExtendableHyperGraph::try_add_hyperedges(
                    &mut hypergraph,
                    hyperedge_count,
                )?;
            $(
                let $hyperedge = _hyperedge_identifiers
                    .next()
                    .ok_or($crate::HypergraphBuildError::MissingHyperedgeIdentifier)?;
                if hyperedges_map
                    .insert(::core::stringify!($hyperedge), $hyperedge)
                    .is_some()
                {
                    return Err($crate::HypergraphBuildError::DuplicateHyperedgeName(
                        ::core::stringify!($hyperedge),
                    ));
                }
            )*
            // add incidence for each node in each hyperedge, kind of a double for loop
            $($(
                $crate::core::MutableIncidenceHyperGraph::add_incidence(
                    &mut hypergraph,
                    $incident_node,
                    $hyperedge,
                )
                .ok_or($crate::HypergraphBuildError::IncidenceRejected)?;
            )*)*
            Ok((hypergraph, nodes_map, hyperedges_map))
        })()
    }};
}

#[cfg(test)]
mod tests {
    use crate::core::{
        HyperEdgeCountable, HyperEdgeIndex, NodeCountable, NodeIncidence, NodeIndex,
    };
    use crate::data_structures::IncidenceMatrix;
    use std::collections::{HashMap, HashSet};
    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn constructs_named_identifiers_and_incidences() -> TestResult {
        let (hypergraph, nodes, hyperedges) = try_create_hypergraph!(
            IncidenceMatrix {
                V = { alice, bob, carol, diana, eric },
                HE = {
                    project_alpha = { alice, bob, carol },
                    project_beta = { bob, diana, eric },
                    review_group = { alice, carol, diana, eric },
                },
            }
        )?;

        assert_eq!(hypergraph.num_nodes(), 5);
        assert_eq!(hypergraph.num_hyperedges(), 3);
        assert_eq!(
            nodes,
            HashMap::from([
                ("alice", NodeIndex(0)),
                ("bob", NodeIndex(1)),
                ("carol", NodeIndex(2)),
                ("diana", NodeIndex(3)),
                ("eric", NodeIndex(4)),
            ])
        );
        assert_eq!(
            hyperedges,
            HashMap::from([
                ("project_alpha", HyperEdgeIndex(0)),
                ("project_beta", HyperEdgeIndex(1)),
                ("review_group", HyperEdgeIndex(2)),
            ])
        );

        let project_alpha = hyperedges
            .get("project_alpha")
            .copied()
            .ok_or("project_alpha")?;
        let incident_nodes = hypergraph
            .incident_nodes(project_alpha)
            .ok_or("project_alpha")?
            .collect::<HashSet<_>>();
        assert_eq!(
            incident_nodes,
            HashSet::from([NodeIndex(0), NodeIndex(1), NodeIndex(2)])
        );
        Ok(())
    }

    #[test]
    fn supports_empty_sets() -> TestResult {
        let (hypergraph, nodes, hyperedges) =
            try_create_hypergraph!(IncidenceMatrix { V = {}, HE = {} })?;

        assert_eq!(hypergraph.num_nodes(), 0);
        assert_eq!(hypergraph.num_hyperedges(), 0);
        assert!(nodes.is_empty());
        assert!(hyperedges.is_empty());
        Ok(())
    }

    #[test]
    fn supports_qualified_storage_types() -> TestResult {
        let (hypergraph, nodes, hyperedges) = try_create_hypergraph!(
            crate::data_structures::IncidenceMatrix {
                V = { a, b, c },
                HE = { e = { a, b, c } },
            }
        )?;

        assert_eq!(hypergraph.num_nodes(), 3);
        assert_eq!(hypergraph.num_hyperedges(), 1);
        assert_eq!(nodes.get("a"), Some(&NodeIndex(0)));
        assert_eq!(hyperedges.get("e"), Some(&HyperEdgeIndex(0)));
        Ok(())
    }

    #[test]
    fn rejects_duplicate_node_names() {
        let result = try_create_hypergraph!(IncidenceMatrix {
            V = { a, a },
            HE = {},
        });

        assert!(matches!(
            result,
            Err(crate::HypergraphBuildError::DuplicateNodeName("a"))
        ));
    }

    #[test]
    fn rejects_duplicate_hyperedge_names() {
        let result = try_create_hypergraph!(
            IncidenceMatrix {
                V = { a, b, c },
                HE = {
                    e = { a, b, c },
                    e = { a, b, c },
                },
            }
        );

        assert!(matches!(
            result,
            Err(crate::HypergraphBuildError::DuplicateHyperedgeName("e"))
        ));
    }
}
