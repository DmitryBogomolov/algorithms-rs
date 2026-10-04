use crate::graph::Graph;

pub fn check_bipartite<G: Graph>(graph: &G) -> Option<Vec<bool>> {
    let mut colors = vec![None; graph.num_vertices()];
    let mut visited = vec![false; graph.num_vertices()];
    let mut stack = Vec::new();
    for vertex in 0..graph.num_vertices() {
        if visited[vertex] {
            continue;
        }
        stack.push((vertex, graph.adjacent_vertices(vertex)));
        visited[vertex] = true;
        colors[vertex] = Some(false);
        while let Some((vertex, adj_iter)) = stack.last_mut() {
            if let Some(adj_vertex) = adj_iter.next() {
                if !visited[adj_vertex] {
                    visited[adj_vertex] = true;
                    colors[adj_vertex] = colors[*vertex].map(|t| !t);
                    stack.push((adj_vertex, graph.adjacent_vertices(adj_vertex)));
                } else if colors[adj_vertex] == colors[*vertex] {
                    return None;
                }
            } else {
                stack.pop();
            }
        }
    }
    Some(colors.into_iter().map(|t| t.expect("must exist")).collect())
}
