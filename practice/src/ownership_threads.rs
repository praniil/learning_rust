use std::{thread, vec};

pub fn ownership_threads() {
    let vec1 = vec![1, 2, 4, 5];

    let handle = thread::spawn(move || {
        println!("vec1: {:?}", vec1)
    });

    handle.join().unwrap();
}