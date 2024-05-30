struct ImportantExcerpt<'a> {
    part: &'a str,  //to mention reference we have to introduce lifetime annotation
}
//like generics we have to mention the lifetime_annotation
impl<'a> ImportantExcerpt <'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("The announcement: {}", announcement);
        self.part
    }
}

pub fn lifetime_annoation_struct () {
    let novel = String::from("Call me Hero. Some years ago...");
    
    let first_sentence = novel.split(".").next().expect("could not find");

    let i : ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };
    i.return_part("Hello");
    println!("{}", i.part);
}