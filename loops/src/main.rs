//three types of loops in rust := loops, while, for
fn main() {
    loop{
        println!("Hello, world!");
        break;//smart way to stop the execution
    }
    //loop is used to execute code unless we externally interrup it 
    let mut number = 4;
    while number > 0 {
        println!("{}", number);
        number -= 1;
    }

    let mut array= [1, 2, 3, 4, 5, 6];
    for items in array.iter_mut() {
        *items = *items + 1;
    }
    println!("{:?}", array);
}
