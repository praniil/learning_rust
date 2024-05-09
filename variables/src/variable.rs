pub fn variable() {
    //capital letter with underscore to seperate words is the convention for defining the constants
    const NEG_VALUE : i32 = -23;
    const POS_VALUE : u32 = 23;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The negative value is: {}", NEG_VALUE);
    println!("The poisitive value is: {}", POS_VALUE);

    //shadowing the variable
    let y = 100;
    //first y shadowed by 2nd
    let y = y + 1;
    //second y shadowed by 3rd
    let _y = y * 2;
    //while using the variable it takes the latest value
    //y is still immutable
    //y = y + 3; compile error
    let y = "Pranil";
    //we cant mutate the type 
    /*let mut space = "    ";
    space = space.len();*/
    println!("The value of y is: {}", y);
}