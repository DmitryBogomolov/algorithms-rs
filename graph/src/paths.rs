use crate::graph::Graph;

#[derive(Clone, PartialEq, Eq)]
pub struct Paths {
    source_vertex: usize,
    connected_count: usize,
    links: Vec<Option<usize>>,
}

impl Paths {
    pub const fn source_vertex(&self) -> usize {
        self.source_vertex
    }

    pub const fn connected_count(&self) -> usize {
        self.connected_count
    }

    pub fn has_path(&self, vertex: usize) -> bool {
        assert!(
            vertex < self.links.len(),
            "vertex {} out of range {}",
            vertex,
            self.links.len(),
        );
        vertex == self.source_vertex || self.links[vertex].is_some()
    }

    pub fn path_to(&self, vertex: usize) -> Option<Vec<usize>> {
        if !self.has_path(vertex) {
            return None;
        }
        let mut path = Vec::new();
        let mut link = Some(vertex);
        while let Some(k) = link {
            path.push(k);
            link = self.links[k];
        }
        path.reverse();
        Some(path)
    }

    pub fn new_dfs<G: Graph>(graph: &G, source_vertex: usize) -> Self {
        find_paths(graph, source_vertex, |graph, vertex, f| {
            visit_dfs(graph, vertex, f)
        })
    }

    pub fn new_bfs<G: Graph>(graph: &G, source_vertex: usize) -> Self {
        find_paths(graph, source_vertex, |graph, vertex, f| {
            visit_bfs(graph, vertex, f)
        })
    }
}

fn find_paths<G, F>(graph: &G, source_vertex: usize, visit_func: F) -> Paths
where
    G: Graph,
    F: FnOnce(&G, usize, &mut dyn FnMut(usize, usize)),
{
    assert!(
        source_vertex < graph.num_vertices(),
        "vertex {} out of range {}",
        source_vertex,
        graph.num_vertices(),
    );
    let mut count = 1;
    let mut links = vec![None; graph.num_vertices()];
    visit_func(graph, source_vertex, &mut |vertex, adj_vertex| {
        count += 1;
        links[adj_vertex] = Some(vertex);
    });
    Paths {
        source_vertex,
        connected_count: count,
        links,
    }
}

fn visit_dfs<G: Graph, F: FnMut(usize, usize)>(graph: &G, source_vertex: usize, mut f: F) {
    let mut visited = vec![false; graph.num_vertices()];
    visit_dfs_recursive(graph, source_vertex, &mut visited, &mut f);
}

fn visit_dfs_recursive<G: Graph, F: FnMut(usize, usize)>(
    graph: &G,
    vertex: usize,
    visited: &mut [bool],
    f: &mut F,
) {
    visited[vertex] = true;
    for adj_vertex in graph.adjacent_vertices(vertex) {
        if !visited[adj_vertex] {
            f(vertex, adj_vertex);
            visit_dfs_recursive(graph, adj_vertex, visited, f);
        }
    }
}

fn visit_bfs<G: Graph, F: FnMut(usize, usize)>(graph: &G, source_vertex: usize, mut f: F) {
    let mut visited = vec![false; graph.num_vertices()];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(source_vertex);
    visited[source_vertex] = true;
    while let Some(vertex) = queue.pop_front() {
        for adj_vertex in graph.adjacent_vertices(vertex) {
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                f(vertex, adj_vertex);
                queue.push_back(adj_vertex);
            }
        }
    }
}

impl std::fmt::Debug for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paths")
            .field("source_vertex", &self.source_vertex)
            .field("connected_count", &self.connected_count)
            .finish()
    }
}
