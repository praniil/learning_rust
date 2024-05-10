fn main() {
    println!("Hello, world!");
    let fact : i32 = factorial(5);
    println!("{}", fact);
    let evaluate : i32 = evaluate(5);
    println!("evaluate: {}", evaluate);
    let result = ressult(true);
    println!("result: {}", result);
}

fn factorial (number: i32) -> i32 {
    if number == 0 || number == 1 {
        return 1;   
    } else {
        return number * factorial(number - 1);
    }
}

fn evaluate (number: i32) -> i32 {
    let _x = number;
    let y = {
        let x = 3;
        x + 1
    };
    return y;
}
//using if in let statement
fn ressult(res: bool) -> i32{
    let condition = res;
    let result = if condition {
        5
    } else {
        6
    };
    return result;
}