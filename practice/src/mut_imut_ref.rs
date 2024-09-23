pub fn references() {
    let mut string1: String = String::from("hello world");

    let string2 = &string1;
    let string3 = &string1;

    //error    let string4 = &mut string1;
    println!("{string2}, {string3}");
    let string4 = &mut string1;
    string4.push('h');
    println!("{string4}");
}