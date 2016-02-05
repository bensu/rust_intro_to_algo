#![feature(ptr_as_ref)]

use std::ptr;
use std::mem;
use std::fmt;

struct RawLink<T> { p: *mut T }

impl<T> Copy for RawLink<T> {}
impl<T> Clone for RawLink<T> {
    #[inline]
    fn clone(&self) -> RawLink<T> {
        RawLink { p: self.p }
    }
}

impl<T> RawLink<T> {
    // Like Option::None for RawLink
    fn none() -> RawLink<T> {
        RawLink { p: ptr::null_mut() }
    }
    // Like Option::Some(T) for RawLink
    fn some(n: &mut T) -> RawLink<T> {
        RawLink { p: n }
    }
    // Transform to Option
    unsafe fn resolve<'a>(&self) -> Option<&'a T> {
        self.p.as_ref()
    }
    unsafe fn resolve_mut<'a>(&mut self) -> Option<&'a mut T> {
        self.p.as_mut()
    }
    fn take(&mut self) -> RawLink<T> {
        mem::replace(self, RawLink::none())
    }
}

impl<'a> From<&'a mut Link> for RawLink<Node> {
    fn from(node: &'a mut Link) -> Self {
        match node.as_mut() {
            None => RawLink::none(),
            Some(ptr) => RawLink::some(ptr),
        }
    }
}

struct Node {
    next: Link,
    prev: RawLink<Node>,
    value: u32,
}

impl Node {
    fn new(n: u32) -> Node {
        Node {
            next: None,
            prev: RawLink::none(),
            value: n,
        }
    }
    fn set_next(&mut self, mut next: Box<Node>) {
        assert!(self.next.is_none());
        next.prev = RawLink::some(self);
        self.next = Some(next);
    }
}

// using Option instead of introducing Nil or Empty
type Link = Option<Box<Node>>;

pub struct LinkedList {
    length: usize,
    head: Link,
    tail: RawLink<Node>,
}

fn debug_link(link: &Link, f: &mut fmt::Formatter) {
    match *link {
        None => (),
        Some(ref box_node) => {
            write!(f, "{}, ", box_node.value);
            debug_link(&box_node.next, f);
        },
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut done = false;
        write!(f, "(");
        debug_link(&self.head, f);
        write!(f, ")")
    }
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: RawLink::none(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false,
        }
    }
    pub fn push(&mut self, n: u32) {
        match mem::replace(&mut self.head, None) {
            None => {
                // List is empty
                let mut node = Node::new(n);
                self.tail = RawLink::some(&mut node);
                self.head = Some(Box::new(node));
            },
            Some(box_node) => {
                let node = Node {
                    value: n,
                    prev: RawLink::none(),
                    next: Some(box_node),
                };
                self.head = Some(Box::new(node));
            },
        }
    }
    pub fn append(&mut self, n: u32) {
        let mut new_tail = Box::new(Node::new(n));
        match unsafe { self.tail.resolve_mut() } {
            None => {
                // List is Empty
                self.tail = RawLink::some(&mut new_tail);
                self.head = Some(new_tail);
            },
            Some(tail_node) => {
                tail_node.set_next(new_tail);
                self.tail = RawLink::from(&mut tail_node.next);
            },
        }
    }
    pub fn pop(&mut self) -> Option<u32> {
        match mem::replace(&mut self.head, None) {
            None => None, // The list is empty
            Some(boxed_node) => {
                let val = boxed_node.value; // Copy the value
                match boxed_node.next {
                    None => {
                        // We are popping the last node
                        self.head = None; // Already by mem::replace
                        self.tail = RawLink::none();
                    },
                    Some(boxed_next_node) => {
                        self.head = Some(boxed_next_node);
                    }
                }
                Some(val)
            },
        }
    }
}

pub trait Queue {
    fn is_empty(&self) -> bool;
    fn queue(&mut self, u32);
    fn dequeue(&mut self) -> Option<u32>;
}

impl Queue for LinkedList {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn queue(&mut self, n: u32) {
        self.append(n);
    }
    fn dequeue(&mut self) -> Option<u32> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;
    use super::Queue;

    #[test]
    fn test_queue() {
        let mut q = LinkedList::new();
        assert!(q.is_empty());
        for i in 0..10 {
            q.queue(i);
        }
        for i in 0..10 {
            assert_eq!(Some(i), q.dequeue());
        }
        assert!(q.is_empty());
    }
}
