#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let penny = Coin::Penny;
    let state = UsState::Alabama;
    let coin_tuple = (
        value_in_cents(penny), 
        Coin::Nickel, 
        Coin::Dime, 
        Coin::Quarter(state),
        Coin::Quarter(UsState::Alaska),
    );
    println!("{:?}", coin_tuple);
    println!("{:?}", value_in_cents(coin_tuple.4));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(v) => Some(v+1),
            None => None,
        }
    }
    let mut x: Option<i32> = Some(10);
    match plus_one(x) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    if let Some(v) = x {
        println!("if let have a value: {}", v)
    }

    if let None = x {
        println!("if let is None.");
    } else  {
        println!("if let else still have {:?}", x);
    }

    x = None;
    match plus_one(x) {
        Some(v) => println!("{}", v),
        _ => {
            println!("None");
            ()
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter's state:{:?}", state);
            25
        },
    }
}