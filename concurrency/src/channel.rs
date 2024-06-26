use std::sync::mpsc;
use std::thread;

pub fn channel() {
    //mpsc multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    let sending_thread = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // println!("the value sent was: {}", val);  Error  //ownership set to the other thread through channel
    }); 

    let received_thread = thread::spawn(move|| {
        let received = rx.recv().unwrap();
        println!("received is: {}", received)
    });


    received_thread.join().unwrap();
    sending_thread.join().unwrap();
}   