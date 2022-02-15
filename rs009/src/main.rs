#[allow(unused_mut, non_snake_case)]
#[deny(unused_variables)]
fn main() {
    let mut guess: i32 = "-42".parse().expect("Wrong Number!");
    println!("guess: {}", guess);
    guess = 101;
    println!("Guess: {}", guess);
    let True = true;
    let False: bool = false;
    println!("{:#?}, {:#?}", True, False);
    // let result = 111 * 1;
    let result = '😘';
    println!("{} {{}}", result);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?} {}", tup, tup.1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let (a, _, b) = (tup.0, tup.1, 111);
    println!("a: {}, b: {}", a, b);
    let a = [1,2,3,4];
    println!("a: {:#?}", a);
    let b: [u8;5] = [2,3,4,5,6];
    let c = [3;2];
    println!("b: {:#?}, c: {:#?}, c[0]: {}", b, c, c[0]);
    // let a: u32 = 1;
    let a: usize = 0;
    println!("c[0]: {}", c[a])
}
