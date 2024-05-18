// enum Option<T> {
//     Some(T), //generic
//     None,
// }
//we can use Some and None without prefixing them with Option::

pub fn enum_option() {
    let mut some_number = Some(6);
    let some_string = Some("any string");
    let y = 3;
    let x : Option<i32> = Some(5);
    let x = None;
    let sum = x.unwrap_or(0) + y;
    // if we dont use unwrap_or() it wont add because of different type, but after we used unwrap_or with default value 0 for None variant it will run the sum successfully
    println!("the sum is: {}", sum);
    let absent_number : Option<i32> = None;
    println!("absent number : {}", absent_number.unwrap_or(0));
    // println!("some number: {}", some_number);
    // println!("some number: {}", some_number);
    //we cant use them without match, or without matching the variant

    match some_number {
        Some(ref mut number) => {
            *number = *number + 5;
            println!("the number is: {}", *number)
        },
        None => println!("No number"),
    }
} 