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
    pub fn new(num_vertices: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let edges: Vec<_> = edges.into_iter().collect();
        let mut adjacency: Vec<Vec<usize>> = Vec::new();
        adjacency.resize_with(num_vertices, Vec::new);
        let mut num_edges = 0;
        for (i, j) in edges {
            num_edges += 1;
            adjacency[i].push(j);
            if i != j {
                adjacency[j].push(i);
            }
        }
        Self {
            num_vertices,
            num_edges,
            adjacency,
        }
    }
}
