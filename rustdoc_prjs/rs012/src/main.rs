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

    let mut hw = String::from("Hello, world!");
    let w = hw.clone();
    // let fw = first_world(&hw);
    println!("{:?}", first_world(&hw));
    hw.clear();
    // println!("{}", fw);
    println!("{:?}", second_world(&hw));
    println!("{:?}", first_world("hello world"));
    println!("{:?}", first_world(&w[..]));

    let hello = &w[0..5];
    let world = &w[7..=11];
    let h1 = &w[..5];
    let w1 = &w[7..w.len()-1];
    println!("{:#?}", (hello, world, h1, w1));

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
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

// fn first_world(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_world(s: &str) -> &str {
// fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    return &s[..]
}


fn second_world(s: &String) -> (usize, usize) {
    (s.len(), s.len())
}