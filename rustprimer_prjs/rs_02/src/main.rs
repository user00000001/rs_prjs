#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]

// extern crate rs_00;
// // or
// use rs_00;

/* 
    multi lines comment
*/

//! # The first line
//! The second line
/// Adds one to the number given.
///
/// # Examples
///
///
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
/*!
  check the result
*/
    x + 1
}

use rs_00::{chinese, english};
use rs_00::{chinese1::greetings::hello as hello1, english1::{farewells::*, hello}};

fn main() {
    println!("{}", hello1());
    println!("{}", hello());
    println!("{}", goodbye());
}