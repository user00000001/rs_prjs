use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // "(Read more...)".to_owned()
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.localtion)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notifies(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize())
}

pub fn notifies_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize())
}

pub fn notifies_same_type1<T>(item1: &T, item2: &T)  
    where T: Summary
{
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize())
}

pub fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        })
    }
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x:T, y:T) -> Self {
        Self { x, y }
    }
}

impl<T: Display+PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}