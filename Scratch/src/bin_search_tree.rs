use std::option::Option::*;
use std::cmp::Ordering::*;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T:Ord + Copy> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T:Ord + Copy + Debug> Node<T> {

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
}

struct BinSearchTree<T: Ord + Copy>(Option<Node<T>>);

impl <T: Ord + Copy + Debug>BinSearchTree<T> {
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
}

#[cfg(test)]
mod tests {
    use crate::bin_search_tree::BinSearchTree;

    #[test]
    fn test_bin_search_tree() {
        let mut tree: BinSearchTree<i32> = BinSearchTree::new();
        assert!(tree.is_empty());
        assert!(tree.insert(10));
        assert!(!tree.insert(10));
        assert!(!tree.is_empty());
        assert!(tree.insert(11));
        assert!(!tree.insert(11));
        assert!(!tree.insert(10));cd
    }
}