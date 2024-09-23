pub fn move_ownership() {
    let s1 = String::from("Hello");
    let mut s2 = s1;    //move
    println!("{}", s2);
    println!("{s2}");
    s2.push('a');
    // s1.push('a');

    deep_copy();
}

fn deep_copy() {
    let s1 = String::from("Hello Copy");
    let s2 = s1.clone();    //deep copy, not only stack values are copied but heap's data as well
    println!("Address s2: {:p}", &s2);
    println!("Address s1: {:p}", &s1);
    println!("s2: {s2} and s1: {s1}");
}