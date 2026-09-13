use graph::Graph;

#[derive(Clone)]
pub struct TestGraph {
    num_vertices: usize,
    num_edges: usize,
    adjacency: Vec<Vec<usize>>,
}

impl Graph for TestGraph {
    type VertexIter<'a> = std::iter::Copied<std::slice::Iter<'a, usize>>;

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn adjacent_vertices(&self, vertex: usize) -> Self::VertexIter<'_> {
        self.adjacency[vertex].iter().copied()
    }
}

impl std::fmt::Debug for TestGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestGraph")
            .field("vertices", &self.num_vertices)
            .field("edges", &self.num_edges)
            .finish()
    }
}

impl TestGraph {
    pub fn new(edges: impl Iterator<Item = (usize, usize)>) -> Self {
        let edges: Vec<_> = edges.collect();
        assert!(!edges.is_empty(), "no edges");
        let max_vertex_id = edges.iter().map(|t| t.0).max().expect("not expected");
        let mut adj: Vec<Vec<usize>> = Vec::new();
        adj.resize_with(max_vertex_id + 1, Vec::new);
        for (i, j) in &edges {
            add_adj(&mut adj[*i], *j);
            add_adj(&mut adj[*j], *i);
        }
        Self {
            num_vertices: adj.len(),
            num_edges: edges.len(),
            adjacency: adj,
        }
    }
}

fn add_adj(adj: &mut Vec<usize>, k: usize) {
    assert!(!adj.contains(&k), "duplicate adjacency {}", k);
    adj.push(k);
}
