use std::cmp::Ord;

pub struct Tree<T>(Option<Node<T>>)
where
    T: Ord;

#[derive(PartialEq)]
pub struct Node<T>
where
    T: Ord,
{
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }
}

impl<T> From<T> for Tree<T>
where
    T: Ord,
{
    fn from(value: T) -> Self {
        Tree(Some(Node::new(value)))
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Tree(None)
    }

    pub fn get(&self, value: T) -> Option<T> {
        let mut node = match &self.0 {
            Some(node) => node,
            None => return None,
        };

        loop {
            if value < node.value {
                node = match &node.left {
                    Some(n) => &n,
                    None => return None,
                };
            } else if value > node.value {
                node = match &node.right {
                    Some(n) => &n,
                    None => return None,
                };
            } else {
                return Some(value);
            }
        }
    }

    pub fn insert(&mut self, value: T) -> () {
        let mut node: &mut Node<T> = match &mut self.0 {
            Some(node) => node,
            None => {
                *self = Tree(Some(Node::new(value)));
                return;
            }
        };

        loop {
            if value < node.value {
                match &mut node.left {
                    Some(n) => node = n,
                    n @ None => {
                        *n = Some(Box::new(Node::new(value)));
                        return;
                    }
                }
            } else if value > node.value {
                match &mut node.right {
                    Some(n) => node = n,
                    n @ None => {
                        *n = Some(Box::new(Node::new(value)));
                        return;
                    }
                }
            } else {
                // the value is already present
                return ();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_if_empty_tree() {
        let n = Tree::new();
        assert_eq!(n.get(42), None);
    }

    #[test]
    fn get_returns_none_if_not_exists() {
        let n = Tree::from(5);
        assert_eq!(n.get(42), None);
    }

    #[test]
    fn get_returns_existing_value() {
        let n = Tree::from(5);
        assert_eq!(n.get(5), Some(5));
    }

    #[test]
    fn insert_then_get() {
        let mut n = Tree::new();
        n.insert(5);
        assert_eq!(n.get(5), Some(5));
    }

    #[test]
    fn string_tree_insert_then_get() {
        let mut n = Tree::new();
        n.insert("hello");
        assert_eq!(n.get("hello"), Some("hello"));
    }
}
