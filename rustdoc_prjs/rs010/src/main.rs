#[allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
    let (x, y) = another_function(30);
    println!("x: {}, y: {}", x, y);
    another_fn1(x);
    let True: bool = true;
    let c = if True {
        1 
    } else {
        // "2"
        2 
    };
    let mut counter = 0;
    let d = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };

    while counter != 0 {
        println!("counter: {}", counter);
        counter -= 1;
    }
    println!("{:#?} {} {}", c, d, counter);

    let a = [10, 20, 30];
    for e in a.iter() {
        println!("element: {}", e);
    }
    for e in (1..=5).rev() {
        println!("{}", e);
    }
}

#[allow(unused)]
fn another_function(arg0: u32) -> (i32, i32) {
    let a: &str;
    println!("It comes from a function.");
    println!("{}", arg0);
    // let x = (let y = 10);
    let y;
    let x = { 
        if arg0 > 10 { y = 100 } else { y = 10 }; y };
    println!("{:#?} {:#?}", x, y);
    (x, y+1)
}

fn another_fn1(arg0: i32) -> i32 {
    if arg0 != 100 {
        100
    } else {
        arg0
    }
}