// Convex Hull

mod util;

mod stack;

use stack::List;
use stack::Stack;

use std::f64;
use std::cmp::Ordering;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new() -> Self {
        Point { x: 0f64, y: 0f64 }
    }
}

fn theta(a: Point, b: Point) -> f64 {
    if a != b {
        let delta_x = b.x - a.x;
        let delta_y = b.y - a.y;
        let atan = delta_y.atan2(delta_x);
        if atan < 0f64 {
            atan + 2f64 * f64::consts::PI
        } else {
            atan
        }
    } else {
        0f64
    }
}

fn random_point() -> Point {
    Point {
        x: util::rand_upto(10) as f64,
        y: util::rand_upto(10) as f64,
    }
}

/// Returns the index for the Point with the minimum y
fn min_y(xs: [Point; N]) -> usize {
    let mut min: usize = 0;
    for i in 0..N {
        if xs[i].y < xs[min].y {
            min = i;
        }
    }
    min
}

const N: usize = 10;

fn cmp_theta(base: Point, a: Point, b: Point) -> Ordering {
    match theta(base, a).partial_cmp(&theta(base, b)) {
        None => { panic!("NaN found"); Ordering::Equal },
        Some(ord) => ord
    }
}

fn main() {
    // let mut a = [Point::new(); N];
    // for i in 0..N {
    //     a[i] = random_point();
    // }
    let mut a = [Point { x: 4f64, y: 0f64 }, Point { x: 9f64, y: 4f64 }, Point { x: 5f64, y: 5f64 }, Point { x: 5f64, y: 5f64 }, Point { x: 5f64, y: 9f64 }, Point { x: 4f64, y: 5f64 }, Point { x: 4f64, y: 2f64 }, Point { x: 1f64, y: 9f64 }, Point { x: 3f64, y: 2f64 }, Point { x: 2f64, y: 2f64 }];

    let min_y_idx = min_y(a);
    let min_y_point = a[min_y_idx];
    a.sort_by(|a, b| cmp_theta(min_y_point, *a, *b));
    println!("{:?}", a);

    let mut hull = stack::List::<(Point,f64)>::new();
    let mut all = stack::List::<Point>::new();
    for i in 0..N {
        all.push(a[N-i-1]);
    }
    all.pop();
    let mut base = min_y_point;
    let mut base_theta = 0f64;
    while !all.is_empty() {
        let mut test;
        match all.pop() {
            None => { panic!("We checked for emptyness"); },
            Some(point) => { test = point; },
        };
        let test_theta = theta(base, test);
        if base_theta <= test_theta {
            // we try the next
            hull.push((base,base_theta));
            base = test;
            base_theta = test_theta;
        } else {
            // we unroll the last one
            match hull.pop() {
                None => { panic!("Shouldn't happen!"); },
                Some((prev, prev_theta)) => {
                    base = prev;
                    base_theta = prev_theta;
                },
            }
            all.push(test);
        };
    }
    hull.push((base,base_theta));

    while !hull.is_empty() {
        println!("{:?}", hull.pop());
    }
}
