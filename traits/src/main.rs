pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    //this implementation overrides the default summary
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }
}

pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub headline: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
    // fn summarize(&self) -> String {
    //     format!("{} by {}", self.content, self.author)
    // }
    //if we donot define the summarize fn inside summary it will do the default implementation
}

//trait
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    //for default implementation
    fn summarize(&self) -> String {
        format!("(Read more from ) {}", self.summarize_author())
    }
}

//trait as argument
pub fn trait_as_argument (item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

//trait bounds
//this generic is limited to something that implements the trait
pub fn trait_as_argument_bound <T: Summary> (item: &T) {
    println!("Breaking new! {}", item.summarize());
}
fn main() {
    println!("Hello, world!");
    let article1 : NewsArticle = NewsArticle{
        author: String::from("Pranil"),
        content: "The pollution is real in the city Kathmandu the heart of Nepal".to_string(),
        headline: "Kathmandu, Nepal is the most polluted city!".to_string(),
    };
    println!("{:?}", article1.summarize());
    let tweet1 : Tweet = Tweet { username: "Pranil".to_string(), content: "KTM is turning hell".to_string(), retweet: false, reply: true };
    println!("{:?}", tweet1.summarize());

    trait_as_argument(&tweet1);
    trait_as_argument_bound(&article1);
}
