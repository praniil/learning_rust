extern crate rand;//letting the rust know we are using a external dependency
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop{

        println!("Please input your guess.");
        //mut is used for making the variable guess mutable
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); //handling possible failure
    //parse string to unsigned int
    //if error its like panic , immediately exits from here
    /*
    let guess : u32 = guess.trim().parse()
        .expect("Please type a number");
    */
    let guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
println!("Your guessed: {}", guess);
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too Small"),
    Ordering::Greater => println!("Too Big"),
    Ordering::Equal => {
        println!("You win!");
        break;
    },
}
}

}
