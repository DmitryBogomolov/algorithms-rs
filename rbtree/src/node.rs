use std::borrow::Borrow;
use std::cmp::Ordering;

pub struct Node<K, V>(Option<Box<Content<K, V>>>);

pub struct Content<K, V> {
    l_node: Node<K, V>,
    r_node: Node<K, V>,
    red: bool,
    size: usize,
    data: (K, V),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    L,
    R,
}

impl Side {
    fn from_is_left(is_left: bool) -> Self {
        match is_left {
            true => Self::L,
            false => Self::R,
        }
    }

    fn from_ord(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => Self::L,
            Ordering::Greater => Self::R,
            _ => unreachable!("unexpected ordering"),
        }
    }

    fn other(self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

impl<K, V> Node<K, V> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |t| t.size)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn is_red(&self) -> bool {
        self.0.as_ref().is_some_and(|t| t.red)
    }

    fn update_size(&mut self) {
        self.content_mut().size = 1 + self.node(Side::L).len() + self.node(Side::R).len();
    }

    fn key_cmp<Q>(&self, k: &Q) -> Ordering
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        k.cmp(self.key().borrow())
    }

    fn content(&self) -> &Content<K, V> {
        self.0.as_ref().unwrap()
    }

    fn content_mut(&mut self) -> &mut Content<K, V> {
        self.0.as_mut().unwrap()
    }

    pub fn key(&self) -> &K {
        &self.content().data.0
    }

    pub fn val(&self) -> &V {
        &self.content().data.1
    }

    pub fn val_mut(&mut self) -> &mut V {
        &mut self.content_mut().data.1
    }

    pub fn node(&self, side: Side) -> &Self {
        match side {
            Side::L => &self.content().l_node,
            Side::R => &self.content().r_node,
        }
    }

    pub fn node_mut(&mut self, side: Side) -> &mut Self {
        match side {
            Side::L => &mut self.content_mut().l_node,
            Side::R => &mut self.content_mut().r_node,
        }
    }

    pub fn into_parts(mut self) -> ((K, V), Self, Self) {
        let content = self.take_content().unwrap();
        (content.data, content.l_node, content.r_node)
    }

    pub fn find<Q>(&self, k: &Q) -> Option<&Self>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        if self.is_empty() {
            return None;
        }
        match self.key_cmp(k) {
            Ordering::Equal => Some(self),
            ord => self.node(Side::from_ord(ord)).find(k),
        }
    }

    pub fn find_mut<Q>(&mut self, k: &Q) -> Option<&mut Self>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        if self.is_empty() {
            return None;
        }
        match self.key_cmp(k) {
            Ordering::Equal => Some(self),
            ord => self.node_mut(Side::from_ord(ord)).find_mut(k),
        }
    }

    fn set_data(&mut self, data: (K, V)) {
        self.0 = Some(Box::new(Content {
            l_node: Self::none(),
            r_node: Self::none(),
            red: true,
            size: 1,
            data,
        }));
    }

    pub fn take_content(&mut self) -> Option<Box<Content<K, V>>> {
        self.0.take()
    }

    pub fn replace_content(
        &mut self,
        content: Option<Box<Content<K, V>>>,
    ) -> Option<Box<Content<K, V>>> {
        match content {
            None => self.0.take(),
            Some(c) => self.0.replace(c),
        }
    }

    fn replace_data(&mut self, data: (K, V)) -> Option<(K, V)> {
        let prev_data = std::mem::replace(&mut self.content_mut().data, data);
        Some(prev_data)
    }

    fn flip_color(&mut self) {
        self.content_mut().red = !self.is_red();
    }

    fn rotate(&mut self, side: Side) {
        let other_side = side.other();
        if self.is_empty() || self.node(other_side).is_empty() {
            return;
        }
        let is_root_red = self.is_red();
        let is_node_red = self.node(other_side).is_red();
        let next_root_content = self.node_mut(other_side).take_content();
        let prev_root_content = self.replace_content(next_root_content);
        let prev_content = self.node_mut(side).replace_content(prev_root_content);
        self.node_mut(side)
            .node_mut(other_side)
            .replace_content(prev_content);
        self.content_mut().red = is_root_red;
        self.node_mut(side).content_mut().red = is_node_red;
        self.node_mut(side).update_size();
        self.update_size();
    }

    fn flip_to_black(&mut self) {
        self.content_mut().red = false;
    }

    fn force_black_root(&mut self) {
        if !self.is_empty() {
            self.flip_to_black();
        }
    }

    fn balance_after_insert(&mut self) {
        let l_red = self.node(Side::L).is_red();
        let r_red = self.node(Side::R).is_red();
        if !l_red && !r_red {
            return;
        }
        if l_red && r_red {
            // red parent and red uncle - recolor
            self.flip_color();
            self.node_mut(Side::L).flip_color();
            self.node_mut(Side::R).flip_color();
            return;
        }
        let side = Side::from_is_left(l_red);
        // inner (LR or RL) violation
        if self.node(side).node(side.other()).is_red() {
            self.node_mut(side).rotate(side);
        }
        // outer (LL or RR) violation
        if self.node(side).node(side).is_red() {
            self.rotate(side.other());
        }
    }

    fn insert_recursive(&mut self, data: (K, V)) -> Option<(K, V)>
    where
        K: Ord,
    {
        if self.is_empty() {
            self.set_data(data);
            return None;
        }
        match self.key_cmp(&data.0) {
            Ordering::Equal => self.replace_data(data),
            ord => {
                let ret = self.node_mut(Side::from_ord(ord)).insert_recursive(data);
                self.balance_after_insert();
                self.update_size();
                ret
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)>
    where
        K: Ord,
    {
        let ret = self.insert_recursive((k, v));
        self.force_black_root();
        ret
    }

    fn propagate_deficit(&mut self, deficit: bool, side: Side) -> bool {
        if deficit {
            self.balance_after_remove(side)
        } else {
            false
        }
    }

    fn remove_core(&mut self) -> (Option<(K, V)>, bool) {
        let no_l = self.node(Side::L).is_empty();
        let no_r = self.node(Side::R).is_empty();
        if no_l && no_r {
            // Removed black leaf creates a black deficit.
            let is_black = !self.is_red();
            let data = self.take_content().map(|c| c.data);
            return (data, is_black);
        }
        if no_l || no_r {
            // Node with single child is black and child is red.
            let node = self.node_mut(Side::from_is_left(!no_l));
            let node_content = node.take_content();
            let data = self.replace_content(node_content).map(|c| c.data);
            // Black node deficit is restored.
            self.flip_to_black();
            return (data, false);
        }
        let (next_data, deficit) = self.node_mut(Side::R).remove_min_recursive();
        let prev_data = self.replace_data(next_data.unwrap());
        // Propagate deficit of removed next min node.
        let deficit = self.propagate_deficit(deficit, Side::R);
        self.update_size();
        (prev_data, deficit)
    }

    fn remove_min_recursive(&mut self) -> (Option<(K, V)>, bool) {
        if self.is_empty() {
            return (None, false);
        }
        if self.node(Side::L).is_empty() {
            let is_black = !self.is_red();
            let r_content = self.node_mut(Side::R).take_content();
            let has_r_node = r_content.is_some();
            let data = self.replace_content(r_content).map(|c| c.data);
            // Red leaf leaves no deficit. Red child of black node restores deficit.
            if has_r_node {
                self.flip_to_black();
            }
            return (data, is_black && !has_r_node);
        }
        let (ret, deficit) = self.node_mut(Side::L).remove_min_recursive();
        let deficit = self.propagate_deficit(deficit, Side::L);
        self.update_size();
        (ret, deficit)
    }

    fn balance_after_remove(&mut self, side: Side) -> bool {
        if self.is_empty() {
            return false;
        }
        let other_side = side.other();
        if self.node(other_side).is_red() {
            // (1) Red sibling. Rotate (the color swap blackens it),moving the deficit down to the now-red child.
            // Red node always absorbs a one-black deficit (2) or resolves it by rotation (3, 4).
            // So that recursion never propagates back here.
            self.rotate(side);
            return self.node_mut(side).balance_after_remove(side);
        }
        if self.node(other_side).is_empty() {
            // Empty here means no deficit reached this node.
            return false;
        }
        let near_red = self.node(other_side).node(side).is_red();
        let far_red = self.node(other_side).node(other_side).is_red();
        if !near_red && !far_red {
            // (2) Both sibling children black. Recolor sibling red.
            // If self is red it absorbs (resolved), else the deficit propagates up.
            self.node_mut(other_side).flip_color();
            return if self.is_red() {
                self.flip_color();
                false
            } else {
                true
            };
        }
        // (3) Near red, far black. Rotate at sibling so the red nephew becomes the far one.
        if near_red && !far_red {
            self.node_mut(other_side).rotate(other_side);
        }
        // (4) Far nephew red. Pull it over. Color-swap rotationsettles colors, deficit resolved.
        if self.node(other_side).node(other_side).is_red() {
            self.node_mut(other_side).node_mut(other_side).flip_color();
        }
        self.rotate(side);
        false
    }

    fn remove_recursive<Q>(&mut self, k: &Q) -> (Option<(K, V)>, bool)
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        if self.is_empty() {
            return (None, false);
        }
        match self.key_cmp(k) {
            Ordering::Equal => self.remove_core(),
            ord => {
                let (ret, deficit) = self.node_mut(Side::from_ord(ord)).remove_recursive(k);
                let deficit = self.propagate_deficit(deficit, Side::from_ord(ord));
                self.update_size();
                (ret, deficit)
            }
        }
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let (ret, _deficit) = self.remove_recursive(k);
        self.force_black_root();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let node: Node<(), ()> = Node::none();
        assert!(node.is_empty());
        assert_eq!(node.len(), 0);
        assert!(!node.is_red());
    }

    #[test]
    fn insert_one_node() {
        let mut node = Node::none();

        assert_eq!(node.insert(10, 'a'), None);
        assert_eq!(node.find(&10).map(|t| t.val()), Some(&'a'));

        assert!(!node.is_empty());
        assert_eq!(node.len(), 1);
        assert_eq!(node.key(), &10);
        assert_eq!(node.val(), &'a');
    }

    #[test]
    fn remove_one_node() {
        let mut node = Node::none();
        node.insert(10, 'a');

        assert_eq!(node.remove(&10), Some((10, 'a')));
        assert!(node.find(&10).is_none());

        assert!(node.is_empty());
        assert_eq!(node.len(), 0);
    }

    fn val<K, V>(n: &Node<K, V>) -> &V {
        n.val()
    }

    #[test]
    fn replace_one_node() {
        let mut node = Node::none();
        node.insert(10, 'a');

        assert_eq!(node.insert(10, 'b'), Some((10, 'a')));
        assert_eq!(node.find(&10).map(val), Some(&'b'));

        assert!(!node.is_empty());
        assert_eq!(node.len(), 1);
        assert_eq!(node.key(), &10);
        assert_eq!(node.val(), &'b');
    }

    #[test]
    fn mutate_value() {
        let mut node = Node::none();
        node.insert(10, 'a');
        node.insert(20, 'b');
        node.insert(30, 'c');

        *node.find_mut(&10).unwrap().val_mut() = 'A';
        *node.find_mut(&20).unwrap().val_mut() = 'B';
        *node.find_mut(&30).unwrap().val_mut() = 'C';

        assert_eq!(node.find(&10).map(val), Some(&'A'));
        assert_eq!(node.find(&20).map(val), Some(&'B'));
        assert_eq!(node.find(&30).map(val), Some(&'C'));
    }

    #[test]
    fn insert_remove() {
        let mut node = Node::none();
        let range = 11..20;

        for i in range.clone() {
            assert_eq!(node.insert(i.to_string(), i), None);
        }

        assert!(!node.is_empty());
        assert_eq!(node.len(), range.len());

        for i in range.clone() {
            assert_eq!(node.find(i.to_string().as_str()).map(val), Some(&i));
        }
        for i in range.clone() {
            assert_eq!(
                node.insert(i.to_string(), i + 100),
                Some((i.to_string(), i))
            );
        }
        for i in range.clone() {
            assert_eq!(node.find(i.to_string().as_str()).map(val), Some(&(i + 100)));
        }
        for i in range.clone() {
            assert_eq!(
                node.remove(i.to_string().as_str()),
                Some((i.to_string(), i + 100))
            );
        }

        assert!(node.is_empty());
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn split_to_parts() {
        let mut node = Node::none();
        node.insert(10, 'a');
        node.insert(20, 'b');
        node.insert(30, 'c');

        let (kv, l_node, r_node) = node.into_parts();
        assert_eq!(kv, (20, 'b'));
        assert_eq!(l_node.key(), &10);
        assert_eq!(r_node.key(), &30);
    }

    fn assert_balanced_ordered<K: Ord + Clone, V>(root: &Node<K, V>) {
        if root.len() < 3 {
            return;
        }
        let mut depths: Vec<usize> = Vec::new();
        let mut keys: Vec<K> = Vec::new();
        collect(root, 0, &mut depths, &mut keys);
        depths.sort();
        let min_depth = *depths.first().unwrap();
        let max_depth = *depths.last().unwrap();
        assert!(
            max_depth <= 2 * min_depth + 1,
            "not balanced ({}, {})",
            min_depth,
            max_depth
        );
        assert!(keys.is_sorted(), "not sorted");
    }

    fn collect<K: Clone, V>(
        node: &Node<K, V>,
        depth: usize,
        depths: &mut Vec<usize>,
        keys: &mut Vec<K>,
    ) {
        let l_node = node.node(Side::L);
        let r_node = node.node(Side::R);
        if l_node.is_empty() && r_node.is_empty() {
            depths.push(depth);
        }
        if !l_node.is_empty() {
            collect(l_node, depth + 1, depths, keys);
        }
        keys.push(node.key().clone());
        if !r_node.is_empty() {
            collect(r_node, depth + 1, depths, keys);
        }
    }

    #[test]
    fn balancing_inc() {
        let n = 100;
        let mut node = Node::none();

        for i in 0..n {
            assert_eq!(node.insert(1000 + i, i), None);
            assert_balanced_ordered(&node);
        }
        assert_eq!(node.len(), n);
        for i in 0..n {
            assert_eq!(node.find(&(1000 + i)).map(val), Some(&i));
        }
        for i in 0..n {
            assert_eq!(node.remove(&(1000 + i)), Some((1000 + i, i)));
            assert_balanced_ordered(&node);
        }
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn balancing_dec() {
        let n = 100;
        let mut node = Node::none();

        for i in (0..n).rev() {
            assert_eq!(node.insert(1000 + i, i), None);
            assert_balanced_ordered(&node);
        }
        assert_eq!(node.len(), n);
        for i in 0..n {
            assert_eq!(node.find(&(1000 + i)).map(val), Some(&i));
        }
        for i in (0..n).rev() {
            assert_eq!(node.remove(&(1000 + i)), Some((1000 + i, i)));
            assert_balanced_ordered(&node);
        }
        assert_eq!(node.len(), 0);
    }

    #[test]
    fn balancing_interleaved() {
        let mut node = Node::none();
        // insert 0..100, remove the even keys, then insert 100..150, then
        // remove everything that remains.
        for i in 0..100 {
            node.insert(1000 + i, i);
            assert_balanced_ordered(&node);
        }
        for i in (0..100).step_by(2) {
            assert_eq!(node.remove(&(1000 + i)), Some((1000 + i, i)));
            assert_balanced_ordered(&node);
        }
        for i in 100..200 {
            node.insert(1000 + i, i);
            assert_balanced_ordered(&node);
        }
        for i in (100..200).step_by(2) {
            assert_eq!(node.remove(&(1000 + i)), Some((1000 + i, i)));
            assert_balanced_ordered(&node);
        }
    }
}
