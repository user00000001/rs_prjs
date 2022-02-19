#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(unused)]
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut tuple1 = (v, vec![0,1,2]);
    println!("{:?}", tuple1);
    let third: &i32 = &tuple1.0[2];
    println!("{}", third);
    match tuple1.1.get(5) {
    // match tuple1.1.get(0) {
        Some(x) => println!("{}", x),
        None => println!("None")
    }
    // let idx100 = &tuple1.1[100]; // out of index.
    // println!("{}", idx100);
    let idx100 = tuple1.1.get(100);
    match idx100 {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    let (mut v, _) = tuple1;
    let first = &v[0];
    // v.push(1);
    println!("{}", first);
    for i in &mut v {
        *i += 100;
    }
    for i in &v {
        println!("{}", i);
    }

    let row = vec![SpreadsheetCell::Int(10), SpreadsheetCell::Float(1.111), SpreadsheetCell::Text(String::from("How Are You!"))];
    for r in &row {
        println!("{:?}", r);
        match r {
            SpreadsheetCell::Int(x) => println!("{}", x),
            _ => println!("Not Care About It!"),
        }
    }
}
