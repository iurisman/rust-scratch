use crate::bin_search_tree::BinSearchTree;

mod bin_search_tree;
fn main() {
    let mut tree: BinSearchTree<i32> = BinSearchTree::new();
    assert!(tree.is_empty());
    assert!(tree.insert(10));
    assert!(!tree.insert(10));
    assert!(!tree.is_empty());
    assert!(tree.insert(11));
    assert!(!tree.insert(11));
    assert!(!tree.insert(10));

}
