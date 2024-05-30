pub fn generical_lifetime_annotation() {
    let string1 = String::from("Hello");
    let result;
    {
        let string2: String = String::from("mae");
        //ref is sent because we dont want to send the actual ownership of the string
        result = longest(string1.as_str(), string2.as_str());
        println!("the longest string here is: {}", result);
    }
    //result is dangling reference because the result's lifetime is valid 
    //smaller lifetime is of string2, so result is not valid because the string2 lifetime is over
}
//string.as_str() is a reference to the string rather than the string itself
//lifetime specifier needed because the fn doesnt know which one will return from the fn
fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    //using generical lifetime makes sure the return type lifetime is same to the smallest lifetime of one of the parameter of the fn
    //if string1 has smaller lifetime than string2 the return value lifetime is equal to the string1
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}