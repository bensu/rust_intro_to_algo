// binary tree
#![feature(box_patterns)]

use std::mem;

struct Node {
    key: usize,
    val: u32,
    left: BinaryTree,
    right: BinaryTree,
}

impl Node {
    fn new(key: usize, val: u32) -> Self {
        Node {
            key: key,
            val: val,
            left: BinaryTree::new(),
            right: BinaryTree::new(),
        }
    }
    fn put(&mut self, key: usize, val: u32) {
        if self.key == key {
            self.val = val;
        } else if key < self.key {
            self.left.put(key, val);
        } else {
            self.right.put(key, val);
        }
    }
    fn get(&self, key: usize) -> Option<u32> {
        if self.key == key {
            Some(self.val)
        } else if key < self.key {
            self.left.get(key)
        } else {
            self.right.get(key)
        }
    }
    fn min_key(&self) -> Option<usize> {
        match self.left.min_key() {
            None => Some(self.key),
            Some(key) => Some(key),
        }
    }
    fn max_key(&self) -> Option<usize> {
        match self.right.max_key() {
            None => Some(self.key),
            Some(key) => Some(key),
        }
    }
}

struct BinaryTree {
    root: Option<Box<Node>>
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }
    fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            _ => false,
        }
    }
    fn put(&mut self, key: usize, val: u32) {
        match mem::replace(&mut self.root, None) {
            None => {
                self.root = Some(Box::new(Node::new(key, val)));
            },
            Some(box_node) => {
                let mut node = *box_node;
                node.put(key,val);
                self.root = Some(Box::new(node));
            }
        }
    }
    // Maybe monad much?
    fn get(&self, key: usize) -> Option<u32> {
        match self.root {
            None => None,
            Some(ref box_node) => {
                let ref node = *box_node;
                node.get(key)
            },
        }
    }
    fn min_key(&self) -> Option<usize> {
        match self.root {
            None => None,
            Some(ref box_node) => {
                let ref node = *box_node;
                node.min_key()
            }
        }
    }
    fn max_key(&self) -> Option<usize> {
        match self.root {
            None => None,
            Some(ref box_node) => {
                let ref node = *box_node;
                node.max_key()
            }
        }
    }
}

fn main() {
    let mut b = BinaryTree::new();
    assert!(b.is_empty());
    for i in 0..10 {
        b.put(i, i as u32);
        assert_eq!(Some(0), b.min_key());
        assert!(!b.is_empty());
        assert_eq!(Some(i), b.max_key());
    }
    for i in 0..10 {
        assert_eq!(Some(i as u32), b.get(i));
    }
}
