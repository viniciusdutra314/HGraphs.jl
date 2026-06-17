mod projections;
mod views;
pub trait NodeCountable {
    fn num_nodes(&self) -> usize;
}

pub trait EdgeCountable {
    fn num_edges(&self) -> usize;
}

pub trait NeighborsOut {
    fn neighbors_out(&self, a: usize) -> impl Iterator<Item = usize>;
}
