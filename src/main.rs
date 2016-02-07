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
}

struct BinaryTree {
    root: Option<Box<Node>>
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
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
    fn get(&self, key: usize) -> Option<u32> {
        match self.root {
            None => None,
            Some(ref box_node) => {
                let ref node = *box_node;
                node.get(key)
            },
        }
    }
}

fn main() {
    let mut b = BinaryTree::new();
    for i in 0..10 {
        b.put(i, i as u32);
    }
    for i in 0..10 {
        assert_eq!(Some(i as u32), b.get(i));
    }
}
