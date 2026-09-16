mod test_graph;

use graph::CC;
use test_graph::TestGraph;

fn check_components(cc: &CC, data: &[&[usize]]) {
    let count = data.len();
    assert_eq!(cc.count(), count);
    for i in 0..count {
        let vertices = data[i];
        let vertex_count = vertices.len();
        assert_eq!(cc.component_size(i), vertex_count);
        assert_eq!(cc.component_vertices(i), vertices);
        for k in vertices {
            assert_eq!(cc.vertex_component(*k), i);
        }
        for k in 0..vertex_count {
            let v1 = vertices[k];
            let v2 = vertices[(k + 1) % vertex_count];
            assert!(cc.connected(v1, v2))
        }
        let v1 = data[i][0];
        let v2 = data[(i + 1) % count][0];
        assert!(!cc.connected(v1, v2));
    }
}

#[test]
fn empty_graph() {
    let cc = CC::new(&TestGraph::new(0, []));

    check_components(&cc, &[]);
}

#[test]
fn no_edges_graph() {
    let cc = CC::new(&TestGraph::new(4, []));

    check_components(&cc, &[&[0], &[1], &[2], &[3]]);
}

#[test]
fn common_graph() {
    let cc = CC::new(&TestGraph::new(
        8,
        [
            (0, 1),
            (1, 4),
            (4, 7),
            (7, 2),
            (2, 0),
            (1, 2),
            (2, 4),
            (5, 6),
        ],
    ));

    check_components(&cc, &[&[0, 1, 2, 4, 7], &[3], &[5, 6]]);
}
