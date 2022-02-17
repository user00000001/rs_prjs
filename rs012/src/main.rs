fn main() {
    let mut s = String::from("Hello, ");
    let (s1, len) = calculate_string_length(&mut s);
    println!("{:#}\n{}", s1, len);
    {
        let r1 = s1;
        println!("{}", r1);
    }
    // let r2 = s1;
    let r2 = &mut s;
    println!("{}", r2);
    let mut h = String::from("hello");
    let r1 = &h;
    let r2 = &h;
    println!("{} {}", r1, r2);
    let r3 = &mut h;
    println!("{}", r3);
    // let reference_to_nothing = dangle();
}

fn calculate_string_length(s: &mut String) -> (&mut String, usize) {
    let length = s.len();
    s.push_str("world!");
    (s, length)
}

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }