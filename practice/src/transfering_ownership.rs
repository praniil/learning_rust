pub fn transfer_ownership() {
    let s1: String = String::from("Hello World");
    let (s2, len) = calculate_len(s1);

    println!("{s2}, {len}");

    //instead of using tuple for the return value, we can only send the reference to the string variable
    let s3 = String::from("sending a reference only not the ownership");
    calucalte_length_using_reference(&s3);  //&s3 is reference of s3 or simply address of s3

    println!("Address: {:p}", &s3);
}

fn calculate_len(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}

fn calucalte_length_using_reference(s: &String) -> usize {
    println!("{s}");
    println!("Address: {:p}", &s);
    return s.len()
}