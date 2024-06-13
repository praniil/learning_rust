#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),   //Box is fixed size pointer in the stack, it compiles now because the size is given in the compile time    
    Nil,
}
//with recursive enum we dont know what size it takes in the compile time
use List::{Cons, Nil};
//string, vec are also smart pointers
pub fn smart_pointer_box() {
    //allocates value in heap
    let b = Box::new(4);
    println!("b: {}", *b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
} 