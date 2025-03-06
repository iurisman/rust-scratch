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

    fn insert(&mut self, t: T) {
        match self.data.cmp(&t) {
            Less => match  &mut self.right {
                None => self.right = Some(Box::new(Node::new(t))),
                Some(ref mut right) => {
                    let foo = right.as_mut();
                    foo.insert(t);
                }
            }
            Greater => match &self.left {
                None => self.left = Some(Box::new(Node::new(t))),
                Some(_) => panic!("Cannot insert a left twice"),
            }
            Equal => println!("Ignored duplicate key: {:?}", t),
        }
    }
}

struct BinSearchTree<T: Ord + Copy>(Option<Node<T>>);

impl <T: Ord + Copy + Debug>BinSearchTree<T> {
    pub fn new() -> BinSearchTree<T>{BinSearchTree(None)}


    pub fn is_empty(&self) -> bool { self.0.is_none() }

    pub fn insert(&mut self, t: T) -> () {
        match &mut self.0 {
            None => self.0 = Some(Node::new(t)),
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
        tree.insert(10);
        assert!(!tree.is_empty());
        tree.insert(11);
        tree.insert(11);
    }
}