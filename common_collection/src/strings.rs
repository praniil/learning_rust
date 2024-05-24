use unicode_segmentation::UnicodeSegmentation;

pub fn strings () {
    println!("in strings");
    let string_slice : &str = "hello";
    let mut string2 = string_slice.to_string();
    string2.push('a');
    string2.push_str(" hello hello");
    println!("string2 :{}", string2);
    
    let string_nepali = String::from("नमस्कार");
    println!("string nepali: {}", string_nepali); 

    let s1 : String = String::from("Hello ");
    let s2 : &str = "World";
    let s3 = s1 + &s2;

    //ownership of s1 is gone to the concatination function 
    println!("string new: {}", s3);

    // println!("{}", s1); //invalid
    println!("{}", s3);

    let string_new = String::from("नमस्कार");

    // let first_char = string_new[0] //this wont work in rust because rust's string is of type utf-8 where a char chan have more than one byte
    //so we cant access the char using index method

    //to access all bytes
    for byteee in string_new.bytes() {
        println!("{}", byteee)
    }

    //to access all the chars used
    for charrs in string_new.chars() {
        println!("{}", charrs)
    }

    for ch in "hello my mate".chars() {
        println!("{}", ch);
    }
    //using graphemes method
    for g in string_new.graphemes(true) {
        println!("{}", g);
    }

    if let Some(first_char) = "hello".chars().nth(0) {
        println!("{}", first_char)
    } else{
        println!("string is empty")
    }

    if let Some(first_ch) = "hello".chars().next() {
        println!("{}", first_ch)
    } else{
        println!("string is empty")
    }

    let string1 : String = String::from("tic");
    let string2 : String = String::from("tac");
    let string3 : String = String::from("toe");

    let string_concat : String = string1 + "-" + &string2 + "-" + &string3;
    println!("{}", string_concat);
    let string1_new : String = String::from("tic");

    //this method is more readable plus it doesnt take ownership

    let new_string_concat = format!("{}-{}-{}", string1_new, string2, string3);
    println!("{}", string1_new);
    println!("{}", new_string_concat);
}