use std::{vec};

// pub trait Iterator {
//     type Item;
//     //Iterator trait requires that you also define an Item type, 
//     //Item type is used in the return type of next method
//     fn next(&mut self) -> Option<Self::Item>;
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    //we can use this next method directly
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter.next());
    for items in v1_iter {
        println!("{}", items);
        println!("x");
    }    

    //iterates over mutable reference
    let mut v2 = vec![2, 3, 4];
    let mut v2_iterator = v2.iter_mut();
    v2_iterator.next();
    println!("{:?}", v2_iterator);
    let sum2 : i32= v2_iterator.map(|x| *x).sum();
    //&mut i32 is converted to i32 by dereferencing
    println!("sum v2: {}", sum2);

    //for taking ownership
    let v3 = vec![3, 5];
    let mut v3_iterator = v3.into_iter();
    v3_iterator.next();
    println!("{:?}", v3_iterator);
    let sum: i32 = v3_iterator.sum();
    println!("sum: {}", sum);
    // let one = v3[0]; error borrow of the moved value 

    let v3 = vec![4, 5, 6, 7];
    let v3_iter = v3.iter();
    let v4: Vec<_> = v3_iter.map(|x| x + 1).collect();
    println!("{:?}", v4);
}