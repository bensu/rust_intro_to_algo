// Selection sort

mod util;

use std::fmt;

pub fn selection<T: Ord + fmt::Debug>(xs: &mut [T]) {
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

fn main() {
    // let mut a = [0; 10];
    // for i in 0..a.len() {
    //     a[i] = util::rand_upto(10);
    // }
    let a = &mut [2, 2, 8, 7, 3, 5, 3, 0, 7, 9];
    selection(a);
    assert_eq!(a, &[0, 2, 2, 3, 3, 5, 7, 7, 8, 9]);
}
