#[derive(Debug)]
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}.", self.headline, self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn lets_summarize() -> impl Summary {
    NewsArticle {
        author: String::from("Krishna Lagad"),
        headline: String::from("Genesis training"),
        content: String::from("KPIT Technologies offers a set of learning practices to newly \
                                    hired freshers, which they called as Genesis training.")
    }
}
/*
    rustc trait.rs -o app && time ./app && rm app
 */
fn main() {
    let article = NewsArticle {
        author: String::from("Krishna Lagad"),
        headline: String::from("Genesis training"),
        content: String::from("KPIT Technologies offers a set of learning practices to newly \
                                    hired freshers, which they called as Genesis training.")
    };

    let tweet = Tweet {
        username: String::from("krishnalagad"),
        content: String::from("Some random text of aa tweet."),
        retweet: false,
        reply: false
    };

    println!("{:#?}", tweet);
    println!("{:#?}", article);

    println!("Summary of Tweet: {}", tweet.summarize());
    println!("Summary of News Article: {}", article.summarize());
    println!("{}", lets_summarize().summarize());
}