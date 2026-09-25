use crate::graph::Graph;

pub fn find_self_loops<G: Graph>(graph: &G) -> impl Iterator<Item = usize> {
    (0..graph.num_vertices()).flat_map(|vertex| {
        graph
            .adjacent_vertices(vertex)
            .filter(move |adj_vertex| *adj_vertex == vertex)
    })
}
