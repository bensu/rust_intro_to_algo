
extern crate rand; // used for testing
use self::rand::random;

// Helper
pub fn rand_upto(n: usize) -> usize {
    // Random usize between 0 and n
    let out = random::<usize>() % n;
    assert!(out < n);
    out
}

pub fn rand_vec(n: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(rand_upto(n));
    }
    v
}

pub fn is_sorted<T: Ord>(a: &[T]) -> bool {
    for i in 0..(a.len() - 1) {
        if a[i] > a[i+1] {
            return false;
        }
    }
    true
}
