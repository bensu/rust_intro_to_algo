extern crate rand;
use rand::random;

mod quick_find {
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
        // Checks if two elements are connected; O(1)
        sets[from] == sets[to]
    }

    pub fn union(sets: &mut [usize], from: usize, to: usize) {
        // Joins two elements; O(n)
        let from_root = sets[from];
        let to_root = sets[to];
        // change all that have from_root to to_root in O(n)
        for i in 0..sets.len() {
            if sets[i] == from_root {
                sets[i] = to_root;
            }
        }
    }

    // WIP
    // pub fn show_sets(sets: &[usize]) {
    //     // Prints the grouped sets
    //     let mut idxs: Vec<Vec<usize>> = Vec::new();
    //     for i in 0..sets.len() {
    //         idxs.push(Vec::new());
    //     }
    //     for (i,val) in sets.iter().enumerate() {
    //         match idxs.get(*val) {
    //             Some(ref xs) => (*xs).push(i),
    //             None => (),
    //         }
    //     }
    //     println!("{:?}", idxs);
    // }
}

mod quick_union {
    /* Quick Find is too slow (O(n)) for union. Instead of changing
     * all the set's indexes on union, change only one, forming a
     * tree. When querying, traverse each tree and find if both have
     * the same root
    */
    fn root(sets: & [usize], node: usize) -> usize {
        // Check the root by traversing the tree.
        // The longest possible tree is N -> O(n)
        if sets[node] != node {
            root(sets, sets[node])
        } else {
            node
        }
    }
    pub fn union(sets: &mut [usize], from: usize, to: usize) {
        sets[root(sets,from)] = root(sets,to);
    }
    pub fn connected(sets: &[usize], from: usize, to: usize) -> bool {
        root(sets,from) == root(sets,to)
    }
}

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
    const UNIONS: usize = 4;

    let mut find_sets: [usize; L] = [0; L];
    start_sets(&mut find_sets);

    let mut union_sets: [usize; L] = [0; L];
    start_sets(&mut union_sets);

    let mut unions: [(usize, usize); UNIONS] = [(0,0); UNIONS];
    for i in 0..UNIONS {
        unions[i] = (random_upto(L), random_upto(L));
    }

    // logic for Union Find
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        quick_find::union(&mut find_sets, from, to);
        quick_union::union(&mut union_sets, from, to);
    }

    println!("The unions are: {:?}", unions);
    println!("The quick_find sets are: {:?}", find_sets);
    println!("The quick_union sets are: {:?}", union_sets);

    // Test
    for i in 0..UNIONS {
        let (from,to) = unions[i];
        assert!(quick_find::connected(&find_sets, from, to));
        assert!(quick_union::connected(&union_sets, from, to));
    }

}
