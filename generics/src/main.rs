struct Point <T, U> {
    x: T,
    y: U,
}
//T and U can be same or different type
//methods using generics
impl <T, U> Point<T, U> {
    fn some_fn(&self) -> &T {
        &self.x
    }
}

impl <T, U> Point <T, U> {
    fn mixup<V, W>(&self, other: Point<V, W>) -> Point<&T, W> {
        Point { x: &self.x, y: other.y }
    }
}

fn main() {
    let array : Vec<i32> = vec![1, 25, 3, 4];
    let largest : i32 = get_largest(array);
    println!("the largest number is: {}", largest);

    let array_char : Vec<char> = vec!['a', 'b', 'c', 'd'];
    let largest_char : char = get_largest(array_char);
    println!("the largest char is: {}", largest_char);

    let p1   = Point { x: 3, y: 4 };
    let p2  = Point { x: 3.0, y: 4.0 };
    let p3 = Point{ x: 3, y: 4.0};
    let some_point = p1.some_fn();
    println!("{} {} {} {}", p1.x, p2.y, p3.x, some_point);

    let number = Point { x: 3, y: 4.0};
    let word = Point {x: 'a', y: "hello"};
    let mixed = number.mixup(word);
    println!("{}, {}", mixed.x, mixed.y)
}
/*
fn get_largest_number(array: Vec<i32>) -> i32 {
    let mut large : i32 = array[0];
    for number in array {
        if number > large {
            large = number;
        }
    }
    return large;
}
*/

//if we want to promote more reusability in code we use generic type
fn get_largest<T: PartialOrd + Copy>(array: Vec<T>) -> T {
    let mut large = array[0];
    for number in array {
        if number > large {
            large = number;
        }
    }
    return large;
}

