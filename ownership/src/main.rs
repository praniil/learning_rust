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
    
    //mutable reference
    let mut s : String = String :: from("hello Hero ");
    change(&mut s);
    println!("original string hello Hero: {}", s);

    //multiple mutable references
    let mut first: String = String::from("i am the original string");
    let first_borrow = &first;  // Immutable borrow
    let second_borrow = &first; // Another immutable borrow
    //cannot use mutable reference after immutable reference
    println!("{}, {}", first_borrow, second_borrow);
    //now we can use it because the scope of first and second borrow is over
    let third_borrow = &mut first; // Mutable borrow
    println!("third borrow: {}", third_borrow);

    //dangling
    let result_form_dangle = dangling();
    println!("result form dangle: {}", result_form_dangle);

}
/*
fn dangling() -> &String {
    let s : String = String::from("in dangling");
    &s
    //we are returning the reference i.e the borrowed value of s
    //the effect of s is dropped after the } bracket so it returns the reference is invalid or reference of nothing
    
}
*/

fn dangling() -> String {
    let s : String = String::from("in dangling");
    s
    //this is correct  
}

fn change(str: &mut String) {
    str.push_str("how are you?");
    //by default the refrences are not mutable
    println!("string after push: {}", str);
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