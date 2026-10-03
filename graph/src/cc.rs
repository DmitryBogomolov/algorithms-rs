use crate::graph::Graph;
use crate::util::assert_in_range;

#[derive(Clone, PartialEq, Eq)]
pub struct CC {
    components: Vec<usize>,
    sizes: Vec<usize>,
}

impl CC {
    pub const fn count(&self) -> usize {
        self.sizes.len()
    }

    pub fn vertex_component(&self, vertex: usize) -> usize {
        assert_in_range(vertex, self.components.len(), "vertex");
        self.components[vertex]
    }

    pub fn component_size(&self, component: usize) -> usize {
        assert_in_range(component, self.sizes.len(), "component");
        self.sizes[component]
    }

    pub fn component_vertices(&self, component: usize) -> impl Iterator<Item = usize> {
        assert_in_range(component, self.sizes.len(), "component");
        self.components
            .iter()
            .enumerate()
            .filter_map(move |(v, c)| (*c == component).then_some(v))
    }

    pub fn connected(&self, vertex1: usize, vertex2: usize) -> bool {
        self.vertex_component(vertex1) == self.vertex_component(vertex2)
    }

    pub fn new<G: Graph>(graph: &G) -> Self {
        let mut components = vec![usize::MAX; graph.num_vertices()];
        let mut sizes = Vec::new();
        let mut size;
        let mut stack = Vec::new();
        let mut visited = vec![false; graph.num_vertices()];
        for vertex in 0..graph.num_vertices() {
            if visited[vertex] {
                continue;
            }
            stack.push((vertex, graph.adjacent_vertices(vertex)));
            visited[vertex] = true;
            components[vertex] = sizes.len();
            size = 1;
            while let Some((_vertex, adj_iter)) = stack.last_mut() {
                if let Some(adj_vertex) = adj_iter.next() {
                    if !visited[adj_vertex] {
                        visited[adj_vertex] = true;
                        components[adj_vertex] = sizes.len();
                        size += 1;
                        stack.push((adj_vertex, graph.adjacent_vertices(adj_vertex)));
                    }
                } else {
                    stack.pop();
                }
            }
            sizes.push(size);
        }
        Self { components, sizes }
    }
}

impl std::fmt::Debug for CC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectedComponents")
            .field("count", &self.sizes.len())
            .finish()
    }
}
