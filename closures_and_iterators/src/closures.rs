use std::{thread, time::Duration};

//closures are anonymous functions you can save in a variable ore pass as arguments to other funcitons
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red, 
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())    
        //some xa vane returns some field, otherwise returns the return of closure
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor :: Red
        } else {
            ShirtColor :: Blue
        }
    }
}

pub fn closures () {
    
    let expensive_closure = |num| {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(0);

    let expen_closure = |x| x;
    expen_closure(String::from("hello world"));
    //once the complier infer the type of x and the return type of the closure to be String, those types are then locked itno the closure in expen_closures
    // expen_closure(3);

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The use with preference {:?} gets {:?}", user_pref2, giveaway2);

    //capturing reference or moving ownership
    //borrowing immutably, borrowing mutably, taking ownership
    //borrowing immutably

    let list: Vec<i32> = vec![1, 2, 3];
    println!("before defining closures: {:?}", list);
    let only_borrows = || println!("From closures: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    //mutable borrow
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);
    //println!("Before: {:?}", list2); //no other borrows are allowed when there is already mutable borrow
    borrows_mutably();
    println!("After calling closure: {:?}", list2);
}
