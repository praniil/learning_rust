use std::thread;
use std::time::Duration;

pub fn thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("from spawned thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    //the main thread will wait until the spawned thread is finish
    // handle.join().unwrap();
    //spawned thread will do the assigned work and then main thread does

    let v: Vec<i32> = vec![1, 2, 3];
    //mentioning move inside spawned thread will insure the ownership of v to the thread
    let handle1 = thread::spawn(move || {
        print!("the vector is {v:?}");
    });

    // drop(v); //we cant do this because v is already dropped but the handle1 may never get the reference to v
    
    for i in 1..5  {
        println!("from main thread {i}");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap(); //will wait for all the threads to finish
    handle1.join().unwrap();

}