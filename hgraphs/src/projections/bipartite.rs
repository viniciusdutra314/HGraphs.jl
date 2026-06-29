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
    pub fn try_from(value: G) -> Result<Self, &'static str> {
        if is_bipartite(&value) {
            Ok(BipartiteGraph { inner_graph: value })
        } else {
            Err("Graph is not bipartited")
        }
    }
    pub unsafe fn new_unchecked(value: G) -> Self {
        BipartiteGraph { inner_graph: value }
    }
}

#[derive(Copy, Clone)]
pub enum BipartiteColor {
    ColorA,
    ColorB,
}
pub enum BipartiteEvent<N> {
    NodeColored { node: N, color: BipartiteColor },
    Conflict { left: N, right: N },
}

pub fn is_bipartite<G, N, VM>(g: G) -> bool
where
    G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers,
    N: Copy,
    VM: VisitMap<N>,
{
    is_bipartite_with(g, |_| ())
}

// TODO: the implementation is suboptimal
pub fn is_bipartite_with<G, N, VM, F>(g: G, mut visitor: F) -> bool
where
    G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N> + IntoNodeIdentifiers,
    N: Copy,
    VM: VisitMap<N>,
    F: FnMut(BipartiteEvent<N>),
{
    let mut color_a = g.visit_map();
    let mut color_b = g.visit_map();
    let mut dfs_stack = Vec::new();

    for start in g.node_identifiers() {
        if color_a.is_visited(&start) || color_b.is_visited(&start) {
            continue;
        }
        color_a.visit(start);
        dfs_stack.push((start, BipartiteColor::ColorA));
        visitor(BipartiteEvent::NodeColored {
            node: start,
            color: BipartiteColor::ColorA,
        });

        while let Some((node, color)) = dfs_stack.pop() {
            let (same_color, other_color, next_color) = match color {
                BipartiteColor::ColorA => (&color_a, &mut color_b, BipartiteColor::ColorB),
                BipartiteColor::ColorB => (&color_b, &mut color_a, BipartiteColor::ColorA),
            };

            for neighbour in g.neighbors(node) {
                if same_color.is_visited(&neighbour) {
                    visitor(BipartiteEvent::Conflict {
                        left: node,
                        right: neighbour,
                    });
                    return false;
                }
                if other_color.visit(neighbour) {
                    visitor(BipartiteEvent::NodeColored {
                        node: neighbour,
                        color: next_color,
                    });
                    dfs_stack.push((neighbour, next_color));
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{BipartiteEvent, BipartiteGraph, is_bipartite_with};
    use rustworkx_core::{generators::complete_graph, petgraph::prelude::*};

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

    #[test]
    fn is_bipartite_with_counts_colored_nodes_on_a_path() {
        let mut g: UnGraph<(), ()> = UnGraph::new_undirected();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        g.add_edge(a, b, ());
        g.add_edge(b, c, ());

        let mut colored_nodes = 0;
        let is_bipartite = is_bipartite_with(&g, |event| {
            if let BipartiteEvent::NodeColored { .. } = event {
                colored_nodes += 1;
            }
        });

        assert!(is_bipartite);
        assert_eq!(colored_nodes, 3);
    }

    #[test]
    fn is_bipartite_with_reports_a_conflict() {
        let g: DiGraph<(), ()> = complete_graph(Some(10), None, || (), || ()).unwrap();
        let mut conflicts = 0;
        let is_bipartite = is_bipartite_with(&g, |event| {
            if let BipartiteEvent::Conflict { .. } = event {
                conflicts += 1;
            }
        });

        assert!(!is_bipartite);
        assert_eq!(conflicts, 1);
    }
}
