use std::option::Option::*;
use std::cmp::Ordering::*;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T:Ord> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T:Ord + Debug> Node<T> {

    fn new(t: T) -> Node<T> {Node { data:t, left: None, right: None } }

    fn insert(&mut self, t: T) -> bool {
        match self.data.cmp(&t) {
            Less => match  &mut self.right {
                None => {
                    self.right = Some(Box::new(Node::new(t)));
                    true
                },
                Some(ref mut right) => {
                    right.as_mut().insert(t)
                }
            }
            Greater => match &mut self.left {
                None => {
                    self.left = Some(Box::new(Node::new(t)));
                    true
                },
                Some(ref mut left) => {
                    left.as_mut().insert(t)
                }
            }
            Equal => false
        }
    }

    fn size(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.size()) +
            self.right.as_ref().map_or(0, |right| right.size()) + 1
    }
}

struct BinSearchTree<T: Ord>(Option<Node<T>>);

impl <T: Ord + Debug>BinSearchTree<T> {
    pub fn new() -> BinSearchTree<T>{BinSearchTree(None)}

    /// New instance of tree node.
    pub fn is_empty(&self) -> bool { self.0.is_none() }

    /// Insert a new key in the tree.
    /// Returns `true` if the insert was successful, or `false` if the key already exists.
    pub fn insert(&mut self, t: T) -> bool {
        match &mut self.0 {
            None => {
                self.0 = Some(Node::new(t));
                true
            },
            Some(r) => r.insert(t)
        }
    }

    pub fn size(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.size())
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::bin_search_tree::BinSearchTree;
    #[test]
    fn test() {
        let mut tree: BinSearchTree<i32> = BinSearchTree::new();
        assert!(tree.is_empty());
        assert!(tree.insert(10));
        assert!(!tree.insert(10));
        assert!(!tree.is_empty());
        assert!(tree.insert(11));
        assert!(!tree.insert(11));
        assert!(!tree.insert(10));
        assert!(tree.insert(8));
        assert!(!tree.insert(8));
        assert_eq!(3, tree.size());

        let mut rand = rand::rng();
        let mut keys: Vec<i32> = Vec::new();

        // Ensure all inserted values are in the tree.
        for _ in 0..1000 {
            let key = rand.random::<i32>() % 1000;
            keys.push(key);
            tree.insert(key);
        }

        // Use iter() because we will need keys again.
        for key in keys.iter() {
            assert!(!tree.insert(*key));
        }

        // Ensure that none of a number of other values are in the list.
        let mut non_keys: Vec<i32> = Vec::new();
        while non_keys.len() < 1000 {
            let key = rand.random::<i32>() % 1000;
            if !keys.contains(&key) && !non_keys.contains(&key) {
                non_keys.push(key);
                assert!(tree.insert(key), "Key {key} is not in the keys list but insert() returned false");
            }
        }
        assert_eq!(2000, non_keys.len());
    }
}
