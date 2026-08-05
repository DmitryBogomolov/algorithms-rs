use std::cmp::Ordering;

// Implements *Reb-Black Binary Search Tree* container.
// https://algs4.cs.princeton.edu/33balanced/
pub struct RBTree<K, V> {
    root: NodePtr<K, V>,
}

type NodePtr<K, V> = Option<Box<Node<K, V>>>;

enum Color {
    RED,
    BLACK,
}

struct Node<K, V> {
    color: Color,
    size: usize,
    l_node: NodePtr<K, V>,
    r_node: NodePtr<K, V>,
    key: K,
    val: V,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |root| root.size)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("")
    }

    pub fn insert(&mut self, key: K, val: V) {

        todo!("")
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        todo!("")
    }

    fn new_node(key: K, val: V) -> Node<K, V> {
        Node {
            color: Color::BLACK,
            size: 0,
            l_node: None,
            r_node: None,
            key,
            val,
        }
    }
}
