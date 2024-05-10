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


pub fn chapter_three() {
    conversion();
}