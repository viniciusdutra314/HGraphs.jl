use std::collections::HashMap;

use crate::{EdgeCountable, NodeCountable};

struct IncidenceMatrix<V, HE> {
    nodes: HashMap<usize, V>,
    hyperedges: HashMap<usize, HE>,
    matrix: ndarray::Array2<bool>,
}

impl<V,HE> NodeCountable for IncidenceMatrix<V,HE>{
    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

impl<V,HE> EdgeCountable for IncidenceMatrix<V,HE>{
    fn num_edges(&self) -> usize {
        self.hyperedges.len()
    }
}