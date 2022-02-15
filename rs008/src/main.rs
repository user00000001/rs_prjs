// #[allow(unused_mut)]
fn main() {
    // let mut x = 5;
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    let x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("max points: {}", MAX_POINTS);
    #[allow(unused_mut)]
    let mut spaces = "      ";
    println!("spa{}cs", spaces);
    // spaces = spaces.len();
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}
