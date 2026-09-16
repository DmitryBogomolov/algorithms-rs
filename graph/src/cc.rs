use crate::graph::Graph;

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
        assert!(
            vertex < self.components.len(),
            "vertex {} out of range {}",
            vertex,
            self.components.len(),
        );
        self.components[vertex]
    }

    pub fn component_size(&self, component: usize) -> usize {
        assert!(
            component < self.sizes.len(),
            "component {} out of range {}",
            component,
            self.sizes.len(),
        );
        self.sizes[component]
    }

    pub fn component_vertices(&self, component: usize) -> Vec<usize> {
        assert!(
            component < self.sizes.len(),
            "component {} out of range {}",
            component,
            self.sizes.len(),
        );
        self.components
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == component)
            .map(|(v, _)| v)
            .collect()
    }

    pub fn connected(&self, vertex1: usize, vertex2: usize) -> bool {
        assert!(
            vertex1 < self.components.len(),
            "vertex {} out of range {}",
            vertex1,
            self.components.len(),
        );
        assert!(
            vertex2 < self.components.len(),
            "vertex {} out of range {}",
            vertex2,
            self.components.len(),
        );
        self.components[vertex1] == self.components[vertex2]
    }

    pub fn new<G: Graph>(graph: &G) -> Self {
        let mut context = VisitContext {
            visited: vec![false; graph.num_vertices()],
            components: vec![0; graph.num_vertices()],
            sizes: vec![],
        };
        for vertex in 0..graph.num_vertices() {
            if !context.visited[vertex] {
                context.sizes.push(0);
                visit_recursive(graph, vertex, &mut context);
            }
        }
        Self {
            components: context.components,
            sizes: context.sizes,
        }
    }
}

struct VisitContext {
    visited: Vec<bool>,
    components: Vec<usize>,
    sizes: Vec<usize>,
}

fn visit_recursive<G: Graph>(graph: &G, vertex: usize, context: &mut VisitContext) {
    context.visited[vertex] = true;
    let component_id = context.sizes.len() - 1;
    context.components[vertex] = component_id;
    context.sizes[component_id] += 1;
    for adj_vertex in graph.adjacent_vertices(vertex) {
        if !context.visited[adj_vertex] {
            visit_recursive(graph, adj_vertex, context);
        }
    }
}

impl std::fmt::Debug for CC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnComps")
            .field("count", &self.sizes.len())
            .finish()
    }
}
