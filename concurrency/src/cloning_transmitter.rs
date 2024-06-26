use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};

pub fn cloning_transmitter () {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vector_string = vec![
            "Hello",
            "How are",
            "YOU",
        ];
        for val in vector_string {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //tx is cloned i.e creating multiple producers by cloning transmitter
    thread::spawn(move || {
        let vector_string = vec![
            "2nd Hello",
            "2nd How are",
            "2nd YOU",
        ];
        for val in vector_string {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in rx{
        println!("received: {}", rec);
    }
}