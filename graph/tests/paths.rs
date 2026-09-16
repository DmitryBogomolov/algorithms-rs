mod test_graph;

use graph::Paths;
use test_graph::TestGraph;

#[test]
fn empty_graph() {
    let g = TestGraph::new(0, []);

    assert!(std::panic::catch_unwind(|| Paths::new_dfs(&g, 0)).is_err());
    assert!(std::panic::catch_unwind(|| Paths::new_bfs(&g, 0)).is_err());
}

#[test]
fn no_edges_graph() {
    let g = TestGraph::new(4, []);

    for i in 0..4 {
        let mut routes: Vec<_> = (0..4).map(|i| (i, None)).collect();
        routes[i].1 = Some(vec![i]);
        check_paths(Paths::new_dfs(&g, i), i, 1, routes.clone());
        check_paths(Paths::new_bfs(&g, i), i, 1, routes.clone());
    }

    assert!(std::panic::catch_unwind(|| Paths::new_dfs(&g, 4)).is_err());
}

#[test]
fn common_graph() {
    let g = TestGraph::new(
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

    check_paths(
        Paths::new_dfs(&g, 0),
        0,
        6,
        [
            (0, Some(vec![0])),
            (1, Some(vec![0, 5, 3, 2, 1])),
            (2, Some(vec![0, 5, 3, 2])),
            (3, Some(vec![0, 5, 3])),
            (4, Some(vec![0, 5, 3, 2, 4])),
            (5, Some(vec![0, 5])),
            (6, None),
        ],
    );
    check_paths(
        Paths::new_bfs(&g, 0),
        0,
        6,
        [
            (0, Some(vec![0])),
            (1, Some(vec![0, 1])),
            (2, Some(vec![0, 2])),
            (3, Some(vec![0, 5, 3])),
            (4, Some(vec![0, 2, 4])),
            (5, Some(vec![0, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 1),
        1,
        6,
        [
            (0, Some(vec![1, 2, 4, 3, 5, 0])),
            (1, Some(vec![1])),
            (2, Some(vec![1, 2])),
            (3, Some(vec![1, 2, 4, 3])),
            (4, Some(vec![1, 2, 4])),
            (5, Some(vec![1, 2, 4, 3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 1),
        1,
        6,
        [
            (0, Some(vec![1, 0])),
            (1, Some(vec![1])),
            (2, Some(vec![1, 2])),
            (3, Some(vec![1, 2, 3])),
            (4, Some(vec![1, 2, 4])),
            (5, Some(vec![1, 0, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 2),
        2,
        6,
        [
            (0, Some(vec![2, 4, 3, 5, 0])),
            (1, Some(vec![2, 4, 3, 5, 0, 1])),
            (2, Some(vec![2])),
            (3, Some(vec![2, 4, 3])),
            (4, Some(vec![2, 4])),
            (5, Some(vec![2, 4, 3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 2),
        2,
        6,
        [
            (0, Some(vec![2, 0])),
            (1, Some(vec![2, 1])),
            (2, Some(vec![2])),
            (3, Some(vec![2, 3])),
            (4, Some(vec![2, 4])),
            (5, Some(vec![2, 3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 3),
        3,
        6,
        [
            (0, Some(vec![3, 2, 1, 0])),
            (1, Some(vec![3, 2, 1])),
            (2, Some(vec![3, 2])),
            (3, Some(vec![3])),
            (4, Some(vec![3, 2, 4])),
            (5, Some(vec![3, 2, 1, 0, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 3),
        3,
        6,
        [
            (0, Some(vec![3, 2, 0])),
            (1, Some(vec![3, 2, 1])),
            (2, Some(vec![3, 2])),
            (3, Some(vec![3])),
            (4, Some(vec![3, 4])),
            (5, Some(vec![3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 4),
        4,
        6,
        [
            (0, Some(vec![4, 2, 3, 5, 0])),
            (1, Some(vec![4, 2, 3, 5, 0, 1])),
            (2, Some(vec![4, 2])),
            (3, Some(vec![4, 2, 3])),
            (4, Some(vec![4])),
            (5, Some(vec![4, 2, 3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 4),
        4,
        6,
        [
            (0, Some(vec![4, 2, 0])),
            (1, Some(vec![4, 2, 1])),
            (2, Some(vec![4, 2])),
            (3, Some(vec![4, 3])),
            (4, Some(vec![4])),
            (5, Some(vec![4, 3, 5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 5),
        5,
        6,
        [
            (0, Some(vec![5, 0])),
            (1, Some(vec![5, 0, 1])),
            (2, Some(vec![5, 0, 1, 2])),
            (3, Some(vec![5, 0, 1, 2, 4, 3])),
            (4, Some(vec![5, 0, 1, 2, 4])),
            (5, Some(vec![5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 5),
        5,
        6,
        [
            (0, Some(vec![5, 0])),
            (1, Some(vec![5, 0, 1])),
            (2, Some(vec![5, 0, 2])),
            (3, Some(vec![5, 3])),
            (4, Some(vec![5, 3, 4])),
            (5, Some(vec![5])),
            (6, None),
        ],
    );

    check_paths(
        Paths::new_dfs(&g, 6),
        6,
        1,
        [
            (0, None),
            (1, None),
            (2, None),
            (3, None),
            (4, None),
            (5, None),
            (6, Some(vec![6])),
        ],
    );

    check_paths(
        Paths::new_bfs(&g, 6),
        6,
        1,
        [
            (0, None),
            (1, None),
            (2, None),
            (3, None),
            (4, None),
            (5, None),
            (6, Some(vec![6])),
        ],
    );
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
