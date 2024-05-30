pub fn lifetime_elision () {
    let any_string : String = String::from("any string");
    let result = first_word(any_string.as_str());
    println!("result: {}", result);
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}