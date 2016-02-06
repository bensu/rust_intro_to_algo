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

fn is_concave(a: Option<&Point>, b: Option<Point>, c: Point) -> bool {
    if let Some(a) = a {
        if let Some(b) = b {
            let rotor = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            rotor <= 0f64
        } else {
            false
        }
    } else {
        false
    }
}

fn convex_hull(a: &mut [Point; N]) -> List<Point> {
    // Will sort the array
    let min_y_idx = min_y(*a);
    let min_y_point = a[min_y_idx];
    a.sort_by(|a, b| cmp_theta(min_y_point, *a, *b));

    let mut hull = stack::List::<Point>::new();
    let mut base = min_y_point;
    let mut base_theta = 0f64;
    // First 2 are guaranteed to be in hull
    hull.push(a[0]);
    hull.push(a[1]);
    // The test one also goes in
    hull.push(a[2]);
    for i in 2..a.len() {
        let mut top = hull.pop();
        while is_concave(stack::car(&hull), top, a[i]) {
            top = hull.pop();
        }
        match top {
            None => { panic!("Something's not right!"); },
            Some(top) => { hull.push(top); },
        }
        hull.push(a[i]);
    }
    hull
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // Generate the points
        let mut a = [Point::new(); N];
        for i in 0..N {
            a[i] = random_point();
        }
        let mut hull = convex_hull(&mut a);
        while !hull.is_empty() {
            println!("{:?}", hull.pop());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tests() {
        let mut a = [Point::new(); N];
        for i in 0..N {
            a[i] = random_point();
        }
        let mut hull = convex_hull(&mut a);
        while !hull.is_empty() {
            println!("{:?}", hull.pop());
        }
    }
}
