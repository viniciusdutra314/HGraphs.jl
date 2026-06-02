use hgraphs_core::NeighborsOut;

struct CollatzConjecture;

impl hgraphs_core::NeighborsOut for CollatzConjecture {
    fn neighbors_out(&self, a: usize) -> impl Iterator<Item = usize> {
        if a % 2 == 0 {
            std::iter::once(a / 2)
        } else {
            std::iter::once(3 * a + 1)
        }
    }
}

fn main() {
    let graph = CollatzConjecture;
    let start_node = 12;
    let mut node_index = start_node;
    while node_index != 1 {
        println!("{}", node_index);
        node_index = graph.neighbors_out(node_index).nth(0).unwrap();
    }
}
