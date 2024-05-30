pub fn dangling_ref() {
    let r;
//dangling reference
    {   
        let x = 32;
        // r = &x; //error:= dangling reference
        r = x; //this is copy so this is valid
    }
    //rust's borrow checker checks for the lifetime of the variable
    println!("r: {}", r);
}