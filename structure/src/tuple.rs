//tuple structs without named field
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn tuple () {
    let black : Color = Color(0, 0, 0);
    let origin: Point= Point(0, 0, 0);
    println!("black: {} {} {}", black.0, black.1, black.2);
    println!("coordinates of origin in plane: {} {} {}", origin.0, origin.1, origin.2);
}