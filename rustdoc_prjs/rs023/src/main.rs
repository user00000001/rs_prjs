use rs023_lib::{Summary, Tweet, NewsArticle, notify, notifies, notifies_same_type1, returns_summarizable, Pair };

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_owned(),
        content: "of course, as you probably know, people".to_owned(),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_owned(),
        location: "Pittsburgh, PA, USA".to_owned(),
        author: "Iceburgh".to_owned(),
        content: "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.".to_owned(),
    };
    println!("New article available! {}", article.summarize()); // impl Summary for NewsArticle {} // to use default summary implementation

    notify(&tweet);
    notify(&article);
    notifies(&tweet, &article);
    notifies_same_type1(&tweet, &tweet);

    println!(
        "{} + {}", 
        returns_summarizable(true).summarize(),
        returns_summarizable(false).summarize()
    );

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Pair::new(1,3);
    p.cmp_display();
}
