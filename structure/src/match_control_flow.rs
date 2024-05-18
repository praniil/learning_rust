#[derive(Debug)]
enum UsState {
    new_york,
    alaska, 
    florida,
    minnesota,
}
enum Coin {
    Penny, 
    Nickel,
    Dime(UsState),
    Quarter,
}


fn value_in_cents(coin : Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(state) => {
            println!("State Dime from{:?} ", state);
            10
        },
        Coin::Quarter => 25,
    }
}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(number) => Some(number + 1),
    }
}

pub fn match_public_flow() {
    let result = value_in_cents(Coin::Dime(UsState::alaska));
    println!("resutl; {}", result);
    let number : Option<i32> = Some(32);
    let plus_one_res = plus_one(number);
    let none = plus_one(None);
    println!("none: {:?}", none);
    println!("plus one result: {:?}", plus_one_res);
}