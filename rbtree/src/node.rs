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

    fn take<'a, K, V>(node: &'a mut Node<K, V>, path: &str) -> &'a mut Node<K, V> {
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

    #[test]
    fn get() {
        let mut root: Node<i32, char> = Node::none();
        root.set_content(20, 'a');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&10), None);

        take(&mut root, "l").set_content(11, 'b');
        take(&mut root, "r").set_content(32, 'c');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));

        take(&mut root, "ll").set_content(10, 'd');
        take(&mut root, "lr").set_content(13, 'e');
        take(&mut root, "rl").set_content(29, 'f');
        take(&mut root, "rr").set_content(34, 'g');

        assert_eq!(root.get(&20), Some(&(20, 'a')));
        assert_eq!(root.get(&11), Some(&(11, 'b')));
        assert_eq!(root.get(&32), Some(&(32, 'c')));
        assert_eq!(root.get(&10), Some(&(10, 'd')));
        assert_eq!(root.get(&13), Some(&(13, 'e')));
        assert_eq!(root.get(&29), Some(&(29, 'f')));
        assert_eq!(root.get(&34), Some(&(34, 'g')));
    }
}
