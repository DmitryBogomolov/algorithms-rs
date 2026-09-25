mod test_graph;

use graph::{find_parallel_edges, find_self_loops};
use test_graph::TestGraph;

#[test]
fn empty_graph() {
    let graph = TestGraph::new(0, []);
    assert_eq!(find_self_loops(&graph).collect::<Vec<_>>(), []);
    assert_eq!(find_parallel_edges(&graph).collect::<Vec<_>>(), []);
}

#[test]
fn no_edges_graph() {
    let graph = TestGraph::new(4, []);
    assert_eq!(find_self_loops(&graph).collect::<Vec<_>>(), []);
    assert_eq!(find_parallel_edges(&graph).collect::<Vec<_>>(), []);
}

#[test]
fn no_self_loops() {
    let graph = TestGraph::new(4, [(0, 1), (1, 2), (2, 0)]);
    assert_eq!(find_self_loops(&graph).collect::<Vec<_>>(), []);
}

#[test]
fn self_loops_in_vertex_order_with_duplicates() {
    let graph = TestGraph::new(5, [(3, 3), (0, 1), (1, 1), (1, 2), (1, 1), (3, 4)]);
    assert_eq!(find_self_loops(&graph).collect::<Vec<_>>(), [1, 1, 3]);
}

#[test]
fn no_parallel_edges() {
    let graph = TestGraph::new(4, [(0, 1), (1, 2), (2, 0), (2, 2)]);
    assert_eq!(find_parallel_edges(&graph).collect::<Vec<_>>(), []);
}

#[test]
fn parallel_edges_in_both_directions_with_duplicates() {
    let graph = TestGraph::new(5, [(0, 2), (0, 1), (0, 2), (0, 2), (1, 2), (3, 4), (3, 4)]);
    assert_eq!(
        find_parallel_edges(&graph).collect::<Vec<_>>(),
        [(0, 2), (0, 2), (2, 0), (2, 0), (3, 4), (4, 3)]
    );
}

#[test]
fn parallel_self_loops() {
    let graph = TestGraph::new(3, [(0, 0), (2, 2), (2, 2), (2, 2)]);
    assert_eq!(
        find_parallel_edges(&graph).collect::<Vec<_>>(),
        [(2, 2), (2, 2)]
    );
}
