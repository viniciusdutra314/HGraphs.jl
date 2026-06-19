use petgraph::visit::{
    GraphBase, GraphRef, IntoNeighbors, IntoNodeIdentifiers, VisitMap, Visitable,
};

pub struct BipartiteGraph<G: GraphBase> {
    inner_graph: G,
}

impl<G> BipartiteGraph<G>
where
    G: Visitable,
    for<'a> &'a G: IntoNeighbors<NodeId = G::NodeId> + IntoNodeIdentifiers,
{
    fn try_from(value: G) -> Result<Self, &'static str> {
        if is_bipartite_undirected(&value) {
            Ok(BipartiteGraph { inner_graph: value })
        } else {
            Err("Graph is not bipartited")
        }
    }
    pub unsafe fn new_unchecked(value: G) -> Self {
        BipartiteGraph { inner_graph: value }
    }
}

// TODO: the implementation is suboptimal
pub fn is_bipartite_undirected<G, N, VM>(g: G) -> bool
where
    G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers,
    N: Copy,
    VM: VisitMap<N>,
{
    #[derive(Copy, Clone)]
    enum Color {
        Red,
        Blue,
    }

    let mut red = g.visit_map();
    let mut blue = g.visit_map();

    let mut dfs_stack = Vec::new();

    for start in g.node_identifiers() {
        if red.is_visited(&start) || blue.is_visited(&start) {
            continue;
        }
        red.visit(start);
        dfs_stack.push((start, Color::Red));

        while let Some((node, color)) = dfs_stack.pop() {
            let (same_color, other_color, next_color) = match color {
                Color::Red => (&red, &mut blue, Color::Blue),
                Color::Blue => (&blue, &mut red, Color::Red),
            };

            for neighbour in g.neighbors(node) {
                if same_color.is_visited(&neighbour) {
                    return false;
                }
                if other_color.visit(neighbour) {
                    dfs_stack.push((neighbour, next_color));
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::projections::BipartiteGraph;
    use rustworkx_core::petgraph::prelude::*;
    #[test]
    fn try_from_accepts_a_bipartite_graph() {
        use rustworkx_core::generators::random_bipartite_graph;
        let g: DiGraph<(), ()> = random_bipartite_graph(20, 20, 0.3, None, || (), || ()).unwrap();
        assert!(BipartiteGraph::try_from(g).is_ok());
    }

    #[test]
    fn try_from_rejects_a_non_bipartite_graph() {
        use rustworkx_core::generators::complete_graph;
        let g1: DiGraph<(), ()> = complete_graph(Some(10), None, || (), || ()).unwrap();
        let g2 = g1.clone();
        assert!(BipartiteGraph::try_from(g1).is_err());
        let _fake_bipartite = unsafe { BipartiteGraph::new_unchecked(g2) };
    }
}
