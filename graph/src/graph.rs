pub struct Graph {
    num_vertices: usize,
    num_edges: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(num_vertices: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let (num_edges, adj) = build_adj(num_vertices, edges);
        Self {
            num_vertices,
            num_edges,
            adj,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    pub fn adjacent_vertices(&self, vertex_id: usize) -> VertexIter<'_> {
        VertexIter {
            iter: self.adj[vertex_id].iter(),
        }
    }
}

fn build_adj(num_vertices: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> (usize, Vec<Vec<usize>>) {
    let mut count = 0;
    let mut adj = vec![Vec::new(); num_vertices];
    for (i, j) in edges {
        count += 1;
        adj[i].push(j);
        adj[j].push(i);
    }
    (count, adj)
}

pub struct VertexIter<'a> {
    iter: std::slice::Iter<'a, usize>,
}

impl<'a> Iterator for VertexIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| t.clone())
    }
}
