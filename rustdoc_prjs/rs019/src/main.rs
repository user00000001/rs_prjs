fn main() {
    let s = String::from("Hello");
    let s1 = "Hello".to_string();
    let mut s2 = String::new();
    let s3 = "H".to_string() + "e" + &"ll".to_string() + "o";
    let s4 = format!("{}", "Hello");
    s2.push_str("Hello");
    assert_eq!(s, s1);
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(s3, s4);
    println!("Hello, world!");
    // let l = s4[0]; // Error on wchar unicode have more size
    for w in "好呀ｓ💣1a!".chars() {
        println!("{:?}", (w, w as u32, w.len_utf8()));
    }
}
