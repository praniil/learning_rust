pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from: {}", self.summarize_author())
    }

}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}, reply?: {}, retweet?: {}", self.username, self.content, self.reply, self.retweet)
    }
    fn summarize_author(&self)-> String {
        format!("@{}", self.username)
    }
}

pub struct NewsArticle {
    pub title: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({}). Says: {}", self.title, self.author, self.location, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub fn traits() {
    let tweet = Tweet{
        username: String::from("Pranil"),
        content: String::from("@TenZ is the goat. He just own NRG's frauds"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
}
