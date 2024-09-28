use std::thread;
use std::time::Duration;

pub fn thread() {   
    let thread1 = thread::spawn(|| {
        for i in 1..10 {
            println!("in thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("in main fn {i}");
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}