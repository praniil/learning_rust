struct Point {
    x : f64,
    y : f64,
}

impl Point{
    fn distance(&self, other: &Point) -> f64 {
        let x_squared : f64 = f64::powi(other.x - self.x, 2);
        let y_squared : f64 = f64::powi(other.y - self.y, 2);
        return f64 :: sqrt(x_squared + y_squared);
    }
}

pub fn multiple_methods_params() {
    let p1 : Point = Point {
        x : 2.0,
        y : 3.0,
    };
    let p2 : Point = Point {
        x : 2.0,
        y : 3.0,
    };
    //automatic referencing of p1
    println!("the distance between p1 and p2 is: {}", p1.distance(&p2));
    //explicit referencing of p1
    println!("the distance is: {}", (&p1).distance(&p2));
}