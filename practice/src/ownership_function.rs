pub fn ownership_stack_heap() {
    let string1 = String::from("Hello World!");
    let mut x = 4;
    // heap_data(string1); //string-> heap ma allocated, only pointer, len, capacity in the stack
    // println!("string1: {}", string1);   //error because the ownership is with the function heap_data now

    // if we want deep copy of the string1
    heap_data(string1.clone());
    println!("string1: {string1}");
    println!("address of string1: {:p}", &string1);
    stack_data(x);
    x = x - 1;

    println!("{x}");
}

fn heap_data(mut string: String) {
    string.push('a');
    println!("address of string: {:p}", &string);
    println!("{string}s");
}

fn stack_data(var: i32) {
    println!("{var}");
}