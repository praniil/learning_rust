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
    let number2 : u32 = number1;
    println!("number1 : {}", number1);
    println!("number2: {}", number2);
    
    let s9 : String = String::from("string 9");
    let s10: String = takes_ownership(s9);      //move
    println!("string10: {}", s10);
    // println!("string9: {}, String1-: {}", s9, s10); //s9 is already moved to the funciton so it is not taken valid any longer

    let number_copy : i32 = 12;
    let result: i32 = makes_copy(number_copy);
    println!("original number: {}, copied number: {}", number_copy, result);

    //returning multiple values, returning the og string, and something that is manipulated using the og string
    let original_string : String = String::from("original string");
    let (og_string, len) = get_orignalstring_and_length(original_string);
    println!("og_string: {}, length: {}", og_string, len);
}

fn takes_ownership(s:  String) -> String {
    return s;
}

fn makes_copy(number: i32) -> i32{
    println!("number: {}", number);
    return number;
}

fn get_orignalstring_and_length(s: String) -> (String, usize){
    let length : usize = s.len();
    return (s, length);
}