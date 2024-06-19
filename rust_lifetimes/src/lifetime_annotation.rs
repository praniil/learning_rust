fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetime_annotation() {
    let string1 = String::from("firstdfs string");
    let result;
    {
        let string2 = String::from("second string");
        result = longest(&string1, &string2);
        println!("result: {}", result);
    }
}