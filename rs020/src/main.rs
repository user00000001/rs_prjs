use std::collections::{HashMap};

#[derive(Debug)]
enum Value {
    One,
    Two,
    Three,
}

fn main() {
    let mut hm = HashMap::new();
    hm.insert("One", Value::Three);
    hm.insert("Two", Value::Two);
    hm.insert("One", Value::One);
    println!("{:?}", hm);
    let values = vec![Value::One, Value::Two, Value::Three];
    let indexs = vec![0, 1, 2];
    let hm1: HashMap<_, _> = indexs.into_iter().zip(values.into_iter()).collect();
    for (i, v) in &mut hm1.iter() {
        println!("{:?}", (i, v));
    }
    match hm1.get(&0) {
        Some(v) => println!("{:?}", v),
        None => println!("None")
    }
    let text = "hello world wonderful        world";
    let mut hm2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = hm2.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", hm2);
}
