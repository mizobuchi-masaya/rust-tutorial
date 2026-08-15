#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let pf = Point { x: 1.0, y: 4.0 };
    println!("pf.distance_from_origin = {}", pf.distance_from_origin());
    
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
