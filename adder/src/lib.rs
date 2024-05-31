
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
        } else if value > 100 {
            panic!("Guess must be smaller than 100")
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

#[cfg(test)]
mod tests {
    //path that includes everything
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller () {
        let large: Rectangle = Rectangle { width:5, height: 2 };
        let smaller : Rectangle = Rectangle { width: 3, height: 1 };
        assert!(large.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger () {
        let large: Rectangle = Rectangle { width:5, height: 2 };
        let smaller : Rectangle = Rectangle { width: 3, height: 1 };
        assert!(!smaller.can_hold(&large));
        //assert macro will invoke paniuc! macro if the provided expression cannot be evaluated to true at runtime
    }

    //assert_eq! allows you to compare two values
    #[test]
    fn test_add_two () {
        assert_ne!(4, add_two(3));
        //ne = not equal
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn check_name() {
        let name = "Phil";
        let result = greeting(name);
        //custom failure message
        assert!(result.contains("Phil"), "Greeting didnot contain the name: {}", result);
    }

    #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100 ")]
    fn greater_than_100 () {
            Guesss::new(10);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 5 {
            Ok(())
        } else {
            //err = panic
            Err(String::from("two plus three doesnt equal four"))
        }
    }
}
