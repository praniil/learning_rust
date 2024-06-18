#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    id : u32,
    book_type : BookType,
    status: Availability,
}

impl Book {
    fn new(title: &str, author: &str, id:u32, book_type: BookType) -> Book{
        Book { title: title.to_string(), author: author.to_string(), id: id, status: Availability::Available, book_type: book_type }
    }

    fn checkout(&mut self) {
        self.status = Availability::Checkout
    }

    fn return_book(&mut self) {
        self.status = Availability::Available
    }
}

#[derive(Debug)]
enum Availability{
    Available,
    Checkout,
}

#[derive(Debug)]
enum BookType{
    Science,
    _History,
    _Mathematics,
    _Economics,
}

pub fn library() {
    // let book: Book = Book::new(title, author, id, book_type)
    let mut book: Book = Book::new("University Physics", "Author UP", 33, BookType::Science);
    println!("The status of the book is: {:?}", book.status);

    book.checkout();
    println!("The status of the book is: {:?}", book.status);

    book.return_book();
    println!("The status of the book is: {:?}", book.status);

}