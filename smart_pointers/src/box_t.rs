enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn box_t() {
    let pointer = Box::new(5);
    println!("{}", pointer);
    let address = &*pointer as *const i32;
    println!("{:?}", address);

    println!("the size of the Box: {}", std::mem::size_of_val(&pointer));   //8 byte in 64 bit architecture

    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
}