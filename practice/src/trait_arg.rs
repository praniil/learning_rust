use std::vec;

trait Area {
    fn area(&self) -> u32;
}

struct Circle {
    radius: u32,
}

impl Area for Circle {
    fn area(&self) -> u32 {
        (22/7) * self.radius * self.radius
    }
}

struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
}

fn print_area_static<T: Area> (shape: &T) {
    println!("Area: {}", shape.area());
}

fn print_area_dynmic(shape: &dyn Area) {
    println!("dyn area: {}", shape.area());
}

fn return_shape() -> impl Area {
    // if result == true {
    //     Circle{
    //         radius: 3,
    //     }
    // } else {
    //     Rectangle{
    //         length: 3,
    //         breadth: 4,
    //     }
    // }
    Circle{
        radius: 3,
    }
}

pub fn trait_arg() {
    let circle = Circle {
        radius: 3,
    };

    let rectange = Rectangle {
        length: 2,
        breadth: 3,
    };

    print_area_static(&circle);
    print_area_static(&rectange);

    let shape_vec: Vec<&dyn Area> = vec![&circle, &rectange];

    for shape in shape_vec {
        print_area_dynmic(shape);
    } 

    let circle1 = return_shape();
    println!("{}", circle1.area())

}