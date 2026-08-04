use hgraphs::{
    AllocatableHyperGraph, ExtendableHyperGraph, HyperEdgeCountable, MutableIncidenceHyperGraph,
    NodeCountable, projections::cliques::hypergraph_from_cliques, storage::IncidenceMatrix,
};
use petgraph::{Undirected, graphmap::GraphMap};
use polars::prelude::*;
use std::{
    collections::{HashMap, hash_map::RandomState},
    error::Error,
    io::{Error as IoError, ErrorKind},
    path::{Path, PathBuf},
};

type TestResult = Result<(), Box<dyn Error>>;
type ProximityGraph = GraphMap<i64, (), Undirected, RandomState>;
type TemporalNetwork<H> = HashMap<i64, H>;

fn invalid_data() -> IoError {
    ErrorKind::InvalidData.into()
}

fn create_copenhagen_temporal_network<H>(
    dataset_path: &Path,
) -> Result<TemporalNetwork<H>, Box<dyn Error>>
where
    H: AllocatableHyperGraph + ExtendableHyperGraph + Default + MutableIncidenceHyperGraph,
{
    let interactions = CsvReadOptions::default()
        .with_projection(Some(vec![0, 1, 2].into()))
        .try_into_reader_with_file_path(Some(dataset_path.to_path_buf()))?
        .finish()?;

    let timestamps = interactions
        .select_at_idx(0)
        .ok_or_else(invalid_data)?
        .i64()?;
    let users_a = interactions
        .select_at_idx(1)
        .ok_or_else(invalid_data)?
        .i64()?;
    let users_b = interactions
        .select_at_idx(2)
        .ok_or_else(invalid_data)?
        .i64()?;

    let mut proximity_graphs = HashMap::<i64, ProximityGraph>::new();
    for ((timestamp, user_a), user_b) in timestamps.iter().zip(users_a.iter()).zip(users_b.iter()) {
        let timestamp = timestamp.ok_or_else(invalid_data)?;
        let user_a = user_a.ok_or_else(invalid_data)?;
        let user_b = user_b.ok_or_else(invalid_data)?;
        if user_b == -1 || user_b == -2 {
            continue;
        }

        let _ = proximity_graphs
            .entry(timestamp)
            .or_default()
            .add_edge(user_a, user_b, ());
    }

    let mut temporal_network = TemporalNetwork::new();
    temporal_network.try_reserve(proximity_graphs.len())?;
    for (timestamp, proximity_graph) in proximity_graphs {
        let hypergraph = hypergraph_from_cliques(&proximity_graph)?;
        let _ = temporal_network.insert(timestamp, hypergraph);
    }

    Ok(temporal_network)
}

fn full_copenhagen_suite<H>() -> TestResult
where
    H: AllocatableHyperGraph
        + ExtendableHyperGraph
        + Default
        + MutableIncidenceHyperGraph
        + NodeCountable
        + HyperEdgeCountable,
{
    let dataset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/datasets/raw_files/copenhagen/bt_symmetric.csv");
    let temporal_network = create_copenhagen_temporal_network::<H>(&dataset_path)?;

    assert!(!temporal_network.is_empty());
    assert!(temporal_network.values().all(|graph| graph.num_nodes() > 0));
    assert!(
        temporal_network
            .values()
            .all(|graph| graph.num_hyperedges() > 0)
    );
    Ok(())
}

#[test]
fn incidence_matrix_loads_copenhagen() -> TestResult {
    full_copenhagen_suite::<IncidenceMatrix>()
}
