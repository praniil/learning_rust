use std::sync::Mutex;

pub fn mutex() {
    let m = Mutex::new(5);  //5 is the data we want to use or manupulate

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m: {:?}", m);
}