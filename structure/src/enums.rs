//when we want to express something in code
//like ip address
//ip address has two variants: V4 and V6
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String), //putting data directly in the enum variant
}

//basically combination of different type of struct
enum Message {
    Quit, 
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

//enum methods:
impl Message{
    fn some_function(&self) {
        match self{ 
            Message::Write(content) => {
                println!("this is some funciton with message: {}", content);
            }
            Message::Move { x, y } => {
                println!("x y {} {}", x, y);
            }
            Message::ChangeColor(a, b, c ) => {
                println!("a, b, c: {} {} {}", a, b, c);
            }
            Message::Quit => {
            println!("Received quit message. Terminating the program...");
            std::process::exit(0);
        } 
        }
    }
}


struct IpAddr {
    kind : IpAddrKind,
    address : String,
}

// fn route(ip_kind: IpAddrKind) {
//     println!("ip kind, Version: {}", ip_kind);
// }
pub fn enums () {
    //variants are namespaced under their identifier
    // let four : IpAddrKind= IpAddrKind::V4;
    //
    let local_host : IpAddrKind = IpAddrKind::V6(String::from("127.0.0.1"));
    // route(four);
    // route(six);
    let home:IpAddrKind = IpAddrKind:: V4(127, 0, 0, 1);
    let msg1 : Message = Message::Write(String::from("hello"));
    msg1.some_function();
    
}
