fn main() {
    // panic!("fuck it! it wont work");
    //backtrace
    a();
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num : i32) {
    if num == 22 {
        panic!("its 22, dont pass it")
    }
} 