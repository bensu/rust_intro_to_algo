// Merge Sort

mod util;

use std::fmt;

const N: usize = 10;

fn is_sorted<T: Ord>(a: &Vec<T>) -> bool {
    for i in 0..(a.len() - 1) {
        if a[i] > a[i+1] {
            return false;
        }
    }
    true
}

/// Copies a elements from a point into outs ends
fn append_from<T: Copy>(a: &Vec<T>, from: usize, out: &mut Vec<T>) {
    assert!(from < a.len());
    for i in from..a.len() {
        out.push(unsafe { *a.get_unchecked(i) });
    }
}

fn merge<T: Ord + Copy + fmt::Debug>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    assert!(a.len() == a.len());
    assert!(is_sorted(&a));
    assert!(is_sorted(&b));
    let n = a.len();
    let mut out = Vec::with_capacity(2*n);
    let mut i = 0;
    let mut j = 0;
    while i < n && j < n {
        let ai = unsafe { a.get_unchecked(i) };
        let bj = unsafe { b.get_unchecked(j) };
        if ai < bj {
            out.push(*ai);
            i = i + 1;
            if i == n {
                // we are done with a, copy what's left of b in out
                append_from(&b, j, &mut out);
            }
        } else {
            out.push(*bj);
            j = j + 1;
            if j == n {
                append_from(&a, i, &mut out);
            }
        }
    }
    assert!(is_sorted(&out));
    out
}

fn main() {
    let mut v = Vec::with_capacity(N);
    for i in 0..N {
        v.push(i);
        // a[i] = util::rand_upto(N);
    }
    assert_eq!(vec![1,2,3,4,5,6], merge(&vec![1,4,5],&vec![2,3,6]));

    println!("{:?}", v);
}
