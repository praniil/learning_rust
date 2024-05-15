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
}