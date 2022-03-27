use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
//-----------------
type Result1<T> = std::result::Result<T, std::io::Error>;

pub trait Write1 {
    fn write(&mut self, buf: &[u8]) -> Result1<usize>;
    fn flush(&mut self) -> Result1<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result1<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result1<()>;
}


fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
//----------------
    let f: Box<dyn Fn() + Send + 'static> = Box::new(||println!("hi"));
    fn takes_returns_long_type(f: Box<dyn Fn() + Send + 'static>) -> Box<dyn Fn() + Send + 'static> {
        f
    }
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f1: Thunk = Box::new(||println!("hi"));
    fn takes_returns_long_type1(f: Thunk) -> Thunk {
        f
    }
//---------------
    fn bar() -> ! { // function never return
        print!("forever");
        loop { // ! type
            print!("and ever ");
            continue; 
            // break; // continue/break have ! type as the empty/never type, 

            // let guess: u32 = match guess.trim().parse() {
            //     Ok(num) => num,
            //     Err(_) => continue, // empty type, guess is u32 type.
            // };

            // impl<T> Option<T> {
            //     pub fn unwrap(self) -> T {
            //         match self {
            //             Some(val) => val,
            //             None => panic!("called `Option::unwrap()` on a `None` value"), // never type, val is T type
            //         }
            //     }
            // } 
        }
    }
//--------------
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?"; s1,s2 have diff str type sizes, not like i32/u32 type do. 
    trait A {}
    // &dyn A / Box<dyn Trait> / Rc<dyn Trait> (pointer like)
    fn generic<T: Sized>(t: T) { // T must be a known size type at compile time.

    }
    fn generic1<T: ?Sized>(t: &T) { // T may or may not be Sized, from T to &T (pointer like)

    }
}
