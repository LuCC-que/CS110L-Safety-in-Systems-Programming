/*
    The compiler is capable of providing basic implementations 
    for some traits via the #[derive] attribute. 
    These traits can still be manually implemented 
    if a more complex behavior is required.
 */
#[derive(Debug, PartialEq, Clone, Copy)]

struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {x: x, y: y}
    }
}

fn main() {
    let the_origin = Point::new(0.0, 0.0);
    let mut p = the_origin; // copy semantics!
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin);
    p.x += 10.0;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin);
}
