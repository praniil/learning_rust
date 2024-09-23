pub fn dangling_ref() {
    let any_var = return_dangling_ref();
    println!("{any_var}")
}

fn return_dangling_ref() -> /*&String*/ String {    //sent ownership instead of reference to correct the error
    let string1 = String::from("Hello World");
    // &string1    //string1 will get dropped and we are sending the ref of that, its a dangling ref
                //returning the mem location of string1 that is dropped
    let string2 = take_ownership(string1);
    // &string1    //dangling ref
    string2
}

fn take_ownership(s1: String) -> String {
    println!("string: {}", s1);
    return s1; 
}