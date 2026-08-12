use std::cmp::Ordering;

pub struct Node<K, V>(Option<Box<Content<K, V>>>);

struct Content<K, V> {
    l_node: Node<K, V>,
    r_node: Node<K, V>,
    red: bool,
    size: usize,
    data: (K, V),
}

/// Which child the deletion-path came through. Used to tell
/// `balance_after_remove` which side may be black-deficient without it having
/// to recompute black heights.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

impl<K: Ord, V> Node<K, V> {
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
        self.0.as_ref().map_or(false, |t| t.red)
    }

    fn update_size(&mut self) {
        self.content_mut().size = 1 + self.l_node().len() + self.r_node().len();
    }

    fn key_cmp(&self, k: &K) -> Ordering {
        k.cmp(&self.content().data.0)
    }

    fn content(&self) -> &Content<K, V> {
        self.0.as_ref().unwrap()
    }

    fn content_mut(&mut self) -> &mut Content<K, V> {
        self.0.as_mut().unwrap()
    }

    fn l_node(&self) -> &Self {
        &self.content().l_node
    }

    fn r_node(&self) -> &Self {
        &self.content().r_node
    }

    fn l_node_mut(&mut self) -> &mut Self {
        &mut self.content_mut().l_node
    }

    fn r_node_mut(&mut self) -> &mut Self {
        &mut self.content_mut().r_node
    }

    pub fn get(&self, k: &K) -> Option<&(K, V)> {
        if self.is_empty() {
            return None;
        }
        let node = match self.key_cmp(k) {
            Ordering::Equal => return Some(&self.content().data),
            Ordering::Less => self.l_node(),
            Ordering::Greater => self.r_node(),
        };
        node.get(k)
    }

    fn set_key_val(&mut self, k: K, v: V) {
        self.0 = Some(Box::new(Content {
            l_node: Self::none(),
            r_node: Self::none(),
            red: true,
            size: 1,
            data: (k, v),
        }));
    }

    fn take_content(&mut self) -> Option<Box<Content<K, V>>> {
        self.0.take()
    }

    fn replace_content(
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
        self.content_mut().red = !self.content().red;
    }

    fn flip_colors(&mut self) {
        if self.is_empty() || self.l_node().is_empty() || self.r_node().is_empty() {
            return;
        }
        self.flip_color();
        self.l_node_mut().flip_color();
        self.r_node_mut().flip_color();
    }

    fn rotate_l(&mut self) {
        if self.is_empty() || self.r_node().is_empty() {
            return;
        }
        let root_red = self.content().red;
        let node_red = self.r_node().content().red;
        let next_root_content = self.r_node_mut().take_content();
        let prev_root_content = self.replace_content(next_root_content);
        let prev_l_content = self.l_node_mut().replace_content(prev_root_content);
        self.l_node_mut()
            .r_node_mut()
            .replace_content(prev_l_content);
        self.content_mut().red = root_red;
        self.l_node_mut().content_mut().red = node_red;
        self.l_node_mut().update_size();
        self.update_size();
    }

    fn rotate_r(&mut self) {
        if self.is_empty() || self.l_node().is_empty() {
            return;
        }
        let root_red = self.content().red;
        let node_red = self.l_node().content().red;
        let next_root_content = self.l_node_mut().take_content();
        let prev_root_content = self.replace_content(next_root_content);
        let prev_r_content = self.r_node_mut().replace_content(prev_root_content);
        self.r_node_mut()
            .l_node_mut()
            .replace_content(prev_r_content);
        self.content_mut().red = root_red;
        self.r_node_mut().content_mut().red = node_red;
        self.r_node_mut().update_size();
        self.update_size();
    }

    fn force_black_root(&mut self) {
        if !self.is_empty() {
            self.content_mut().red = false;
        }
    }

    /// CLRS RB-INSERT-FIXUP, applied bottom-up. `self` is the grandparent of
    /// the node just inserted; a red-red violation exists on one side when
    /// `self`'s child (the red parent) is red and that child itself has a red
    /// child. The sibling of the red parent is the uncle. The recursion up
    /// through `insert_rec` plays the role of CLRS's `while z.p.color == RED`
    /// loop: case 1 (recolor) may leave `self` red, which the parent's next
    /// call up the stack observes.
    ///
    /// Because `rotate_l`/`rotate_r` swap the two rotated nodes' colors,
    /// case 3 needs no explicit recolor — the rotation alone puts the
    /// grandparent's black on the promoted parent and the parent's red on the
    /// demoted grandparent, exactly the CLRS recolor-and-rotate.
    fn balance_after_insert(&mut self) {
        let l_red = self.l_node().is_red();
        let r_red = self.r_node().is_red();
        if !l_red && !r_red {
            return;
        }
        if l_red {
            // red parent is the left child; uncle is the right child
            if r_red {
                // case 1: uncle red -> recolor parent, uncle, self
                self.flip_colors();
                return;
            }
            // case 2: inner (LR) violation -> rotate the parent left so the
            // red grandchild becomes an outer (LL) grandchild
            if self.l_node().r_node().is_red() {
                self.l_node_mut().rotate_l();
            }
            // case 3: outer (LL) violation -> rotate self right; the color
            // swap recolors the new root black and the demoted self red
            if self.l_node().l_node().is_red() {
                self.rotate_r();
            }
        } else {
            // red parent is the right child; uncle is the left child (mirror)
            if self.r_node().l_node().is_red() {
                self.r_node_mut().rotate_r();
            }
            if self.r_node().r_node().is_red() {
                self.rotate_l();
            }
        }
    }

    fn insert_rec(&mut self, k: K, v: V) -> Option<(K, V)> {
        if self.is_empty() {
            self.set_key_val(k, v);
            return None;
        }
        let node = match self.key_cmp(&k) {
            Ordering::Equal => return self.replace_data((k, v)),
            Ordering::Less => self.l_node_mut(),
            Ordering::Greater => self.r_node_mut(),
        };
        let ret = node.insert_rec(k, v);
        self.balance_after_insert();
        self.update_size();
        ret
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        let ret = self.insert_rec(k, v);
        self.force_black_root();
        ret
    }

    /// Removes `self`'s data, returning it and whether the subtree is now one
    /// black short (a black node was removed). The three node shapes:
    /// leaf, single-child, and two-child. The bool is seeded by the color of
    /// the node being removed here or (for the two-child case) threaded up
    /// from `remove_min`; it is then either absorbed locally or handed to
    /// `balance_after_remove` to propagate further.
    fn remove_core(&mut self) -> (Option<(K, V)>, bool) {
        let (no_l, no_r) = (self.l_node().is_empty(), self.r_node().is_empty());
        if no_l && no_r {
            // leaf: removing a black leaf creates a black deficit; a red one
            // does not.
            let was_red = self.is_red();
            let data = self.take_content().map(|c| c.data);
            return (data, !was_red);
        }
        if no_l || no_r {
            // single child. In a red-black tree a node with exactly one child
            // is black and that child is a red leaf; promote and blacken it,
            // which absorbs any deficit locally.
            let node = if no_l {
                self.r_node_mut()
            } else {
                self.l_node_mut()
            };
            let node_content = node.take_content();
            let data = self.replace_content(node_content).map(|c| c.data);
            self.flip_color_to(false);
            return (data, false);
        }
        // two children: replace self's data with the successor (the min of the
        // right subtree), removing that successor node instead. The deficit,
        // if any, comes from the right subtree's `remove_min`.
        let (next_data, right_deficit) = self.r_node_mut().remove_min();
        let prev_data = self.replace_data(next_data.unwrap());
        let deficit = if right_deficit {
            self.balance_after_remove(Side::Right)
        } else {
            false
        };
        self.update_size();
        (prev_data, deficit)
    }

    /// Removes and returns the minimum (leftmost) node of this subtree, with a
    /// bool saying whether the subtree is now one black short.
    fn remove_min(&mut self) -> (Option<(K, V)>, bool) {
        if self.is_empty() {
            return (None, false);
        }
        if self.l_node().is_empty() {
            // `self` is the minimum node: it has no left child. In a general
            // (not necessarily left-leaning) red-black tree it may still have
            // a right child, which must be promoted — `take_content` would
            // pull the whole subtree and return only `data`, dropping it.
            let was_red = self.is_red();
            let has_right = !self.r_node().is_empty();
            let r_content = self.r_node_mut().take_content();
            let data = self.replace_content(r_content).map(|c| c.data);
            // a red leaf leaves no deficit. A black leaf does. A black node
            // with a right child has a red child (invariant); promoting and
            // blackening it absorbs the deficit.
            if has_right {
                self.flip_color_to(false);
                return (data, false);
            }
            return (data, !was_red);
        }
        let (ret, child_deficit) = self.l_node_mut().remove_min();
        let deficit = if child_deficit {
            self.balance_after_remove(Side::Left)
        } else {
            false
        };
        self.update_size();
        (ret, deficit)
    }

    /// Sets this node's color to `red`, leaving it empty unchanged. Cheaper
    /// than a full flip when the target color is known.
    fn flip_color_to(&mut self, red: bool) {
        if let Some(c) = self.0.as_mut() {
            c.red = red;
        }
    }

    /// Bottom-up red-black delete fixup. `self`'s child on `side` has lost a
    /// black node below (is one black short); the other child — the sibling —
    /// is a valid red-black tree. This is CLRS RB-DELETE-FIXUP, but instead of
    /// a double-black marker re-derived by `black_height` comparison, the
    /// deficit is threaded in as the `side` argument and threaded out as the
    /// return value: `true` means the deficit was not absorbed here and
    /// `self`'s subtree is now one black short, so the parent's own
    /// `balance_after_remove` call must finish the job.
    ///
    /// Uses only O(1) color checks (`is_red`/`is_empty`); no `black_height`.
    fn balance_after_remove(&mut self, side: Side) -> bool {
        if self.is_empty() {
            return false;
        }
        let sibling_red = match side {
            Side::Left => self.r_node().is_red(),
            Side::Right => self.l_node().is_red(),
        };
        if sibling_red {
            // case 1: red sibling -> rotate (the color swap blackens it),
            // moving the deficit down to the now-red child. A red node always
            // absorbs a one-black deficit (case 2) or resolves it by rotation
            // (cases 3-4), so that recursion never propagates back here.
            return match side {
                Side::Left => {
                    self.rotate_l();
                    self.l_node_mut().balance_after_remove(Side::Left)
                }
                Side::Right => {
                    self.rotate_r();
                    self.r_node_mut().balance_after_remove(Side::Right)
                }
            };
        }
        let w_empty = match side {
            Side::Left => self.r_node().is_empty(),
            Side::Right => self.l_node().is_empty(),
        };
        if w_empty {
            // A genuine deficit requires a non-empty sibling to borrow from;
            // empty here means no deficit reached this node. Defensive.
            return false;
        }
        // near = sibling child on the deficit side; far = the other.
        let (near_red, far_red) = match side {
            Side::Left => (
                self.r_node().l_node().is_red(),
                self.r_node().r_node().is_red(),
            ),
            Side::Right => (
                self.l_node().r_node().is_red(),
                self.l_node().l_node().is_red(),
            ),
        };
        if !near_red && !far_red {
            // case 2: both sibling children black -> recolor sibling red; if
            // self is red it absorbs (resolved), else the deficit propagates up.
            match side {
                Side::Left => self.r_node_mut().flip_color(),
                Side::Right => self.l_node_mut().flip_color(),
            }
            if self.is_red() {
                self.flip_color();
                false
            } else {
                true
            }
        } else {
            // case 3: near red, far black -> rotate at sibling so the red
            // nephew becomes the far one, then case 4 resolves it.
            if near_red && !far_red {
                match side {
                    Side::Left => self.r_node_mut().rotate_r(),
                    Side::Right => self.l_node_mut().rotate_l(),
                }
            }
            // case 4: far nephew red -> pull it over; the color-swap rotation
            // settles colors, deficit resolved.
            match side {
                Side::Left => {
                    if self.r_node().r_node().is_red() {
                        self.r_node_mut().r_node_mut().flip_color();
                    }
                    self.rotate_l();
                }
                Side::Right => {
                    if self.l_node().l_node().is_red() {
                        self.l_node_mut().l_node_mut().flip_color();
                    }
                    self.rotate_r();
                }
            }
            false
        }
    }

    fn remove_rec(&mut self, k: &K) -> (Option<(K, V)>, bool) {
        if self.is_empty() {
            return (None, false);
        }
        let (side, node) = match self.key_cmp(k) {
            Ordering::Equal => return self.remove_core(),
            Ordering::Less => (Side::Left, self.l_node_mut()),
            Ordering::Greater => (Side::Right, self.r_node_mut()),
        };
        let (ret, child_deficit) = node.remove_rec(k);
        let deficit = if child_deficit {
            self.balance_after_remove(side)
        } else {
            false
        };
        self.update_size();
        (ret, deficit)
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        let (ret, _deficit) = self.remove_rec(k);
        self.force_black_root();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let node: Node<i32, char> = Node::none();
        assert!(node.is_empty());
        assert_eq!(node.len(), 0);
        assert!(!node.is_red());
    }

    fn pick<'a, K, V>(node: &'a mut Node<K, V>, path: &str) -> &'a mut Node<K, V> {
        let mut ret = node;
        for c in path.chars() {
            let content = ret.0.as_mut().unwrap();
            ret = match c {
                'l' => &mut content.l_node,
                'r' => &mut content.r_node,
                _ => panic!("bad path"),
            };
        }
        ret
    }

    fn black_height_ok<K: Ord, V>(n: &Node<K, V>) -> Option<usize> {
        if n.is_empty() {
            return Some(0);
        }
        // no two red links in a row (red links may lean either way)
        if n.is_red() && (n.l_node().is_red() || n.r_node().is_red()) {
            return None;
        }
        let (lh, rh) = (black_height_ok(n.l_node())?, black_height_ok(n.r_node())?);
        if lh != rh {
            return None;
        }
        if n.len() != 1 + n.l_node().len() + n.r_node().len() {
            return None;
        }
        Some(lh + if n.is_red() { 0 } else { 1 })
    }

    fn collect_keys<K: Ord + Clone, V>(n: &Node<K, V>, out: &mut Vec<K>) {
        if n.is_empty() {
            return;
        }
        collect_keys(n.l_node(), out);
        out.push(n.content().data.0.clone());
        collect_keys(n.r_node(), out);
    }

    fn is_valid<K: Ord + Clone, V>(node: &Node<K, V>) -> bool {
        let mut keys = Vec::new();
        collect_keys(node, &mut keys);
        let ordered = keys.windows(2).all(|w| w[0] < w[1]);
        black_height_ok(node).is_some() && ordered && !node.is_red()
    }

    #[test]
    fn get() {
        let root = &mut Node::none();
        root.set_key_val(20, 'a');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&10), None);

        pick(root, "l").set_key_val(11, 'b');
        pick(root, "r").set_key_val(32, 'c');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));

        pick(root, "ll").set_key_val(10, 'd');
        pick(root, "lr").set_key_val(13, 'e');
        pick(root, "rl").set_key_val(29, 'f');
        pick(root, "rr").set_key_val(34, 'g');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));
        assert_eq!(root.get(&10), Some(&(10, 'd')));
        assert_eq!(root.get(&13), Some(&(13, 'e')));
        assert_eq!(root.get(&29), Some(&(29, 'f')));
        assert_eq!(root.get(&34), Some(&(34, 'g')));
    }

    #[test]
    fn insert() {
        let root = &mut Node::none();
        let items = [(20, 'a'), (15, 'b'), (35, 'c'), (12, 'd'), (36, 'e')];

        for (k, v) in items {
            assert_eq!(root.insert(k, v), None);
            assert!(is_valid(root));
        }
        assert_eq!(root.len(), items.len());
        for (k, v) in items {
            assert_eq!(root.get(&k), Some(&(k, v)));
        }

        // duplicate keys return the old value and update in place
        assert_eq!(root.insert(15, 'B'), Some((15, 'b')));
        assert!(is_valid(root));
        assert_eq!(root.get(&15), Some(&(15, 'B')));
        assert_eq!(root.insert(36, 'E'), Some((36, 'e')));
        assert!(is_valid(root));
        assert_eq!(root.get(&36), Some(&(36, 'E')));
        assert_eq!(root.insert(20, 'A'), Some((20, 'a')));
        assert!(is_valid(root));
        assert_eq!(root.get(&20), Some(&(20, 'A')));
        assert_eq!(root.len(), items.len());
    }

    #[test]
    fn remove() {
        let root = &mut Node::none();
        let items = [
            (20, 'a'),
            (15, 'b'),
            (35, 'c'),
            (11, 'e'),
            (16, 'f'),
            (22, 'g'),
            (41, 'h'),
            (39, 'j'),
            (37, 'k'),
        ];
        for (k, v) in items {
            root.insert(k, v);
            assert!(is_valid(root));
        }
        assert_eq!(root.len(), items.len());

        let to_remove = [16, 15, 35, 41, 20];
        let mut remaining: Vec<(i32, char)> = items.to_vec();
        for k in to_remove {
            let v = root.remove(&k).unwrap();
            assert_eq!(v.0, k);
            assert_eq!(root.get(&k), None);
            assert!(is_valid(root));
            remaining.retain(|(rk, _)| *rk != k);
            assert_eq!(root.len(), remaining.len());
            for (rk, rv) in &remaining {
                assert_eq!(root.get(rk), Some(&(*rk, *rv)));
            }
        }

        // absent key returns None, tree unchanged in content
        assert_eq!(root.remove(&10), None);
        assert!(is_valid(root));
        assert_eq!(root.len(), remaining.len());

        // drain the rest
        let rest: Vec<i32> = remaining.iter().map(|(k, _)| *k).collect();
        for k in rest {
            assert_eq!(root.remove(&k).unwrap().0, k);
            assert!(is_valid(root));
        }
        assert!(root.is_empty());
    }

    #[test]
    fn stress() {
        let root = &mut Node::none();
        let n = 200;
        for k in 0..n {
            assert_eq!(root.insert(k, k), None);
            assert!(is_valid(root));
        }
        assert_eq!(root.len(), n as usize);
        for k in 0..n {
            assert_eq!(root.get(&k), Some(&(k, k)));
        }
        for k in 0..n {
            assert_eq!(root.remove(&k), Some((k, k)));
            assert!(is_valid(root));
        }
        assert!(root.is_empty());
        assert_eq!(root.remove(&0), None);
    }

    /// Deterministic pseudo-random permutation (LCG), so the test is
    /// reproducible without a `rand` dependency.
    fn shuffled(n: u32) -> Vec<u32> {
        let mut v: Vec<u32> = (0..n).collect();
        let mut state: u32 = 0x9e37_79b9;
        for i in (1..n).rev() {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            let j = (state >> 16) as usize % (i as usize + 1);
            v.swap(i as usize, j);
        }
        v
    }

    #[test]
    fn stress_descending() {
        let root = &mut Node::none();
        let n = 200;
        for k in (0..n).rev() {
            assert_eq!(root.insert(k, k), None);
            assert!(is_valid(root));
        }
        for k in (0..n).rev() {
            assert_eq!(root.remove(&k), Some((k, k)));
            assert!(is_valid(root));
        }
        assert!(root.is_empty());
    }

    #[test]
    fn stress_random_order() {
        let root = &mut Node::none();
        let n = 300u32;
        let insert_order = shuffled(n);
        for &k in &insert_order {
            assert_eq!(root.insert(k, k), None);
            assert!(is_valid(root));
        }
        let remove_order = shuffled(n);
        let mut left = n as usize;
        for &k in &remove_order {
            assert_eq!(root.remove(&k), Some((k, k)));
            assert!(is_valid(root));
            left -= 1;
            assert_eq!(root.len(), left);
        }
        assert!(root.is_empty());
    }

    #[test]
    fn stress_interleaved() {
        let root = &mut Node::none();
        // insert 0..100, remove the even keys, then insert 100..150, then
        // remove everything that remains.
        for k in 0..100 {
            root.insert(k, k);
            assert!(is_valid(root));
        }
        for k in (0..100).step_by(2) {
            assert_eq!(root.remove(&k), Some((k, k)));
            assert!(is_valid(root));
        }
        for k in 100..150 {
            root.insert(k, k);
            assert!(is_valid(root));
        }
        let remaining: Vec<i32> = (0..150).filter(|k| k % 2 != 0 || *k >= 100).collect();
        assert_eq!(root.len(), remaining.len());
        for k in remaining {
            assert_eq!(root.remove(&k).unwrap().0, k);
            assert!(is_valid(root));
        }
        assert!(root.is_empty());
    }
}
