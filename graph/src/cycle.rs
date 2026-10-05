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
    let mut count = vec![graph.num_vertices(); graph.num_vertices()];
    (0..graph.num_vertices())
        .flat_map(|vertex| {
            graph
                .adjacent_vertices(vertex)
                .map(move |adj_vertex| (vertex, adj_vertex))
        })
        .filter(move |&(vertex, adj_vertex)| {
            let is_checked = checked[adj_vertex] == Some(vertex) && count[adj_vertex] == 1;
            if checked[adj_vertex] == Some(vertex) {
                count[adj_vertex] += 1;
            } else {
                count[adj_vertex] = 1;
            }
            checked[adj_vertex] = Some(vertex);
            is_checked
        })
}

pub fn find_cycle<G: Graph>(graph: &G) -> Option<Vec<usize>> {
    if let Some(vertex) = find_self_loops(graph).next() {
        return Some(vec![vertex]);
    }
    if let Some((v1, v2)) = find_parallel_edges(graph).next() {
        return Some(vec![v1, v2]);
    }
    find_cycle_raw(graph)
}

fn find_cycle_raw<G: Graph>(graph: &G) -> Option<Vec<usize>> {
    let mut links = vec![None; graph.num_vertices()];
    let mut visited = vec![false; graph.num_vertices()];
    let mut stack = Vec::new();
    for vertex in 0..graph.num_vertices() {
        if visited[vertex] {
            continue;
        }
        stack.push((vertex, graph.adjacent_vertices(vertex)));
        visited[vertex] = true;
        while let Some((vertex, adj_iter)) = stack.last_mut() {
            if let Some(adj_vertex) = adj_iter.next() {
                if !visited[adj_vertex] {
                    visited[adj_vertex] = true;
                    links[adj_vertex] = Some(*vertex);
                    stack.push((adj_vertex, graph.adjacent_vertices(adj_vertex)));
                } else if adj_vertex != links[*vertex].expect("link must exist") {
                    let mut cycle = Vec::new();
                    let mut v = *vertex;
                    while v != adj_vertex {
                        cycle.push(v);
                        v = links[v].expect("link must exist");
                    }
                    cycle.push(adj_vertex);
                    cycle.reverse();
                    return Some(cycle);
                }
            } else {
                stack.pop();
            }
        }
    }
    None
}
