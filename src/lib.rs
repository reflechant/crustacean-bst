use std::cmp::Ord;

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
    pub fn new(value: T) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    pub fn get(&self, value: T) -> Option<T> {
        let mut node = self;

        loop {
            if value < node.value {
                node = match &node.left {
                    Some(n) => &n,
                    None => return None,
                };
            } else if value > node.value {
                node = match &node.left {
                    Some(n) => &n,
                    None => return None,
                };
            } else {
                return Some(value);
            }
        }

        None
    }

    pub fn insert(&self, value: T) -> () {
        todo!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_if_not_exists() {
        let n = Node::new(5);
        assert_eq!(n.get(42), None);
    }

    #[test]
    fn get_returns_existing_value() {
        let n = Node::new(5);
        assert_eq!(n.get(5), Some(5));
    }
}
