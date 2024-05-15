struct User{
    username : String,
    email : String,
    sign_in_count : u64,
    active: bool,
}

//field init shorthand := if the field's name and the parameter of the function is same
fn build_user(email: String, username: String) -> User {
    User { username: username, email: email, sign_in_count: 1, active: true }
}

pub fn defining_and_instanting () {
    //user1 is an instance of the struct User
    //if the instance is mutable we can change the value of instance
    let user1 = User{
        username: String::from("Nancy"),
        email : String::from("nanacywheeler32@gmail.com"),
        active: true,
        sign_in_count : 1,
    };
    let username : String = String::from("steve");
    let email : String = String::from("steve@gmail.com");
    let user2 : User = build_user(email, username);
   
    println!("username: {}", user1.username);
    println!("user email: {}", user1.email);
    println!("user signin count: {}", user1.sign_in_count);
    println!("user active status: {}", user1.active);

    println!("username: {}", user2.username);
    println!("user email: {}", user2.email);
    println!("user signin count: {}", user2.sign_in_count);
    println!("user active status: {}", user2.active);

    //creating instances form other instances with struct update syntax
    //created a new User instance, user3 and setting some fields to values of the sam fields from user1 and user2
    let user3 : User = User{
        username: String::from("Dustin"),
        email: String::from("iamdustingnerd@gmail.com"),
        active : user1.active,
        sign_in_count : user1.sign_in_count,
    };

    //the same can be achieved by struct update syntax

    let user4 : User = User {
        username: String::from("Mike"),
        email: String::from("miketheleader@gmail.com"),
        ..user1 //struct update syntax
    };

    println!("username: {}", user3.username);
    println!("user email: {}", user3.email);
    println!("user signin count: {}", user3.sign_in_count);
    println!("user active status: {}", user3.active);

    println!("username: {}", user4.username);
    println!("user email: {}", user4.email);
    println!("user signin count: {}", user4.sign_in_count);
    println!("user active status: {}", user4.active);

}