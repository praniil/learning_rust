struct CustomSmartPointer {
    data : String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom Pointer with data: {}", self.data);
    }
}
pub fn drop_traits() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Other stuff"),
    };
    println!("Smart pointer created");
    drop(c);
    println!("Smart pointer dropped before main");
}