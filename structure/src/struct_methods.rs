struct Rectange {
    length: u32,
    breadth : u32,
}

//defining the method of struct
impl Rectange{
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
}

pub fn struct_methods() {
    let rectange: Rectange = Rectange{
        length: 50,
        breadth: 100,
    };
    println!("the area of rectange using the struct method is: {}", rectange.area());
}