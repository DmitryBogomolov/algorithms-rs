use crate::graph::Graph;
use crate::util::assert_in_range;

const UNASSIGNED: usize = usize::MAX;

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
        let mut components = vec![UNASSIGNED; graph.num_vertices()];
        let mut sizes = Vec::new();
        for vertex in 0..graph.num_vertices() {
            if components[vertex] == UNASSIGNED {
                let size = visit_recursive(graph, vertex, sizes.len(), &mut components);
                sizes.push(size);
            }
        }
        Self { components, sizes }
    }
}

fn visit_recursive<G: Graph>(
    graph: &G,
    vertex: usize,
    component_id: usize,
    components: &mut [usize],
) -> usize {
    components[vertex] = component_id;
    let mut size = 1;
    for adj_vertex in graph.adjacent_vertices(vertex) {
        if components[adj_vertex] == UNASSIGNED {
            size += visit_recursive(graph, adj_vertex, component_id, components);
        }
    }
    size
}

impl std::fmt::Debug for CC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectedComponents")
            .field("count", &self.sizes.len())
            .finish()
    }
}
