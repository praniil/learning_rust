use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;
fn main() {
    // panic!(" it wont work");
    //backtrace
    a();
    let file = File::open("/home/pranil/rustProjects/learningRust/error/src/score.txt");
    let mut file = match file{
        Ok(file) => {
            file
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("/home/pranil/rustProjects/learningRust/error/src/score.txt") {
                    Ok(okay) => okay,
                    Err(error) => panic!("couldnot create the desired file {}", error),
                }
                other_error => panic!("couldnot open the file: {}", other_error),
            }
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => println!("File content: {}", content),
        Err(error) => panic!("couldnot red the file's content: {}", error),
    }

    //simplifying the error handling for opening and reading the content of the file
    let result = open_and_read_from_file();
    match result {
        Ok(conent) => conent,
        Err(error) => panic!("error: {}", error), 
    };
    //second way
    let hello1_file = File::open("hello1.txt").expect("couldnot open the file");
    //third way
    let hello_file = File::open("hello.txt").unwrap();  //similar to match for OK and Err status

}

fn open_and_read_from_file() -> Result<String, io::Error> {
    let mut s : String = String::new();
    //? = if error returns it otherwise carry the next function
    File::open("/home/pranil/rustProjects/learningRust/error/src/hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
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