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

    

}