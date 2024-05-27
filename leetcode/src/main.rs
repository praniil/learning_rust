

fn main() {
    //length of last word question
    let result = length_of_last_word(String::from("Hello World                      "));
    println!("{}", result);
}

fn length_of_last_word(s: String) -> i32 {
    let vector : Vec<&str> = s.split_whitespace().collect();
    let length : usize = vector.len();
    let last = &vector[length - 1];
    return last.len() as i32;
}