use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (x:&'a str, y:&'a str, announcement: T) -> &'a str 
where
    T:Display
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

pub fn generics_traits_lifetime () {
    let string1: String = String::from("first string");
    let string2: String = String::from("second string");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Yoo mate");
    println!("result: {}", result);
}