
#[allow(unused_mut, unused_variables)]
fn main() {
    let str = "Hello, world!";
    // #[allow(unused_mut)]
    let mut string = str.to_string();
    println!("{}", string);

    let a = [0,1,2,3,4];
    // #[allow(unused_variables)]
    let middle = &a[1..4];
    // #[allow(unused_mut)]
    let mut ten_zeros: [i64; 10] = [0; 10];
    println!("{:?}", ten_zeros);

    let tuple: (i32, &str) = (50, "hello");
    // #[allow(unused_variables)]
    let (fifty, _) = tuple;
    // #[allow(unused_variables)]
    let hello = tuple.1;
    println!("{:?}", tuple);

    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe{ *raw };
    println!("{:?}", points_at);

    fn foo(x: i32) -> i32 {x}
    let bar: fn(i32) -> i32 = foo;
    println!("{:?}", bar);

    let a = String::from("aaaaaaaaaa");
    // print_str(a);
    println!("{:#?}", print_str(a));
}

fn print_str(s: String) -> String {
    println!("{:#?}", s);
    let mut b = String::new();
    b.insert_str(0, "String: ");
    let s = b + &s;
    s
}