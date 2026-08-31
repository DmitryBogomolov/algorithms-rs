use super::node::{Node, Side};
use super::rbtree::RBTree;

pub trait IterItem: Sized {
    type Data;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn split(self) -> (Self::Data, Self, Self);
}

impl<K, V> IterItem for Node<K, V> {
    type Data = (K, V);

    fn len(&self) -> usize {
        Node::len(self)
    }

    fn is_empty(&self) -> bool {
        Node::is_empty(self)
    }

    fn split(self) -> (Self::Data, Self, Self) {
        self.into_parts()
    }
}

impl<'a, K, V> IterItem for &'a Node<K, V> {
    type Data = (&'a K, &'a V);

    fn len(&self) -> usize {
        Node::len(self)
    }

    fn is_empty(&self) -> bool {
        Node::is_empty(self)
    }

    fn split(self) -> (Self::Data, Self, Self) {
        (self.key_val(), self.node(Side::L), self.node(Side::R))
    }
}

impl<'a, K, V> IterItem for &'a mut Node<K, V> {
    type Data = (&'a K, &'a mut V);

    fn len(&self) -> usize {
        Node::len(self)
    }

    fn is_empty(&self) -> bool {
        Node::is_empty(self)
    }

    fn split(self) -> (Self::Data, Self, Self) {
        let ptr: *mut Node<K, V> = self;
        unsafe {
            (
                (*ptr).key_val_mut(),
                (*ptr).node_mut(Side::L),
                (*ptr).node_mut(Side::R),
            )
        }
    }
}

pub struct TreeIter<T: IterItem> {
    stack: Vec<(T::Data, T)>,
    len: usize,
}

impl<T: IterItem> TreeIter<T> {
    fn new(node: T) -> Self {
        let len = node.len();
        if len == 0 {
            return Self {
                stack: Vec::new(),
                len,
            };
        }
        let depth = (len as f32).log2().ceil() as usize;
        let mut iter = Self {
            stack: Vec::with_capacity(depth),
            len,
        };
        iter.push(node);
        iter
    }

    fn push(&mut self, node: T) {
        if node.is_empty() {
            return;
        }
        let (data, l_node, r_node) = node.split();
        self.stack.push((data, r_node));
        self.push(l_node);
    }
}

impl<T: IterItem> Iterator for TreeIter<T> {
    type Item = T::Data;

    fn next(&mut self) -> Option<Self::Item> {
        let (data, r_node) = self.stack.pop()?;
        self.len -= 1;
        self.push(r_node);
        Some(data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T: IterItem> ExactSizeIterator for TreeIter<T> {}

pub type TreeIterOut<K, V> = TreeIter<Node<K, V>>;
pub type TreeIterRef<'a, K, V> = TreeIter<&'a Node<K, V>>;
pub type TreeIterMut<'a, K, V> = TreeIter<&'a mut Node<K, V>>;

fn iter_out<K, V>(tree: &mut RBTree<K, V>) -> TreeIterOut<K, V> {
    let mut node = Node::none();
    node.replace_content(tree.root.take_content());
    TreeIterOut::new(node)
}

fn iter_ref<K, V>(tree: &RBTree<K, V>) -> TreeIterRef<'_, K, V> {
    TreeIterRef::new(&tree.root)
}

fn iter_mut<K, V>(tree: &mut RBTree<K, V>) -> TreeIterMut<'_, K, V> {
    TreeIterMut::new(&mut tree.root)
}

impl<K, V> IntoIterator for RBTree<K, V> {
    type Item = (K, V);
    type IntoIter = TreeIterOut<K, V>;

    fn into_iter(mut self) -> Self::IntoIter {
        iter_out(&mut self)
    }
}

impl<'a, K, V> IntoIterator for &'a RBTree<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = TreeIterRef<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_ref(self)
    }
}

impl<'a, K, V> IntoIterator for &'a mut RBTree<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = TreeIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_mut(self)
    }
}

impl<K, V> RBTree<K, V> {
    pub fn iter(&self) -> TreeIterRef<'_, K, V> {
        iter_ref(self)
    }

    pub fn iter_mut(&mut self) -> TreeIterMut<'_, K, V> {
        iter_mut(self)
    }

    pub fn drain(&mut self) -> TreeIterOut<K, V> {
        iter_out(self)
    }
}
