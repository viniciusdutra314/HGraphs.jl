use petgraph::visit::{GraphRef, IntoNeighbors, VisitMap, Visitable};

// struct BipartiteGraph<G> {
//     inner_graph: G,
// }

// impl<G> BipartiteGraph<G>
// where
//     G: GraphRef + IntoNeighbors + Visitable,
// {
//     fn try_from(value: G) -> Result<Self, &'static str> {
//         if is_bipartite_undirected(value) {
//             Ok(BipartiteGraph { inner_graph: value })
//         } else {
//             Err("Graph is not bipartited")
//         }
//     }
// }

// pub fn is_bipartite_undirected<G, N, VM>(g: G, start: N) -> bool
// where
//     G: GraphRef + Visitable<NodeId = N, Map = VM> + IntoNeighbors<NodeId = N>,
//     N: Copy,
//     VM: VisitMap<N>,
// {
//     #[derive(Copy, Clone)]un
//     enum Color {
//         Red,
//         Blue,
//     }

//     let mut red = g.visit_map();
//     let mut blue = g.visit_map();

//     let mut dfs_stack = Vec::new();
//     red.visit(start);
//     dfs_stack.push((start, Color::Red));

//     while let Some((node, color)) = dfs_stack.pop() {
//         let (same_color, other_color, next_color) = match color {
//             Color::Red => (&red, &mut blue, Color::Blue),
//             Color::Blue => (&blue, &mut red, Color::Red),
//         };

//         for neighbour in g.neighbors(node) {
//             if same_color.is_visited(&neighbour) {
//                 return false;
//             }
//             if other_color.visit(neighbour) {
//                 dfs_stack.push((neighbour, next_color));
//             }
//         }
//     }
//     true
// }
