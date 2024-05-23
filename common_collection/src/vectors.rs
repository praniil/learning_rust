use std::vec;

pub fn vectors() {
    let _array1= [1, 2, 3];  //creating in stack
    let second = &_array1[1];
    println!("{}", second);
    let mut v:Vec<i32> = Vec::new();    //creating in heap
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    //if i want to initialize a vector
    let mut v2: Vec<i32> = vec![1, 2, 36];
    v2.push(34);
    println!("{:?}", v2);

    let third = &v2[2]; 
    println!("third element of vector: {}", third);
    //safer way to access the element of vector;
    //using get method

    match v.get(2) {
        Some(value) => println!("the second element is: {}", value),
        None => println!("there is no second element"),
    }

    let mut vec1 : Vec<i32> = vec![1, 2, 5, 6, 14];
    {
        let forth : &mut i32 = &mut vec1[3];
        println!("forth {}", forth);
    }    //immutable reference to vec1
    vec1.push(24);  //mutate error.

    // iterating through the element of the vector

    for element in &mut vec1 {
        *element = *element + 1;
    }

    for i in &vec1{
        println!("{}", i);
    }    

    let mut vector_one = vec![1, 2, 3];
    vector_one.push(69);

    let forth :&i32 = &vector_one[3];
    println!("the forth element is: {}", forth);

    let third = vector_one.get(3); 
    match third {
        Some(&num) => println!("the forth element is: {}", num),
        None => println!("we didint find the forth element")
    }

    /*
    let first = &vector_one[0]; //immutable reference to the first element of vector
    vector_one.push(33);    //we are trying to change or append a element at last Error
    println!("first: {}", first);
    */ 

    //we know that vectors can hold elements of same kind
    //what if we want to use elements of different types in vector?
    //we can use enum: enum has specific variants may be of different types
    //now we can make vector of enum type

    enum SpreadSheet {
        Int(i32),
        Text(String),
        Float(f64),
    }
    //vector that has same kind of data i.e SpreadSheet of enum type
    let vector_spread_sheet : Vec<SpreadSheet> = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Text(String::from("hello")),
        SpreadSheet::Float(2.4),
    ];

    let first = &vector_spread_sheet[1];
    match first {
        SpreadSheet::Int(i) => println!("{}", i),
        _ => println!("not a integer")
    }
}