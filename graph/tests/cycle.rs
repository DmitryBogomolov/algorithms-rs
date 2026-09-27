mod test_graph;

use graph::{find_cycle, find_parallel_edges, find_self_loops};
use test_graph::TestGraph;

fn check<T: Eq + std::fmt::Debug>(
    actual: impl Iterator<Item = T>,
    expected: impl IntoIterator<Item = T>,
) {
    let actual_vec: Vec<_> = actual.collect();
    let expected_vec: Vec<_> = expected.into_iter().collect();
    assert_eq!(actual_vec, expected_vec);
}

#[test]
fn empty_graph() {
    let graph = TestGraph::new(0, []);
    check(find_self_loops(&graph), []);
    check(find_parallel_edges(&graph), []);
    assert_eq!(find_cycle(&graph), None);
}

#[test]
fn no_edges_graph() {
    let graph = TestGraph::new(4, []);
    check(find_self_loops(&graph), []);
    check(find_parallel_edges(&graph), []);
    assert_eq!(find_cycle(&graph), None);
}

#[test]
fn simple_graph() {
    let graph = TestGraph::new(4, [(0, 1), (1, 2), (2, 0), (3, 2)]);
    check(find_self_loops(&graph), []);
    check(find_parallel_edges(&graph), []);
    assert_eq!(find_cycle(&graph), Some(vec![0, 1, 2]));
}

#[test]
fn simple_graph_no_cycle() {
    let graph = TestGraph::new(4, [(0, 1), (1, 2), (3, 2)]);
    check(find_self_loops(&graph), []);
    check(find_parallel_edges(&graph), []);
    assert_eq!(find_cycle(&graph), None);
}

#[test]
fn self_loops_with_duplicates() {
    let graph = TestGraph::new(5, [(3, 3), (0, 1), (1, 1), (1, 2), (1, 1), (3, 4)]);
    check(find_self_loops(&graph), [1, 1, 3]);
    check(find_parallel_edges(&graph), [(1, 1)]);
}

#[test]
fn parallel_edges_with_duplicates() {
    let graph = TestGraph::new(5, [(0, 2), (0, 1), (0, 2), (0, 2), (1, 2), (3, 4), (3, 4)]);
    check(find_self_loops(&graph), []);
    check(
        find_parallel_edges(&graph),
        [(0, 2), (2, 0), (3, 4), (4, 3)],
    );
}

#[test]
fn parallel_self_loops() {
    let graph = TestGraph::new(3, [(0, 0), (2, 2), (2, 2), (0, 0), (2, 2), (2, 2)]);
    check(find_self_loops(&graph), [0, 0, 2, 2, 2, 2]);
    check(find_parallel_edges(&graph), [(0, 0), (2, 2)]);
}
