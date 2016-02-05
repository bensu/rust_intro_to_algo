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

fn main() {
    // let mut a = [0; 10];
    // for i in 0..a.len() {
    //     a[i] = util::rand_upto(10);
    // }
    let a = &mut [2, 2, 8, 7, 3, 5, 3, 0, 7, 9];
    shell(a);
    assert_eq!(a, &[0, 2, 2, 3, 3, 5, 7, 7, 8, 9]);
}
