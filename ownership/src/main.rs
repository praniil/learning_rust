fn main() {
    println!("Hello, world!");
    let mut s : String = String::from("hello");
    s.push_str(", world");
    // let mut s1 : &str = "hell0";    //string literal, stack
    // so we cant mutate string s1;
    let s1 : String = String::from("first string");
    // let s2 = s1;
    // println!("s1: {}", s1);    //this throws error as string in s1 is moved to s2 not copied

    let s3: String = s1.clone();
    println!("s1: {}", s1); //copied
    println!("s3: {}", s3); //copied

    let number1 : u32 = 33;
    let number2 = number1;
    println!("number1 : {}", number1);
    println!("number2: {}", number2);
}
