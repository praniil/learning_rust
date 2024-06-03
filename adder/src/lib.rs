
#[derive(Debug)]
pub struct Rectangle{
    pub width: u32,
    pub height : u32,
}

impl Rectangle{ 
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height     
    }
}

pub struct Guesss {
    pub number: i32,
}

impl Guesss {
    pub fn new(value: i32) -> Guesss {
        if value < 1 {
            panic!("Guess must be greater than 1")
        } else if value >= 100 {
            panic!("Guess value must be less than or equal to 100 ")
        }
        Guesss { number: value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Namaste! {}", name)
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two_fn (a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    //path that includes everything
    use super::*;
    
    #[test]
    #[ignore]
    fn larger_can_hold_smaller () {
        let large: Rectangle = Rectangle { width:5, height: 2 };
        let smaller : Rectangle = Rectangle { width: 3, height: 1 };
        assert!(large.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_can_hold_larger () {
        let large: Rectangle = Rectangle { width:5, height: 2 };
        let smaller : Rectangle = Rectangle { width: 3, height: 1 };
        assert!(!smaller.can_hold(&large));
        //assert macro will invoke paniuc! macro if the provided expression cannot be evaluated to true at runtime
    }

    //assert_eq! allows you to compare two values
    #[test]
    #[ignore]
    fn test_add_two () {
        assert_ne!(4, add_two(3));
        //ne = not equal
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn check_name() {
        let name = "Phil";
        let result = greeting(name);
        //custom failure message
        assert!(result.contains("Phil"), "Greeting didnot contain the name: {}", result);
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Guess value must be less than or equal to 100 ")]
    fn greater_than_100 () {
            Guesss::new(1000);
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 5 {
            Ok(())
        } else {
            //err = panic
            Err(String::from("two plus three doesnt equal four"))
        }
    }

    #[test]
    #[ignore]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(8);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(8, value);
    }

    #[test]
    #[ignore]
    fn one_hundered_and_two () {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn test_add () {
        assert_eq!(4, internal_adder(2, 2));
    }

    
}
