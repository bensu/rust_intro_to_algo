/// binary heap

mod util;

extern crate rand; // used for testing
use self::rand::random;

const N: usize = 10;

#[derive(Debug)]
struct BinaryHeap {
    a: [u32; N],
    n: usize,
}

impl BinaryHeap {
    fn new() -> Self {
        BinaryHeap {
            a: [0; N],
            n: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn parent(&self, i: usize) -> usize {
        if i == 1 {
            panic!("already at root!");
        }
        i/2
    }

    fn first_child(&self, i: usize) -> usize {
        assert!(i != 0);
        let out = 2*i;
        assert!(out <= self.n);
        out
    }

    fn second_child(&self, i :usize) -> Option<usize> {
       assert!(i != 0);
       let out = 2*i + 1;
       if self.n < out {
           None
       } else {
           Some(out)
       }
    }

    fn swim(&mut self, i: usize) {
        // do nothing for root
        if i != 1 {
            let e = self.a[i];
            let pi = self.parent(i);
            let p = self.a[pi];
            if p < e {
                self.a[pi] = e;
                self.a[i] = p;
                self.swim(pi);
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < j);
        let min = self.a[i];
        let max = self.a[j];
        self.a[i] = max;
        self.a[j] = min;
    }

    fn sink(&mut self, i: usize) {
        // do nothing for last level
        if 2*i <= self.n {
            let e = self.a[i];
            let fi = self.first_child(i);
            let fc = self.a[fi];
            match self.second_child(i) {
                None => {
                    if e < fc {
                        self.swap(i,fi);
                        self.sink(fi);
                    }
                },
                Some(si) => {
                    let sc = self.a[si];
                    if e < fc && sc <= fc {
                        self.swap(i,fi);
                        self.sink(fi);
                    } else if e < sc && fc < sc {
                        self.swap(i,si);
                        self.sink(si);
                    }
                }
            }
        }
    }

    fn insert(&mut self, elem: u32) {
        self.n = self.n + 1;
        self.a[self.n] = elem;
        let elem_index = self.n;
        self.swim(elem_index);
    }

    fn remove_max(&mut self) -> u32 {
        assert!(!self.is_empty());
        let max = self.peek_max();
        self.a[1] = self.a[self.n];
        // delete last node
        self.a[self.n] = 0;
        self.n = self.n - 1;
        self.sink(1);
        max
    }

    fn peek_max(&self) -> u32 {
        assert!(!self.is_empty());
        self.a[1]
    }
}

fn step_test() {
    let mut h = BinaryHeap::new();
    h.insert(1);
    assert_eq!(1, h.peek_max());
    h.insert(2);
    assert_eq!(2, h.peek_max());
    h.insert(1);
    assert_eq!(2, h.peek_max());
    h.insert(3);
    assert_eq!(3, h.peek_max());
    assert_eq!(3, h.remove_max());
    h.insert(3);
    h.insert(2);
    h.insert(5);
    assert_eq!(5, h.remove_max());
    assert_eq!(3, h.remove_max());
    assert_eq!(2, h.remove_max());
    assert_eq!(2, h.remove_max());
    assert_eq!(1, h.remove_max());
    assert_eq!(1, h.remove_max());
    assert!(h.is_empty());
}


pub fn rand_vec(n: usize) -> Vec<u32> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(util::rand_upto(n) as u32);
    }
    v
}

fn gen_test() {
    step_test();
    let mut v = rand_vec(N-1);
    let mut h = BinaryHeap::new();
    for e in v.iter() {
        h.insert(*e);
    }
    v.sort();
    v.reverse();
    for e in v.iter() {
        assert_eq!(*e, h.remove_max());
    }
}
