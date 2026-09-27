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
    if graph.num_vertices() == 0 || graph.num_edges() == 0 {
        return None;
    }
    let mut visited = vec![false; graph.num_vertices()];
    let mut links = vec![None; graph.num_vertices()];
    for vertex in 0..graph.num_vertices() {
        if !visited[vertex] {
            let cycle = find_cycle_recursive(graph, vertex, None, &mut visited, &mut links);
            if cycle.is_some() {
                return cycle;
            }
        }
    }
    None
}

fn find_cycle_recursive<G: Graph>(
    graph: &G,
    vertex: usize,
    prev_vertex: Option<usize>,
    visited: &mut [bool],
    links: &mut [Option<usize>],
) -> Option<Vec<usize>> {
    visited[vertex] = true;
    links[vertex] = prev_vertex;
    for adj_vertex in graph.adjacent_vertices(vertex) {
        if !visited[adj_vertex] {
            let cycle = find_cycle_recursive(graph, adj_vertex, Some(vertex), visited, links);
            if cycle.is_some() {
                return cycle;
            }
        } else if Some(adj_vertex) != prev_vertex {
            let mut cycle = Vec::new();
            let mut v = vertex;
            while v != adj_vertex {
                cycle.push(v);
                v = links[v].expect("link must exist");
            }
            cycle.push(adj_vertex);
            cycle.reverse();
            return Some(cycle);
        }
    }
    None
}
