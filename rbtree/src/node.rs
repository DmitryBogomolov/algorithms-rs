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

    pub fn get(&self, k: &K) -> Option<&(K, V)> {
        if self.0.is_none() {
            return None;
        }
        let content = self.0.as_ref().unwrap();
        let node = match k.cmp(&content.data.0) {
            Ordering::Equal => {
                return Some(&content.data);
            },
            Ordering::Less => &content.l_node,
            Ordering::Greater => &content.r_node,
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

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        if self.0.is_none() {
            self.set_content(k, v);
            return None;
        }
        let content = self.0.as_mut().unwrap();
        let node = match k.cmp(&content.data.0) {
            Ordering::Equal => {
                let data = std::mem::replace(&mut content.data, (k, v));
                return Some(data);
            },
            Ordering::Less => &mut content.l_node,
            Ordering::Greater => &mut content.r_node,
        };
        node.insert(k, v)
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        if self.0.is_none() {
            return None;
        }
        let content = self.0.as_mut().unwrap();
        let node = match k.cmp(&content.data.0) {
            Ordering::Equal => {
                let mut content = self.0.take().unwrap();
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
                return result;
            },
            Ordering::Less => &mut content.l_node,
            Ordering::Greater => &mut content.r_node,
        };
        node.remove(k)
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

        assert_eq!(root.insert(15, 'b'), None);
        assert_eq!(data(pick(root, "l")), &(15, 'b'));

        assert_eq!(root.insert(35, 'c'), None);
        assert_eq!(data(pick(root, "r")), &(35, 'c'));

        assert_eq!(root.insert(12, 'd'), None);
        assert_eq!(data(pick(root, "ll")), &(12, 'd'));
        
        assert_eq!(root.insert(36, 'e'), None);
        assert_eq!(data(pick(root, "rr")), &(36, 'e'));

        assert_eq!(root.insert(15, 'B'), Some((15, 'b')));
        assert_eq!(data(pick(root, "l")), &(15, 'B'));

        assert_eq!(root.insert(36, 'E'), Some((36, 'e')));
        assert_eq!(data(pick(root, "rr")), &(36, 'E'));

        assert_eq!(root.insert(20, 'A'), Some((20, 'a')));
        assert_eq!(data(root), &(20, 'A'));
    }

    #[test]
    fn remove() {
        let root = &mut Node::none();
        root.set_content(20, 'a');
        pick(root, "l").set_content(15, 'b');
        pick(root, "r").set_content(35, 'c');
        pick(root, "ll").set_content(11, 'e');
        pick(root, "lr").set_content(16, 'f');
        pick(root, "rl").set_content(22, 'g');
        pick(root, "rr").set_content(41, 'h');
        pick(root, "rrl").set_content(39, 'j');
        pick(root, "rrll").set_content(37, 'k');

        assert_eq!(root.remove(&16), Some((16, 'f')));
        assert!(pick(root, "l").0.as_ref().unwrap().r_node.is_empty());

        assert_eq!(root.remove(&15), Some((15, 'b')));
        assert_eq!(data(pick(root, "l")), &(11, 'e'));

        assert_eq!(root.remove(&35), Some((35, 'c')));
        assert_eq!(data(pick(root, "r")), &(37, 'k'));
        assert!(pick(root, "rrl").0.as_ref().unwrap().l_node.is_empty());

        assert_eq!(root.remove(&10), None);
    }
}
