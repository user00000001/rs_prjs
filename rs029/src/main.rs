// use rand;

fn main() {
    println!("Hello, world!");
    add_one(1);
}
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one(arg);

/// assert_eq!(6, answer);
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}