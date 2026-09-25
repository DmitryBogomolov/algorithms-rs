use crate::graph::Graph;

pub fn find_self_loops<G: Graph>(graph: &G) -> impl Iterator<Item = usize> {
    (0..graph.num_vertices()).flat_map(|vertex| {
        graph
            .adjacent_vertices(vertex)
            .filter(move |&adj_vertex| adj_vertex == vertex)
    })
}

pub fn find_parallel_edges<G: Graph>(graph: &G) -> impl Iterator<Item = (usize, usize)> {
    let mut checked = vec![None; graph.num_vertices()];
    (0..graph.num_vertices())
        .flat_map(|vertex| {
            graph
                .adjacent_vertices(vertex)
                .map(move |adj_vertex| (vertex, adj_vertex))
        })
        .filter(move |&(vertex, adj_vertex)| {
            let is_checked = checked[adj_vertex] == Some(vertex);
            checked[adj_vertex] = Some(vertex);
            is_checked
        })
}
