use std::ops::Add;

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

//if you dont define traits from your own object
//compiler will choose to run this one
pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        //unreasonble traits
        //rust will use it over the defining function
        1.0
    }
}

// The following doesn't really make sense, it's just meant to illustrate the
// idea that the functions in traits can have default implementations!
impl ComputeNorm for Option<u32> {}

impl ComputeNorm for Point {
    fn compute_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ComputeNorm for Vec<f64> {
    fn compute_norm(&self) -> f64 {
        // an example of functional rust syntax
        self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
    }
}

impl Add for Point {
    type Output = Self; // an "associated type"
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
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
    let some_opt = Some(110);
    

    // following doest work as the ComputeNorm is only for u32
    // let somestring = Some(String::from("Hello"));
    // println!("norm of the hello: {}", somestring.compute_norm());


    //using the traits over the defining function
    println!("norm of some_opt: {}", some_opt.compute_norm());
    let lil_vec: Vec<f64> = vec![3.0, 4.0];
    println!("norm of lil_vec: {}", lil_vec.compute_norm());
    println!("norm of (3, 4) + the_origin: {}",
        (the_origin + Point::new(3.0, 4.0)).compute_norm());
    
}
