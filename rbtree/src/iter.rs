use super::rbtree::RBTree;
use super::node::Node;

pub struct TreeIter<K, V> {
    stack: Vec<((K, V), Node<K, V>)>,
}

impl<K, V> TreeIter<K, V> {
    fn new(node: Node<K, V>) -> Self {
        let depth = (node.len() as f32).log2().ceil() as usize;
        let mut iter = Self {
            stack: Vec::with_capacity(depth),
        };
        iter.push(node);
        iter
    }

    fn push(&mut self, node: Node<K, V>) {
        if node.is_empty() {
            return;;
        }
        let (data, l_node, r_node) = node.into_parts();
        self.stack.push((data, r_node));
        self.push(l_node);
    }
}

impl<K, V> Iterator for TreeIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let (data, r_node) = self.stack.pop()?;
        self.push(r_node);
        Some(data)
    }
}

impl<K, V> IntoIterator for RBTree<K, V> {
    type Item = (K, V);
    type IntoIter = TreeIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.take_root())
    }
}
