use std::mem;

pub trait Stack {
    fn push(&mut self, u32);
    fn pop(&mut self) -> Option<u32>;
    fn is_empty(&self) -> bool;
}

pub struct List {
    head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem: u32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

/* The patter used with mem::replace is useful whenever the current value of a
 * mutable reference is need to construct the next one. At some point,
 * we will need to extract its value without borrowing it, use the
 * value to construct the next value, and assign the next value to the
 * same reference. There is an intermediate step (while the next value
 * is being produced) where the mutable reference needs to have a
 * dummy holder.
 */
impl Stack for List {
    fn push(self: &mut List, t: u32) {
        // self is a mutable reference (borrowed value)
        let new_node = Node {
            elem: t,
            /* if we pass it somehow to .next, it'd be transferred,
             * and no longer useful in the scope (we need to user later!).
             * We put List::Empty in self.head, and put the old self.head
             * in next (what we want!) */
            next: mem::replace(&mut self.head, Link::Empty),
        };
        // We now overwrite the List::Empty with the new node (which
        // contains in it the old self.head)
        self.head = Link::More(Box::new(new_node));
    }
    fn pop(self: &mut List) -> Option<u32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(box_node) => {
                let node = *box_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    fn is_empty(self: &List) -> bool {
        match self.head {
            Link::Empty => true,
            _ => false,
        }
    }
}

const N: usize = 10;

struct ArrayStack {
    n: usize,
    s: [Option<u32>; N],
}

impl ArrayStack {
    pub fn new() -> Self {
        ArrayStack {
            n: 0,
            s: [None; N],
        }
    }
}

impl Stack for ArrayStack {
    fn push(&mut self, ele: u32) {
        self.s[self.n] = Some(ele);
        self.n = self.n + 1;
        assert!(self.n < N);
    }
    fn pop(&mut self) -> Option<u32> {
        if self.n != 0 {
            self.n = self.n - 1;
        }
        let out = self.s[self.n];
        out
    }
    fn is_empty(&self) -> bool {
        self.n == 0
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use super::Stack;
    use super::ArrayStack;
    fn test_push_pop<T: Stack>(s: &mut T) {
        assert!(s.is_empty());
        assert_eq!(None, s.pop());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(Some(3), s.pop());
        assert_eq!(Some(2), s.pop());
        assert_eq!(Some(1), s.pop());
        assert!(s.is_empty());
        for i in 0..15 {
            s.push(i);
        }
    }
    #[test]
    fn list_stack() {
        let mut list_stack = List::new();
        test_push_pop(&mut list_stack);
    }
    #[test]
    fn array_stack() {
        let mut array_stack = ArrayStack::new();
        test_push_pop(&mut array_stack);
    }
}

fn main() {
    /* Implement Stacks
     */
}
