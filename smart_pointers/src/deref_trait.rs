struct MyBox<T> (T);
use std::ops::Deref;


impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_traits() {
    let var1 = 5;
    let var2 = MyBox::new(var1);

    let var3 = 10;
    let var4 = &var3;

    println!("{}", var4);

    assert_eq!(5, var1);
    let any_var = *var2;

    let reference_to_inner: &i32 = &*var2;
    println!("{}", reference_to_inner);

    println!("any var: {}", any_var);
    assert_eq!(5, *(var2.deref()));


    let deref = *(var2.deref());
    println!("{}", deref);
    println!("var2 deref: {}", *(var2.deref()));

    let a = 5;
    //value of a is not copied to b
    let b = &5;

    assert_eq!(5, a);
    assert_eq!(5, *b);
    // assert_eq!(5, b);

    // using Box for allocating value in heap
    let x = 5;
    //here the value of x is copied
    //y points to the copied value of x
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    //&MyBox<String> -> &String -> &str
    //without rust deref trait
    println!("{}", *m);
    println!("{}", &(*m));
    println!("{}", &(*m)[..]);
    hello(&(*m)[..]);
    //with deref_trait
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}