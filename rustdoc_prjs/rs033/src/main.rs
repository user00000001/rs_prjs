mod lib;

use lib as mylib;
use mylib::{AveragedCollection, Button, Draw, Screen, 
    Post,
    Post1,
    // Screen1, 
};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "width: {}, height: {}, options: {:?}", 
            self.width,
            self.height,
            self.options,
        );
    }
}

fn main() {
    let mut ac = AveragedCollection::new(vec![], 0.0);
    ac.add(1);
    println!("{:?}'s average is: {}", ac, ac.average());
    ac.add(3);
    println!("{:?}'s average is: {}", ac, ac.average());
    ac.add(5);
    println!("{:?}'s average is: {}", ac, ac.average());
    if let Some(result) = ac.remove() {
        println!("{:?}'s average is: {}, removed is {}.", ac, ac.average(), result);
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // let screen1 = Screen1 {
    //     components: vec![
    //         SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         },
    //         Button { // mismatched types (SelectBox, Button)
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         },
    //     ],
    // };
    // screen1.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_ne!("", post.content());

    post.request_review();
    assert_ne!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{}", post.content());


    let mut post1 = Post1::new();

    post1.add_text("I ate a salad for lunch today");

    let post1 = post1.request_review();

    let post1 = post1.approve();
    assert_eq!("I ate a salad for lunch today", post1.content());
    
    println!("{}", post1.content());
}
