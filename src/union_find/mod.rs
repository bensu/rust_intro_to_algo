/// Contains several implementations for disjoints sets following the
/// interface:
/// - union(set, a, b) -> void
/// - connected(sets, a, b) -> bool

/* Could probably do it with traits, but I don't know how to make a
 * struct that has size known at runtime, but passed from somewhere
 * else
 * -> Change code to take size at run time (with Vec)
 */

extern crate rand; // used for testing

pub mod quick_find {
    #![allow(dead_code)]
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

pub mod quick_union {
    #![allow(dead_code)]
    /* Quick Find is too slow (O(n)) for union. Instead of changing
     * all the set's indexes on union, change only one, forming a
     * tree. When querying, traverse each tree and find if both have
     * the same root
     */
    // Since root is O(n), everything else is O(n)
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

#[derive(Debug, Copy, Clone)]
pub enum Node {
    Root(u32),   // Roots contain the depth of their tree
    Leaf(usize), // Leafs point to some other Node
}

pub mod b_quick_union {
    #![allow(dead_code)]
    /* To prevent the trees from growing deep, we *balance* them:
     * whenever we join to trees, add the shallow one under the deep
     * one. Then the tree depth only increases (by one) if the both
     * trees have the same depth.
     * Now roots has lg(N) speed, since that is the depth of weighted trees.
     */
    use super::Node;
    fn root(sets: &[Node], n: usize) -> (usize, u32) {
        // O(lg(n)) since the trees are weighted
        match sets[n] {
            Node::Root(x) => (n, x),
            Node::Leaf(r) => root(sets, r),
        }
    }
    fn set_leaf(sets: &mut [Node], n: usize, r: usize) {
        // Used to impose n != r preconditions
        assert!(n != r);
        sets[n] = Node::Leaf(r);
    }
    pub fn union(sets: &mut [Node], from: usize, to: usize) {
        // O(lg(n)) since it depends on root
        let (from_root, from_depth) = root(sets, from);
        let (to_root, to_depth) = root(sets, to);
        // guard agains infinite loop (& extra work)
        if from_root != to_root {
            // Add the shallow one to the deeper one
            if from_depth < to_depth {
                set_leaf(sets, from_root, to_root);
            } else if from_depth == to_depth {
                // if equal, increase the weight
                sets[to_root] = Node::Root(to_depth + 1);
                set_leaf(sets, from_root, to_root);
            } else {
                set_leaf(sets, to_root, from_root);
            }
        }
    }
    pub fn connected(sets: &[Node], from: usize, to: usize) -> bool {
        // O(lg(n)) since it depends on root
        root(sets,from) == root(sets,to)
    }
}

pub mod pc_quick_union {
    #![allow(dead_code)]
    /* When we check for a node's root, we *compress the path* from
     * the node to the root by setting the pointer directly and saving
     * ourselves from future work.
     */

    use super::Node;
    fn set_leaf(sets: &mut [Node], n: usize, r: usize) {
        // Used to impose n != r preconditions
        assert!(n != r);
        sets[n] = Node::Leaf(r);
    }
    fn root(sets: &mut [Node], n: usize) -> (usize, u32) {
        // O(lg(n)) since the trees are weighted
        // The function is can not be Tail Call Optimized
        match sets[n] {
            Node::Root(x) => (n, x),
            Node::Leaf(r) => {
                let (r, d) = root(sets, r);
                set_leaf(sets, n, r); // Compress the path
                (r, d)
            }
        }
    }
    pub fn union(sets: &mut [Node], from: usize, to: usize) {
        // O(lg(n)) since it depends on root
        let (from_root, from_depth) = root(sets, from);
        let (to_root, to_depth) = root(sets, to);
        // guard agains infinite loop (& extra work)
        if from_root != to_root {
            // Add the shallow one to the deeper one
            if from_depth < to_depth {
                set_leaf(sets, from_root, to_root);
            } else if from_depth == to_depth {
                // if equal, increase the weight
                sets[to_root] = Node::Root(to_depth + 1);
                set_leaf(sets, from_root, to_root);
            } else {
                set_leaf(sets, to_root, from_root);
            }
        }
    }
    pub fn connected(sets: &mut [Node], from: usize, to: usize) -> bool {
        // O(lg(n)) since it depends on root
        root(sets,from) == root(sets,to)
    }
}

pub mod test {
    // Test Helpers
    fn random_upto(n: usize) -> usize {
        use super::rand::random;
        random::<usize>() % n
    }
    fn start_sets(sets: &mut [usize]) {
        for i in 0..sets.len() {
            sets[i] = i;
        }
    }
    // Main tests
    pub fn main() {
        use super::*;

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
}
