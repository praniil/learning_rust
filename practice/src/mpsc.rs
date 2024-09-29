use std::time::Duration;
use std::{sync::mpsc, thread};

pub fn multiple_producer_single_consumer() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let thread1 = thread::spawn(move|| {
        let vec1 = vec![1, 2, 3, 4, 5];
        for val in vec1 {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread1.join().unwrap();

    let thread2 = thread::spawn(move|| {
        let vec2 = vec![6, 7, 8, 9, 10];
        for val in vec2 {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for receive in rx {
        println!("received: {receive}");
    }

    thread2.join().unwrap();
    
}