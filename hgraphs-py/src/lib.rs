use pyo3::prelude::*;
macro_rules! expose_hypergraph_trait_method {
    ($py_type:ty,NodeCountable) => {
        use hgraphs::NodeCountable;
        fn num_nodes(&self) -> usize {
            self.data.num_nodes()
        }
    };
    ($py_type:ty,HyperEdgeCountable) => {
        use hgraphs::HyperEdgeCountable;
        fn num_hyperedges(&self) -> usize {
            self.data.num_hyperedges()
        }
    };
}

macro_rules! expose_hypergraph_to_python {
    ($py_type:ty, [ $( $trait_flag:ident ),* $(,)? ]) => {
        #[pyo3::pymethods]
        impl $py_type {
            #[new]
            fn new() -> Self {
                Self::default()
            }

            *$(expose_hypergraph_trait_method!($py_type, $trait_flag);)*
        }

    };
}

#[pymodule]
mod hgraphs_py {
    use pyo3::prelude::*;

    #[pyclass(name = "IncidenceMatrix")]
    #[derive(Default)]
    struct PyIncidenceMatrix {
        data: hgraphs::storage::IncidenceMatrix,
    }
    expose_hypergraph_to_python!(PyIncidenceMatrix, [NodeCountable, HyperEdgeCountable]);
}
