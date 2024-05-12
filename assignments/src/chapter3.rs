fn conversion() {
    loop{

        println!("enter the number 1 for farenheit and 2 for celcius");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)
        .expect("failed to read line");
    let choice : i32 = match choice.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Error: invalid input");
            return;
        }
    };
    // println!("choice: {}", choice);
    match choice{
        1 => {
            println!("Pressed 1");
            let mut farh: String = String::new();
            println!("Enter the temperature in farenheit: ");
            std::io::stdin().read_line(&mut farh)
                .expect("failed to read temperature");
            let farh: i32 = match farh.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("couldnt parse the temperature");
                    return;
                } 
            };
            let celcius: i32 = 5 / 9 * (farh - 32);
            println!("the temperature in celcius is: {}", celcius);
        },
        2 => {
            let mut celcius : String = String::new();
            println!("enter the temperature in celcius");
            std::io::stdin().read_line(&mut celcius)
                .expect("failed to read temperature");  
            let celcius : i32 = match celcius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("couldnt parse the temperature");
                    return;
                }
            };
            let farh = celcius * (9 / 5) + 32;
            println!("the temperature in farh is: {}", farh);
        },
        _ => break,
    }
}
    
}

fn generate_nth_fibo() {
    let mut number: String = String::new();
    println!("enter the number the nth number:");
    std::io::stdin().read_line(&mut number)
        .expect("couldnt read the number");
    let number : u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("couldnt parse the number");
            return;
        }
    };
    let fibo_number: u32 = fibo(number);
    println!("the nth fibo number is: {}", fibo_number)
}

fn fibo(number :u32) -> u32 {
    if number == 0 {
        return 0;
    } else if number == 1 || number == 2 {
        return 1;
    } else {
        return fibo(number - 1) + fibo(number - 2);
    }
}

// fn tweleve_days_christmas() {
//     let array : [&str; 3] = ["one", "two", "three"];
// }

pub fn chapter_three() {
    conversion();
    generate_nth_fibo();
    // tweleve_days_christmas();
}