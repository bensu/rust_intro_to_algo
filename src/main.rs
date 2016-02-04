extern crate rand;
use rand::random;

mod union_find;

use union_find::*;

// Helpers

fn random_upto(n: usize) -> usize {
    random::<usize>() % n
}

fn start_sets(sets: &mut [usize]) {
    for i in 0..sets.len() {
        sets[i] = i;
    }
}

fn main() {

    const L: usize = 10;
    const UNIONS: usize = 9;

    let mut find_sets: [usize; L] = [0; L];
    start_sets(&mut find_sets);

    let mut union_sets: [usize; L] = [0; L];
    start_sets(&mut union_sets);

    let mut balanced_sets: [Node; L] = [Node::Root(1); L];

    let mut compressed_sets: [Node; L] = [Node::Root(1); L];


    let mut unions: [(usize, usize); UNIONS] = [(0,0); UNIONS];
    for i in 0..UNIONS {
        let from = random_upto(L);
        let to = random_upto(L);
        unions[i] = (from, to);
    }
    println!("The unions are: {:?}", unions);

    // logic for Union Find
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        quick_find::union(&mut find_sets, from, to);
        quick_union::union(&mut union_sets, from, to);
        b_quick_union::union(&mut balanced_sets, from, to);
        pc_quick_union::union(&mut compressed_sets, from, to);
    }

    println!("The quick_find sets are: {:?}", find_sets);
    println!("The quick_union sets are: {:?}", union_sets);
    println!("The balanced sets are: {:?}", balanced_sets);
    println!("The compressed sets are: {:?}", compressed_sets);

    // Test
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        assert!(quick_find::connected(&find_sets, from, to));
        assert!(quick_union::connected(&union_sets, from, to));
        assert!(b_quick_union::connected(&balanced_sets, from, to));
        assert!(pc_quick_union::connected(&mut compressed_sets, from, to));
    }
}
