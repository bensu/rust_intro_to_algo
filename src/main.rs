// Convex Hull

mod util;

#[derive(Debug,Copy,Clone)]
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
    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;
    let h = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
    (delta_x / h).cos()
}

fn random_point() -> Point {
    Point {
        x: util::rand_upto(10) as f64,
        y: util::rand_upto(10) as f64,
    }
}

const N: usize = 10;

fn main() {
    let mut a = [Point::new(); N];
    for i in 0..N {
        a[i] = random_point();
    }
    println!("{:?}", a);
    println!("asF");
}
