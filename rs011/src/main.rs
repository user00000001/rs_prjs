fn main() {
    let hello = String::from("Hello, ");
    // let mut hw = hello;
    let mut hw = hello.clone();
    hw.push_str("world!");
    println!("{}", hw);
    println!("{}", hello);
    takes_ownership(hello);
    let x = 5;
    makes_copy(x);
    let s = gives_ownership();
    let s1 = takes_and_gives_back(s);
    println!("I'm back: {}", s1);
    let (s2, len) = calculate_length(s1);
    println!("String: \"{}\"'s length is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let s = String::from("I'm back!");
    println!("gives_ownership: {}", s);
    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("takes_and_gives_back: {}", s);
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}