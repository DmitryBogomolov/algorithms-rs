mod common;

use common::TestGraph;
use graph::Paths;

#[test]
fn empty_graph() {
    let graph = TestGraph::new(0, []);

    assert!(std::panic::catch_unwind(|| Paths::new_dfs(&graph, 0)).is_err());
    assert!(std::panic::catch_unwind(|| Paths::new_bfs(&graph, 0)).is_err());
}

#[test]
fn no_edges_graph() {
    let graph = TestGraph::new(4, []);

    for i in 0..4 {
        let mut routes: Vec<_> = (0..4).map(|i| (i, None)).collect();
        routes[i].1 = Some(vec![i]);
        check_paths(Paths::new_dfs(&graph, i), i, 1, routes.clone());
        check_paths(Paths::new_bfs(&graph, i), i, 1, routes.clone());
    }

    assert!(std::panic::catch_unwind(|| Paths::new_dfs(&graph, 4)).is_err());
}

#[test]
fn common_graph() {
    let graph = TestGraph::new(
        7,
        [
            (0, 5),
            (2, 4),
            (2, 3),
            (1, 2),
            (0, 1),
            (3, 4),
            (3, 5),
            (0, 2),
        ],
    );

    let cases = [
        (
            0,
            6,
            [
                Some(vec![0]),
                Some(vec![0, 5, 3, 2, 1]),
                Some(vec![0, 5, 3, 2]),
                Some(vec![0, 5, 3]),
                Some(vec![0, 5, 3, 2, 4]),
                Some(vec![0, 5]),
                None,
            ],
            [
                Some(vec![0]),
                Some(vec![0, 1]),
                Some(vec![0, 2]),
                Some(vec![0, 5, 3]),
                Some(vec![0, 2, 4]),
                Some(vec![0, 5]),
                None,
            ],
        ),
        (
            1,
            6,
            [
                Some(vec![1, 2, 4, 3, 5, 0]),
                Some(vec![1]),
                Some(vec![1, 2]),
                Some(vec![1, 2, 4, 3]),
                Some(vec![1, 2, 4]),
                Some(vec![1, 2, 4, 3, 5]),
                None,
            ],
            [
                Some(vec![1, 0]),
                Some(vec![1]),
                Some(vec![1, 2]),
                Some(vec![1, 2, 3]),
                Some(vec![1, 2, 4]),
                Some(vec![1, 0, 5]),
                None,
            ],
        ),
        (
            2,
            6,
            [
                Some(vec![2, 4, 3, 5, 0]),
                Some(vec![2, 4, 3, 5, 0, 1]),
                Some(vec![2]),
                Some(vec![2, 4, 3]),
                Some(vec![2, 4]),
                Some(vec![2, 4, 3, 5]),
                None,
            ],
            [
                Some(vec![2, 0]),
                Some(vec![2, 1]),
                Some(vec![2]),
                Some(vec![2, 3]),
                Some(vec![2, 4]),
                Some(vec![2, 3, 5]),
                None,
            ],
        ),
        (
            3,
            6,
            [
                Some(vec![3, 2, 1, 0]),
                Some(vec![3, 2, 1]),
                Some(vec![3, 2]),
                Some(vec![3]),
                Some(vec![3, 2, 4]),
                Some(vec![3, 2, 1, 0, 5]),
                None,
            ],
            [
                Some(vec![3, 2, 0]),
                Some(vec![3, 2, 1]),
                Some(vec![3, 2]),
                Some(vec![3]),
                Some(vec![3, 4]),
                Some(vec![3, 5]),
                None,
            ],
        ),
        (
            4,
            6,
            [
                Some(vec![4, 2, 3, 5, 0]),
                Some(vec![4, 2, 3, 5, 0, 1]),
                Some(vec![4, 2]),
                Some(vec![4, 2, 3]),
                Some(vec![4]),
                Some(vec![4, 2, 3, 5]),
                None,
            ],
            [
                Some(vec![4, 2, 0]),
                Some(vec![4, 2, 1]),
                Some(vec![4, 2]),
                Some(vec![4, 3]),
                Some(vec![4]),
                Some(vec![4, 3, 5]),
                None,
            ],
        ),
        (
            5,
            6,
            [
                Some(vec![5, 0]),
                Some(vec![5, 0, 1]),
                Some(vec![5, 0, 1, 2]),
                Some(vec![5, 0, 1, 2, 4, 3]),
                Some(vec![5, 0, 1, 2, 4]),
                Some(vec![5]),
                None,
            ],
            [
                Some(vec![5, 0]),
                Some(vec![5, 0, 1]),
                Some(vec![5, 0, 2]),
                Some(vec![5, 3]),
                Some(vec![5, 3, 4]),
                Some(vec![5]),
                None,
            ],
        ),
        (
            6,
            1,
            [None, None, None, None, None, None, Some(vec![6])],
            [None, None, None, None, None, None, Some(vec![6])],
        ),
    ];

    for (source, count, dfs_routes, bfs_routes) in cases {
        check_paths(
            Paths::new_dfs(&graph, source),
            source,
            count,
            dfs_routes.into_iter().enumerate(),
        );
        check_paths(
            Paths::new_bfs(&graph, source),
            source,
            count,
            bfs_routes.into_iter().enumerate(),
        );
    }
}

fn check_paths(
    paths: Paths,
    source: usize,
    count: usize,
    routes: impl IntoIterator<Item = (usize, Option<Vec<usize>>)>,
) {
    assert_eq!(paths.source_vertex(), source, "source_vertex");
    assert_eq!(paths.connected_count(), count, "connected_count");
    for (target, points) in routes {
        assert_eq!(
            paths.has_path(target),
            points.is_some(),
            "has path to {}",
            target
        );
        assert_eq!(paths.path_to(target), points, "path to {}", target);
    }
}
