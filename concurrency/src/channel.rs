use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channel() {
    //mpsc multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    // let sending_thread = thread::spawn(move || {
    //     let val = String::from("hello");
    //     tx.send(val).unwrap();
    //     // println!("the value sent was: {}", val);  Error  //ownership set to the other thread through channel
    // }); 

    // let received_thread = thread::spawn(move|| {
    //     let received = rx.recv().unwrap();
    //     println!("received is: {}", received)
    // });

    let sending_vector_of_string = thread::spawn(move|| {
        let vec_string = vec![
            String::from("1st message"),
            String::from("2nd message"),
            String::from("3rd message"),
        ];

        for val in vec_string{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("received: {}", received);
    }
    sending_vector_of_string.join().unwrap();
    // received_thread.join().unwrap();
    // sending_thread.join().unwrap();
}   