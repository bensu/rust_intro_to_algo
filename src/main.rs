// Merge Sort

mod util;

use std::fmt;

const N: usize = 10;

fn is_sorted<T: Ord>(a: &[T]) -> bool {
    for i in 0..(a.len() - 1) {
        if a[i] > a[i+1] {
            return false;
        }
    }
    true
}

/// Copies a elements from a point into outs ends
fn append_from<T: Copy>(a: &[T], from: usize, out: &mut Vec<T>) {
    assert!(from < a.len());
    for i in from..a.len() {
        out.push(unsafe { *a.get_unchecked(i) });
    }
}

fn merge<T: Ord + Copy + fmt::Debug>(a: &[T], b: &[T]) -> Vec<T> {
    assert!(is_sorted(a));
    assert!(is_sorted(b));
    let na = a.len();
    let nb = b.len();
    let mut out = Vec::with_capacity(na + nb);
    let mut i = 0;
    let mut j = 0;
    // until we are done with one of the two
    while i < na && j < nb {
        let ai = unsafe { a.get_unchecked(i) };
        let bj = unsafe { b.get_unchecked(j) };
        if ai < bj {
            out.push(*ai);
            i = i + 1;
            // we are done with a, copy what's left of b in out
            if i == na {
                append_from(b, j, &mut out);
            }
        } else {
            out.push(*bj);
            j = j + 1;
            if j == nb {
                append_from(a, i, &mut out);
            }
        }
    }
    assert!(is_sorted(&out));
    out
}

fn sort<T: Ord + Copy + fmt::Debug>(v: &[T]) -> Vec<T> {
    let n = v.len();
    if n == 1 {
        let mut out = Vec::with_capacity(1);
        out.push(v[0]);
        out
    } else {
        let slices = v.split_at(n/2);
        let a = slices.0;
        let b = slices.1;
        merge(&sort(a), &sort(b))
    }
}

fn main() {
    let mut v = Vec::with_capacity(N);
    for i in 0..N {
        v.push(util::rand_upto(N));
    }
    assert_eq!(vec![1,2,3,4,5,6], merge(&vec![1,4,5],&vec![2,3,6]));
    assert_eq!(vec![1,3,4,5,6,8], sort(&vec![1,8,3,4,5,6]));
    assert!(is_sorted(&sort(&v)));
}
