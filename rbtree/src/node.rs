use std::cmp::Ordering;

pub struct Node<K, V>(Option<Box<Content<K, V>>>);

enum Clr {
    R,
    B,
}

struct Content<K, V> {
    l_node: Node<K, V>,
    r_node: Node<K, V>,
    clr: Clr,
    size: usize,
    data: (K, V),
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

    fn update_size(&mut self) {
        let content = self.content_mut();
        content.size = 1 + content.l_node.len() + content.r_node.len();
    }

    fn key_to_self(&self, k: &K) -> Ordering {
        k.cmp(&self.content().data.0)
    }

    fn content(&self) -> &Content<K, V> {
        self.0.as_ref().unwrap()
    }

    fn content_mut(&mut self) -> &mut Content<K, V> {
        self.0.as_mut().unwrap()
    }

    pub fn get(&self, k: &K) -> Option<&(K, V)> {
        if self.is_empty() {
            return None;
        }
        let node = match self.key_to_self(k) {
            Ordering::Equal => {
                return Some(&self.content().data);
            },
            Ordering::Less => &self.content().l_node,
            Ordering::Greater => &self.content().r_node,
        };
        node.get(k)
    }

    fn set_kev_val(&mut self, k: K, v: V) {
        self.0 = Some(Box::new(Content {
            l_node: Self::none(),
            r_node: Self::none(),
            clr: Clr::B,
            size: 1,
            data: (k, v),
        }));
    }

    fn take_content(&mut self) -> Box<Content<K, V>> {
        self.0.take().unwrap()
    }

    fn replace_content(&mut self, content: Box<Content<K, V>>) -> Box<Content<K, V>> {
        self.0.replace(content).unwrap()
    }

    fn insert_core(&mut self, k: K, v: V) -> Option<(K, V)> {
        let data = std::mem::replace(&mut self.content_mut().data, (k, v));
        return Some(data);
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        if self.is_empty() {
            self.set_kev_val(k, v);
            return None;
        }
        let node = match self.key_to_self(&k) {
            Ordering::Equal => {
                return self.insert_core(k, v);
            },
            Ordering::Less => &mut self.content_mut().l_node,
            Ordering::Greater => &mut self.content_mut().r_node,
        };
        let ret = node.insert(k, v);
        self.update_size();
        ret
    }

    fn remove_core(&mut self) -> Option<(K, V)> {
        let (no_l, no_r) = (self.content().l_node.is_empty(), self.content().r_node.is_empty());
        if no_l && no_r {
            let content = self.take_content();
            return Some(content.data);
        }
        if no_l || no_r {
            let node = if no_l { &mut self.content_mut().r_node } else { &mut self.content_mut().l_node };
            let node_content = node.take_content();
            let content = self.replace_content(node_content);
            return Some(content.data);
        }
        let next_data = self.content_mut().r_node.remove_min().unwrap();
        self.update_size();
        let data = std::mem::replace(&mut self.content_mut().data, next_data);
        return Some(data);
    }

    fn remove_min(&mut self) -> Option<(K, V)> {
        if self.is_empty() {
            return None;
        }
        let node = &mut self.content_mut().l_node;
        if node.is_empty() {
            return Some(self.take_content().data);
        }
        let ret = node.remove_min();
        self.update_size();
        ret
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        if self.is_empty() {
            return None;
        }
        let node = match self.key_to_self(k) {
            Ordering::Equal => {
                return self.remove_core();
            },
            Ordering::Less => &mut self.content_mut().l_node,
            Ordering::Greater => &mut self.content_mut().r_node,
        };
        let ret = node.remove(k);
        self.update_size();
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

    fn data<K, V>(node: &Node<K, V>) -> &(K, V) {
        &node.0.as_ref().unwrap().data
    }

    #[test]
    fn get() {
        let root = &mut Node::none();
        root.set_kev_val(20, 'a');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&10), None);

        pick(root, "l").set_kev_val(11, 'b');
        pick(root, "r").set_kev_val(32, 'c');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));

        pick(root, "ll").set_kev_val(10, 'd');
        pick(root, "lr").set_kev_val(13, 'e');
        pick(root, "rl").set_kev_val(29, 'f');
        pick(root, "rr").set_kev_val(34, 'g');

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

        assert_eq!(root.insert(20, 'a'), None);
        assert_eq!(data(root), &(20, 'a'));
        assert!(!root.is_empty());
        assert_eq!(root.len(), 1);

        assert_eq!(root.insert(15, 'b'), None);
        assert_eq!(data(pick(root, "l")), &(15, 'b'));
        assert_eq!(root.len(), 2);

        assert_eq!(root.insert(35, 'c'), None);
        assert_eq!(data(pick(root, "r")), &(35, 'c'));
        assert_eq!(root.len(), 3);

        assert_eq!(root.insert(12, 'd'), None);
        assert_eq!(data(pick(root, "ll")), &(12, 'd'));
        assert_eq!(root.len(), 4);

        assert_eq!(root.insert(36, 'e'), None);
        assert_eq!(data(pick(root, "rr")), &(36, 'e'));
        assert_eq!(root.len(), 5);

        assert_eq!(root.insert(15, 'B'), Some((15, 'b')));
        assert_eq!(data(pick(root, "l")), &(15, 'B'));
        assert_eq!(root.len(), 5);

        assert_eq!(root.insert(36, 'E'), Some((36, 'e')));
        assert_eq!(data(pick(root, "rr")), &(36, 'E'));
        assert_eq!(root.len(), 5);

        assert_eq!(root.insert(20, 'A'), Some((20, 'a')));
        assert_eq!(data(root), &(20, 'A'));
        assert_eq!(root.len(), 5);
    }

    #[test]
    fn remove() {
        let root = &mut Node::none();
        [
            (20, 'a'),
            (15, 'b'),
            (35, 'c'),
            (11, 'e'),
            (16, 'f'),
            (22, 'g'),
            (41, 'h'),
            (39, 'j'),
            (37, 'k'),
        ].into_iter().for_each(|(k, v)| { root.insert(k, v); });

        assert_eq!(root.len(), 9);

        assert_eq!(root.remove(&16), Some((16, 'f')));
        assert_eq!(root.len(), 8);
        assert!(pick(root, "l").0.as_ref().unwrap().r_node.is_empty());

        assert_eq!(root.remove(&15), Some((15, 'b')));
        assert_eq!(root.len(), 7);
        assert_eq!(data(pick(root, "l")), &(11, 'e'));

        assert_eq!(root.remove(&35), Some((35, 'c')));
        assert_eq!(root.len(), 6);
        assert_eq!(data(pick(root, "r")), &(37, 'k'));
        assert!(pick(root, "rrl").0.as_ref().unwrap().l_node.is_empty());

        assert_eq!(root.remove(&10), None);
        assert_eq!(root.len(), 6);
    }
}
