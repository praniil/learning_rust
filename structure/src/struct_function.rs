#[derive(Debug)]
struct Rectange {
    length : u32,
    width : u32,
}

impl Rectange {
    //now i am defining associated function which doesnt include self parameter
    //Associated functions are often used for constructors that will return a new instance os the struct
    fn square(size: u32) -> Rectange {
        Rectange{length: size, width: size}
    }
}

pub fn struct_function () {
    let square : Rectange = Rectange::square(5);
    println!("square length: {}", square.length);
    println!("square breadth: {}", square.width);
    println!("the square: {:#?}", square);
}