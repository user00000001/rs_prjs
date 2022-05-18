// #[cfg(test)]
mod front_of_house {
    pub use super::serve_order;
    #[derive(Debug)]
    pub struct S1();
    impl S1 {
        pub fn print(&self) {
            println!("{:?}", self);
        }
    }
    pub enum E1 {
        First,
        Second,
    }
    pub mod hosting {
        #[test]
        fn name() {
            add_to_waitlist();
            println!("hosting test");
        }
        pub use super::serve_order;
        pub fn add_to_waitlist() {
            super::serve_order();
            crate::serve_order();
            super::it_works();
            let s1 = super::S1();
            s1.print();
        }
    }
    #[test]
    fn name() {
        println!("front_of_house test");
        assert_eq!(2 + 2, 4);
    }
    fn it_works() {
        super::serve_order();
        serve_order();
    }
}

pub fn serve_order () {}

use crate::front_of_house as foh;
use self::front_of_house::hosting;
use front_of_house::hosting::{self as hosting0, add_to_waitlist};

mod front_of_house1;
use front_of_house1::hosting1::*;

#[allow(unused)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    foh::hosting::serve_order();
    hosting::add_to_waitlist();
    hosting0::add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist1();
    front_of_house1::hosting::add_to_waitlist2();
    let s1 = front_of_house::S1();
    s1.print();
    let first = crate::front_of_house::E1::First;
    let second = front_of_house::E1::Second;
}