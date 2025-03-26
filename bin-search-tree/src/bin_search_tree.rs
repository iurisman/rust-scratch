use std::option::Option::*;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::ops::Deref;
use rand::{Rng};
use crate::either::Either;
use crate::either::Either::{Right, Left};

#[derive(Debug)]
struct Node<T:Ord> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl <T:Ord> Node<T> {

    fn new(t: T) -> Node<T> {
        Node { data: t, left: None, right: None }
    }
}

#[derive(Debug)]
struct BinSearchTree<T: Ord>(Option<Box<Node<T>>>);

impl <T: Ord>BinSearchTree<T> {

    /// New instance of tree node.
    pub fn new() -> BinSearchTree<T>{BinSearchTree(None)}

}

#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::bin_search_tree::BinSearchTree;
    #[test]
    fn test_insert() {
        let mut tree = BinSearchTree::<i32>::new();
    }
}
