extern crate rand;
use rand::random;

mod union_find {
    /* Implements disjoints sets through Union Find
     * Each index of the sets array maps onto an object in the graph.
     * The user can specify if two objects (a & b) are connected by
     * stating `union(&mut sets,a,b)` which will mutate the sets to
     * take the union into account.
     * The user can check if two elements are conencted with
     * connected(&sets, a, b)` and get a boolean.
     */

    /* The underlying data structure (sets) tracks the connected
     * elements by keeping their values equal. If two indexes have the
     * same value, they are connected. If two indexes need to be
     * connected, one of them (and all it's associated indexes) change
     * value to the others
     */
    pub fn connected(sets: &[usize], from: usize, to: usize) -> bool {
        // Checks if two elements are connected.
        sets[from] == sets[to]
    }

    pub fn union(sets: &mut [usize], from: usize, to: usize) {
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

    pub fn show_sets(sets: &[usize]) {
        // Prints the grouped sets
        let mut idxs: Vec<Vec<usize>> = Vec::new();
        for i in 0..sets.len() {
            idxs.push(Vec::new());
        }


        for (i,val) in sets.iter().enumerate() {
            match idxs.get(*val) {
                Some(ref xs) => (*xs).push(i),
                None => (),
            }
        }
        println!("{:?}", idxs);
    }
}

fn random_upto(n: usize) -> usize {
    random::<usize>() % n
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
        union_find::union(&mut sets, from, to);
    }

    println!("The unions are: {:?}", unions);
    println!("The sets are:");
    union_find::show_sets(&sets);

    // test
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        assert!(union_find::connected(&sets, from, to));
    }

}
