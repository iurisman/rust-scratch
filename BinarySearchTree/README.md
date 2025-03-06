# binary_search_tree

* [Documentation](https://docs.rs/binary_search_tree/)
* [Crate](https://crates.io/crates/binary_search_tree)


## Description

<p>A classic Binary Search Tree written in Rust.</p>

In this implementation, each node of the binary tree contains only one valuable value. To order the nodes, the elements must implement the ```Ord``` trait.


## Usage

As a library
```rust
extern crate binary_search_tree;

use binary_search_tree::BinarySearchTree;
```


## Features & capabilities

The BinarySearchTree struct provides the following methods: 
* [Viewing the root element](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.root)
* [Is the tree empty](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.is_empty)
* [Insertion](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.insert)
* [Insertion without duplicating](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.insert_without_dup)
* [Check for the presence of an element in the tree](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.contains)
* [Viewing the minimum](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.min)
* [Viewing the maximum](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.max)
* [Extracting the minimum](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.extract_min)
* [Extracting the maximum](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.extract)
* [Deleting an arbitrary value](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.remove)
* [Successor](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.successor)
* [Predecessor](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.predecessor)
* [Viewing the number of items in the tree](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.len)
* [Clearing the tree](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.clear)
* [Viewing values in the tree in ascending order](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.sorted_vec)
* [Moving the tree to a sorted vector](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.into_sorted_vec)
* [Creating a tree with elements from an iterator](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.from_iter)
* [Extending the tree with elements from the iterator](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.extend)
* [Inorder traversal](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.inorder)
* [Reverse order traversal](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.reverse_order)
* [Preorder traversal](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.preorder)
* [Postorder traversal](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.postorder)
* [Level order traversal](https://docs.rs/binary_search_tree/0.2.2/binary_search_tree/struct.BinarySearchTree.html#method.level_order)


If you have any comments or suggestions, or you suddenly found an error, please write to prototyperailgun@gmail.com.
