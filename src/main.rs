extern crate rand;

use rand::random;

fn random_upto(n: usize) -> usize {
    random::<usize>() % n
}

fn connected(sets: &[usize], from: usize, to: usize) -> bool {
    // Checks if two elements are connected.
    sets[from] == sets[to]
}

fn union(sets: &mut [usize], from: usize, to: usize) {
    // Joins two elements
    let from_root = sets[from];
    let to_root = sets[to];
    // change all that have from_root to to_root
    for i in 0..sets.len() {
        if sets[i] == from_root {
            sets[i] = to_root;
        }
    }
}

fn main() {

    const L: usize = 10;
    const UNIONS: usize = 4;

    let mut sets: [usize; L] = [0; L];
    for i in 0..L {
        sets[i] = i;
    }

    let mut unions: [(usize, usize); UNIONS] = [(0,0); UNIONS];
    for i in 0..UNIONS {
        unions[i] = (random_upto(L), random_upto(L));
    }

    // logic for Union Find
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        union(&mut sets, from, to);
    }

    // test
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        println!("{}", connected(&sets, from, to));
    }

    println!("{:?}", unions);
    println!("{:?}", sets);
}
