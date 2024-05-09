//Scalar and Compound Type
pub fn data_types() {
    let _guess : u32 = "45".parse()
        .expect("not a number");
    let dec : u64 = 98_222;
    let hex : u64 = 0xff;
    let oct : u64 = 0o77;

    let x = 2.0;//by default f64
    let y: f32 = 3.0;
    println!("dec: {}", dec);
    println!("hex: {}", hex);
    println!("octal: {}", oct);
    println!("{}",x);
    print!("{}\n",y);

    //boolean type
    let t = true;//implicit type annotation
    let f : bool = false;//explicit type annotation
    println!("{} & {}", t, f);

    let character : char = 'x';
    println!("{}", character);

    //Compound Types
    //Tuples and Arrays
    //different values or different data types ko grouping:= tuple
    let tup: (i32, u32, &str) = (500, 40, "Pranil");
    println!("{}", tup.0);  //direct accesing the items of tuple
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    //array collection of multiple values of same data type
    let array : [u32; 4] = [1, 2, 3, 4];
    println!("{}", array[0]);
}