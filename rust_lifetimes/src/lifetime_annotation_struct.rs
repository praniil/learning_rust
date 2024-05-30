struct ImportantExcerpt<'a> {
    part: &'a str,  //to mention reference we have to introduce lifetime annotation
}

pub fn lifetime_annoation_struct () {
    let novel = String::from("Call me Hero. Some years ago...");
    
    let first_sentence = novel.split(".").next().expect("could not find");

    let i : ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}