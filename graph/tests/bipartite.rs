mod common;

use common::TestGraph;
use graph::check_bipartite;

#[test]
fn empty_graph() {
    assert_eq!(check_bipartite(&TestGraph::new(0, [])), Some(vec![]));
}

#[test]
fn no_edges_graph() {
    assert_eq!(
        check_bipartite(&TestGraph::new(4, [])),
        Some(vec![false, false, false, false])
    );
}

#[test]
fn bipartite_graph() {
    assert_eq!(
        check_bipartite(&TestGraph::new(4, [(0, 1), (1, 2)])),
        Some(vec![false, true, false, false])
    );
    assert_eq!(
        check_bipartite(&TestGraph::new(4, [(0, 1), (1, 2), (2, 3)])),
        Some(vec![false, true, false, true])
    );
    assert_eq!(
        check_bipartite(&TestGraph::new(4, [(0, 1), (1, 2), (2, 3), (0, 3)])),
        Some(vec![false, true, false, true])
    );
    assert_eq!(
        check_bipartite(&TestGraph::new(
            6,
            [(0, 1), (0, 3), (1, 2), (2, 3), (1, 4), (2, 5), (4, 5)]
        )),
        Some(vec![false, true, false, true, false, true])
    );
}

#[test]
fn not_bipartite_graph() {
    assert_eq!(
        check_bipartite(&TestGraph::new(4, [(0, 1), (1, 2), (0, 2)])),
        None
    );
    assert_eq!(
        check_bipartite(&TestGraph::new(
            6,
            [
                (0, 1),
                (0, 3),
                (1, 2),
                (2, 3),
                (2, 4),
                (1, 4),
                (2, 5),
                (4, 5)
            ]
        )),
        None
    );
}
