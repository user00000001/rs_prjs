use std::cell::RefCell;
use hello_macro_derive::{HelloMacro, make_answer, show_streams};
use hello_macro::HelloMacro;

#[macro_export]
macro_rules! vec1 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

struct Pancakes;

#[derive(HelloMacro)]
struct Pancakes1;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams { delimiters } ]
fn invoke4() {}

fn main() {
    let v1 = vec1![1,2,4,8];
    let a = {10};
    println!("{:?}, {}", v1, a);
    Pancakes::hello_macro();
    
    Pancakes1::hello_macro();

    make_answer!();
    println!("{}", answer());
    invoke1();
    invoke2();
    invoke3();
    invoke4();

    // Used as an expression.
    let _x = vec![1,2,3];

    // Used as a statement.
    println!("Hello!");

    // Used in a pattern.
    macro_rules! pat {
        ($i:ident) => {
            Some($i)
        };
    }
    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
        println!("{}", x);
    }

    // Used in a type.
    macro_rules! Tuple {
        { $A:ty, $B:ty } => {
            ($A, $B)
        };
    }
    type N2 = Tuple!(i32, i32);
    let a: N2 = (12,34);
    println!("{:?}", a);

    // Used as a item.
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    // Used as an associated item.
    macro_rules! const_maker {
        ($t:ty, $v:tt) => {
            const CONST: $t = $v;
        };
    }
    trait T {
        const_maker!{i32, 7}
    }

    // Macro calls within macros.
    macro_rules! example {
        () => {
            println!("Macro call in a macro!")
        };
    }
    example!() // Outer macro `example` is expanded, then inner macro `println` is expanded.
}
