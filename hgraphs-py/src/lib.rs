use pyo3::prelude::*;

#[pymodule]
mod hgraphs_py {
    use hgraphs::{HyperEdgeCountable, NodeCountable};
    use pyo3::prelude::*;

    #[pyclass(name = "IncidenceMatrix")]
    #[derive(Default)]
    struct PyIncidenceMatrix {
        data: hgraphs::storage::IncidenceMatrix,
    }
    #[pymethods]
    impl PyIncidenceMatrix {
        #[new]
        fn new() -> Self {
            Self::default()
        }
        
        fn num_nodes(&self) -> usize {
            self.data.num_nodes()
        }
        pub fn num_hyperedges(&self) -> usize {
            self.data.num_hyperedges()
        }
    }
}
