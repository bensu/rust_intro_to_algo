// Selection sort

mod util;

use std::fmt;

pub fn selection<T: Ord>(xs: &mut [T]) {
    let l = xs.len();
    for i in 0..(l - 1) {
        let mut min = i;
        for j in (i+1)..l {
            if xs[j] < xs[min] {
                min = j;
            }
        }
        xs.swap(i, min);
    }
}

pub fn insertion<T: Ord>(xs: &mut [T]) {
    let l = xs.len();
    for i in 1..l {
        let mut j = i;
        while (0 < j) && (xs[j] < xs[j-1]) {
            xs.swap(j-1, j);
            j = j - 1;
        }
    }
}

pub fn shell<T :Ord>(xs: &mut [T]) {
    let l = xs.len();
    let mut h: usize = 1;
    while h < l/3 {
        h = 3*h + 1;
    }
    while 1 <= h {
        let mut i = h;
        while i < l {
            let mut j = i;
            while (h <= j) && (xs[j] < xs[j-h]) {
                xs.swap(j-h, j);
                j = j - h;
            }
            i = i + h;
        }
        h = h/3;
    }
}

pub fn shuffle<T>(xs: &mut [T]) {
    for i in 0..xs.len() {
        if i != 0 {
            xs.swap(i, util::rand_upto(i));
        }
    }
}

// Merge Sort

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
    // TODO: this loop can be replaced with a Vector API function
    for i in from..a.len() {
        out.push(unsafe { *a.get_unchecked(i) });
    }
}

/// Merges to already sorted slices into a sorted vector
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

fn merge_sort<T: Ord + Copy + fmt::Debug>(v: &[T]) -> Vec<T> {
    let n = v.len();
    if n == 1 {
        let mut out = Vec::with_capacity(1);
        out.push(v[0]);
        out
    } else {
        let slices = v.split_at(n/2);
        let a = slices.0;
        let b = slices.1;
        merge(&merge_sort(a), &merge_sort(b))
    }
}

fn partition<T: Ord>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    assert!(lo < hi);
    // grabs the first value
    let mut i = lo;
    let mut j = hi;
    while i < j {
        // find the first value that is bigger than p from the beginning
        while i < hi && v[i] <= v[lo] {
            i = i + 1;
        }
        // we have a value that is bigger than p
        while lo < j && v[lo] <= v[j] {
            j = j - 1;
        }
        // we a value that is smaller than p
        // then we have two values that are out of place. exchange them
        if i < j {
            v.swap(i,j);
        }
    }
    v.swap(lo,j);
    j
}

fn quick_sort_rec<T: Ord>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if lo < hi {
        let j = partition(v, lo, hi);
        if j > 0 {
            quick_sort_rec(v, lo, j - 1);
        }
        if j + 1 < hi {
            quick_sort_rec(v, j + 1, hi);
        }
    }
}

fn quick_sort<T: Ord>(v: &mut Vec<T>) {
    // TODO: add shuffling
    let n = v.len();
    quick_sort_rec(v, 0, n - 1);
}


#[cfg(test)]
mod tests {

    const N: usize = 10;


    #[test]
    fn step_test() {
        let mut v = util::rand_vec(N);
        assert_eq!(vec![1,2,3,4,5,6], merge(&vec![1,4,5],&vec![2,3,6]));
        assert_eq!(vec![1,3,4,5,6,8], merge_sort(&vec![1,8,3,4,5,6]));
        assert!(is_sorted(&merge_sort(&v)));
        let mut u = vec![1,3,5,2,4,6];
        let n = u.len();
        partition(&mut u, 0, n - 1);
        assert_eq!(vec![1, 3, 5, 2, 4, 6], u);
        partition(&mut u, 1, n - 1);
        assert_eq!(vec![1, 2, 3, 5, 4, 6], u);
        partition(&mut u, 2, n - 1);
        assert_eq!(vec![1, 2, 3, 5, 4, 6], u);
        partition(&mut u, 3, n - 1);
        assert!(is_sorted(&u));
    }
    #[test]
    fn second_step_test() {
        let mut v = vec![1, 3, 4, 4, 4, 5, 5, 0, 2, 1];
        let n = v.len();
        partition(&mut v, 0, n - 1);
        assert_eq!(v, vec![0, 1, 4, 4, 4, 5, 5, 3, 2, 1]);
        partition(&mut v, 1, n - 1);
        assert_eq!(v, vec![0, 1, 4, 4, 4, 5, 5, 3, 2, 1]);
        partition(&mut v, 2, n - 1);
        assert_eq!(v, vec![0, 1, 3, 4, 4, 1, 2, 4, 5, 5]);
        partition(&mut v, 2, 6);
        assert!(is_sorted(&v));
        // Test the whole thing
        let mut v = vec![1, 3, 4, 4, 4, 5, 5, 0, 2, 1];
        quick_sort(&mut v);
        assert!(is_sorted(&v));
    }


    #[test]
    fn third_step() {
        let mut v = vec![9, 8, 5, 3, 1, 5, 7, 0, 4, 7];
        let n = v.len();
        assert_eq!(9, partition(&mut v, 0, n - 1));
        assert_eq!(v, vec![7, 8, 5, 3, 1, 5, 7, 0, 4, 9]);
        assert_eq!(7, partition(&mut v, 0, 8));
        assert_eq!(v, vec![0, 4, 5, 3, 1, 5, 7, 7, 8, 9]);
        assert_eq!(0, partition(&mut v, 0, 6));
        assert_eq!(v, vec![0, 4, 5, 3, 1, 5, 7, 7, 8, 9]);
        assert_eq!(3, partition(&mut v, 1, 6));
        assert_eq!(v, vec![0, 3, 1, 4, 5, 5, 7, 7, 8, 9]);
        assert_eq!(2, partition(&mut v, 1, 2));
        assert!(is_sorted(&v));
        // full
        let mut v = vec![9, 8, 5, 3, 1, 5, 7, 0, 4, 7];
        quick_sort(&mut v);
        assert!(is_sorted(&v));
    }


    #[test]
    fn multi_test() {
        for i in 0..100 {
            let mut v = util::rand_vec(N);
            quick_sort(&mut v);
            assert!(is_sorted(&v));
        }
    }

    #[test]
    fn test() {
        // let mut a = [0; 10];
        // for i in 0..a.len() {
        //     a[i] = util::rand_upto(10);
        // }
        let a = &mut [2, 2, 8, 7, 3, 5, 3, 0, 7, 9];
        shell(a);
        assert_eq!(a, &[0, 2, 2, 3, 3, 5, 7, 7, 8, 9]);
    }
}
