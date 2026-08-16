use super::rbtree::RBTree;
use super::node::{Node, Side};

pub struct TreeIter<K, V> {
    stack: Vec<((K, V), Node<K, V>)>,
}

pub struct TreeIterRef<'a, K, V> {
    stack: Vec<((&'a K, &'a V), &'a Node<K, V>)>,
}

pub struct TreeIterMut<'a, K, V> {
    stack: Vec<((&'a K, &'a mut V), &'a mut Node<K, V>)>,
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
            return;
        }
        let (data, l_node, r_node) = node.into_parts();
        self.stack.push((data, r_node));
        self.push(l_node);
    }
}

impl<'a, K, V> TreeIterRef<'a, K, V> {
    fn new(node: &'a Node<K, V>) -> Self {
        let depth = (node.len() as f32).log2().ceil() as usize;
        let mut iter = Self {
            stack: Vec::with_capacity(depth),
        };
        iter.push(node);
        iter
    }

    fn push(&mut self, node: &'a Node<K, V>) {
        if node.is_empty() {
            return;
        }
        self.stack.push(((node.key(), node.val()), node.node(Side::R)));
        self.push(node.node(Side::L));
    }
}

impl<'a, K, V> TreeIterMut<'a, K, V> {
    fn new(node: &'a mut Node<K, V>) -> Self {
        let depth = (node.len() as f32).log2().ceil() as usize;
        let mut iter = Self {
            stack: Vec::with_capacity(depth),
        };
        iter.push(node);
        iter
    }

    fn push(&mut self, node: *mut Node<K, V>) {
        unsafe {
            if (*node).is_empty() {
                return;
            }
            self.stack.push((((*node).key(), (*node).val_mut()), (*node).node_mut(Side::R)));
            self.push((*node).node_mut(Side::L));
        }
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

impl<'a, K, V> Iterator for TreeIterRef<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let (data, r_node) = self.stack.pop()?;
        self.push(r_node);
        Some(data)
    }
}

impl<'a, K, V> Iterator for TreeIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

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

impl<'a, K, V> IntoIterator for &'a RBTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = TreeIterRef<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.root())
    }
}

impl<'a, K, V> IntoIterator for &'a mut RBTree<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = TreeIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.root_mut())
    }
}

impl<K, V> RBTree<K, V> {
    pub fn iter(&self) -> TreeIterRef<'_, K, V> {
        TreeIterRef::new(self.root())
    }

    pub fn iter_mut(&mut self) -> TreeIterMut<'_, K, V> {
        TreeIterMut::new(self.root_mut())
    }
}
