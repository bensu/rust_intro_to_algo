
extern crate rand; // used for testing
use self::rand::random;

// Helper
pub fn rand_upto(n: usize) -> usize {
    // Random usize between 0 and n
    let out = random::<usize>() % n;
    assert!(out < n);
    out
}
