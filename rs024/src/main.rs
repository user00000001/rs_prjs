use std::fmt::Display;

fn main() {
    let (x, y) = ("haha", "hehehe");
    println!("The longest string is {}", longest(&x, &y));
    println!("The longest string is {}", longest("aaaaaaaa".to_owned().as_str(), &"bbbbbbbbbb"[..=5]));
    println!("The longest string is {}", longest_with_an_announcement("aaaaaaaa".to_owned().as_str(), &"bbbbbbbbbb"[..=5], "HOHOHO"));
    // let result;
    // {
    //     let s2 = "xyz".to_owned();
    //     result = longest(&s2, y);
    // }
    // println!("{}", result); // result may come from s2 which might out of scope.
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i; 
    {
        i = ImportantExcerpt {
            part: first_sentence,
        }; 
    }
    println!("{:#?}", i);
    println!("{:#?}", i.level());
    println!("{:#?}", i.announce_and_return_part("hehe"));

    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}