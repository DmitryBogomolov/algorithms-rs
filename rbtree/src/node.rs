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

    fn cmp_key(&self, k: &K) -> Ordering {
        k.cmp(&self.0.as_ref().unwrap().data.0)
    }

    fn content(&self) -> &Content<K, V> {
        self.0.as_ref().unwrap()
    }

    fn content_mut(&mut self) -> &mut Content<K, V> {
        self.0.as_mut().unwrap()
    }

    pub fn get(&self, k: &K) -> Option<&(K, V)> {
        if self.0.is_none() {
            return None;
        }
        let node = match self.cmp_key(k) {
            Ordering::Equal => {
                return Some(&self.content().data);
            },
            Ordering::Less => &self.content().l_node,
            Ordering::Greater => &self.content().r_node,
        };
        node.get(k)
    }

    fn set_content(&mut self, k: K, v: V) {
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

    fn insert_core(&mut self, k: K, v: V) -> Option<(K, V)> {
        let data = std::mem::replace(&mut self.content_mut().data, (k, v));
        return Some(data);
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        if self.0.is_none() {
            self.set_content(k, v);
            return None;
        }
        let node = match self.cmp_key(&k) {
            Ordering::Equal => {
                return self.insert_core(k, v);
            },
            Ordering::Less => &mut self.content_mut().l_node,
            Ordering::Greater => &mut self.content_mut().r_node,
        };
        let ret = node.insert(k, v);
        if ret.is_none() {
            self.0.as_mut().unwrap().size += 1;
        }
        ret
    }

    fn remove_core(&mut self) -> Option<(K, V)> {
        let mut content = self.take_content();
        let result = Some(content.data);
        if content.l_node.is_empty() && content.r_node.is_empty() {
            return result;
        }
        if content.l_node.is_empty() {
            self.0.replace(content.r_node.0.take().unwrap());
            return result;
        }
        if content.r_node.is_empty() {
            self.0.replace(content.l_node.0.take().unwrap());
            return result;
        }
        let mut target = &mut content.r_node;
        while !target.0.as_mut().unwrap().l_node.is_empty() {
            target = &mut target.0.as_mut().unwrap().l_node;
        }
        self.0.replace(target.0.take().unwrap());
        self.0.as_mut().unwrap().l_node = content.l_node;
        self.0.as_mut().unwrap().r_node = content.r_node;
        self.0.as_mut().unwrap().size = content.size - 1;
        return result;        
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        if self.0.is_none() {
            return None;
        }
        let node = match self.cmp_key(k) {
            Ordering::Equal => {
                return self.remove_core();
            },
            Ordering::Less => &mut self.content_mut().l_node,
            Ordering::Greater => &mut self.content_mut().r_node,
        };
        let ret = node.remove(k);
        if !ret.is_none() {
            self.0.as_mut().unwrap().size -= 1;
        }
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
        root.set_content(20, 'a');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&10), None);

        pick(root, "l").set_content(11, 'b');
        pick(root, "r").set_content(32, 'c');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));

        pick(root, "ll").set_content(10, 'd');
        pick(root, "lr").set_content(13, 'e');
        pick(root, "rl").set_content(29, 'f');
        pick(root, "rr").set_content(34, 'g');

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
