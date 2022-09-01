#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]

//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    {
        // A comment
        let x = 5;
        let y = 6; // B comment

/*
    let x = 42;
    println!("{}", x);
*/
    }

    {
        let x = 5;
        let y = if x == 5 { 10 } else { 15 }; // y: i32

        let x = Some(5);
        if let Some(y) = x { 
            println!("{}", y); // y is 5
        }
        let z = if let Some(y) = x {
            y
        } else { 
            0 
        }; // z is 5

        // equals above
        let x = Some(5);
        match x {
            Some(y) => println!("{}", y),
            None => ()
        }
        let z = match x {
            Some(y) => y,
            None => 0
        };

        let x = 5;
        match x {
            1 => {
                println!("One")
            },
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            5 => println!("five"),
            _ => println!("Something else"),
        }

        for x in 0..10 {
            println!("{}", x); // x: i32
        }
        for (i, j) in (5..9).enumerate() {
            println!("i = {} and j = {}", i, j);
        }

        let lines = "Content of line one
Content of line two
Content of line three
Content of line four".lines();
        for (linenumber, line) in lines.enumerate() {
            println!("{}: {}", linenumber, line);
        }

        let mut x = 5;
        let mut done = false;
        while !done {
            x += x - 3;
            println!("{}", x);
            if x % 5 == 0 {
                done = true;
            }
        }

        // while true {
        //     // do someting
        // }
        // // equals above
        // loop {
        //     // do someting
        // }

        let mut x = 5;
        loop {
            x += x - 3;
            println!("{}", x);
            if x % 5 == 0 { break; }
        }

        for x in 0..10 {
            if x  % 2 == 0 { continue; }
            println!("{}", x);
        }

        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 { continue 'outer; } // continues the loop over x
                if y % 2 == 0 { continue 'inner; } // continues the loop over y
                println!("x: {}, y: {}", x, y);
            }
        }
    }

    {
        // bool
        let is_she_love_me = false;
        let mut is_he_love_me: bool = true;

        // char
        let c = 'x';
        let cc = '王';

        // number
        let a = 0i8;
        let b = 0u16;
        let c = 0f32;
        let d = 0usize;
        let d = 0isize;

        // array
        let a = [8, 9, 10];
        let b: [u8; 3] = [8, 6, 5];
        println!("{}", a[0]);

        // slice
        let arr = [1,2,3,4,5,6];
        let slice_complete = &arr[..]; // all elements
        let slice_middle = &arr[1..4]; // [2, 3, 4]
        let slice_right = &arr[1..]; // [2,3,4,5,6]
        let slice_left = &arr[..3]; // [1,2,3]

        fn show(arr: &[u8]) {
            for i in arr {
                print!("{} ", i);
            }
            println!("");
        }
        let a: [u8;3] = [1,2,3];
        let slice_a = &a[..];
        show(slice_a);
        let b: [u8;4] = [1,2,3,4];
        show(&b[..]);

        // vec
        let mut v1: Vec<i32> = vec![1,2,3]; // macro
        let v2 = vec![0;10];
        println!("{}", v1[0]);
        for i in &v1 {
            print!("{}", i); // transfer &Vec<i32> to &[i32] by Deref
        }
        println!("");
        for i in &mut v1 {
            *i = *i+1;
            print!("{}", i);
        }
        println!("");

        // raw str
        let raw_str: &str = "";

        // Functions
        fn foo(x: i32) -> i32 { x + 1 }
        let x: fn(i32) -> i32 = foo;
        assert_eq!(11, x(10));
    }

    {
        // Tuple 
        let y = (2, "hello world");
        let x: (i32, &str) = (3, "world hello");

        let (w, z) = y; // w = 2, z = "hello world"

        let f = x.0; // f = 3;
        let e = x.1; // e = "world hello"

        // struct
        struct A {
            attr1: i32,
            attr2: String,
        }
        struct B(i32, u16, bool);
        struct C {}; // error before rust 1.9
        struct D;

        struct Person {
            name: String,
        }
        impl Person {
            fn new(n: &str) -> Person {
                Person {
                    name: n.to_string()
                }
            }
            fn greeting(&self) {
                println!("{} say hello.", self.name);
            }
        }
        let peter = Person::new("Peter");
        peter.greeting();

        // ref in struct
        struct RefBoy<'a> {
            loc: &'a i32, // ref field in struct must have an explict lifetime
        }

        struct A1 {
            a: i32,
        }
        impl A1 {
            pub fn show(self) {
                println!("{}", self.a)
            }
        }
        let ast = A1 { a: 12i32 };
        ast.show();
        // println!("{}", ast.a); // error: ast was moved to show method, then destoried after invoke show

        #[derive(Copy, Clone)]
        struct A2 {
            a: i32,
        }
        impl A2 {
            pub fn show(&self) {
                println!("{}", self.a);
                // self.add_one(); // error: self is immutable
            }
            pub fn add_two(&mut self) {
                self.add_one();
                self.add_one();
                self.show();
            }
            pub fn add_one(&mut self) {
                self.a += 1;
            }
        }
        let mut ast = A2 { a: 12i32 };
        ast.show();
        ast.add_two();

        // enum
        enum Direction {
            West, 
            North,
            Sourth,
            East,
        }
        enum SpecialPoint {
            Point(i32, i32),
            Special(String),
        }
        enum SpecialPoint1 {
            Point {
                x: i32,
                y: i32,
            },
            Special(String)
        }
        let sp = SpecialPoint::Point(0, 0);
        match sp {
            SpecialPoint::Point(x, y) => { // SpecialPoint1::Point{ x: x, y: y } => {
                println!("I'am SpecialPoint(x={}, y={})", x, y);
            }
            SpecialPoint::Special(why) => {
                println!("I'am Special because I am {}", why);
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 1, y: 2 };
        let Point { x: _x, y: _y } = point;
        let Point {x, y} = point;
        let Point { x: _x, ..} = point;
    }

    {
        let x = "Hello";
        let x: &'static str = "Hello";
        let z = "foo
bar";
        let w = "foo\nbar";
        assert_eq!(z, w);
        let d: &'static str = r"abc \n abc";
        let c: &'static str = "abc \\n abc";
        assert_eq!(d, c);

        let x: &'static str = "hello";
        let mut y: String = x.to_string();
        println!("{}", y);
        y.push_str(", world");
        println!("{}", y);

        fn use_str(s: &str) {
            println!("I am: {}", s);
        }
        let s = "Hello".to_string();
        use_str(&*s); // * Deref, & turn str to &str
        use_str(&s); // * Deref(default, add multi *), & turn str to &str

        let miao = vec![229, 150, 181];
        let meow = String::from_utf8(miao).unwrap();
        assert_eq!("喵", meow);

        let x = "hello".to_string();
        // x[1]; // compile error: can be indexed
        let x = "哎哟我去".to_string();
        for i in x.as_bytes() {
            print!("{} ", i);
        }
        println!("");
        for i in x.chars() {
            print!("{}", i);
        }
        x.chars().nth(2);
        println!("");
    }

    {
        // ops 

        // *    Deref

        // +    std::ops::Add
        // -    std::ops::Sub
        // *    std::ops::Mul
        // /    std::ops::Div
        // %    std::ops::Rem

        // &    std::ops::BitAnd
        // |    std::ops::BitOr
        // ^    std::ops::BitXor
        // <<   std::ops::BitShl
        // >>   std::ops::BitShr

        // ==   PartialEq
        // !=   PartialEq
        // >    PartialOrd
        // <    PartialOrd
        // >=   PartialOrd
        // <=   PartialOrd // Ord Eq not match float NaN

        fn avg(vals: &[f64]) -> f64 {
            let sum: f64 = vals.iter().sum();
            let num: f64 = vals.len() as f64;
            sum / sum
        }

        use std::ops::{Add, Sub};
        #[derive(Copy, Clone)]
        struct A(i32);
        impl Add for A {
            type Output = A;
            fn add(self, rhs: A) -> A {
                A(self.0 + rhs.0)
            }
        }
        impl Sub for A {
            type Output = A;
            fn sub(self, rhs: A) -> A {
                A(self.0 - rhs.0)
            }
        }
        let a1 = A(10i32);
        let a2 = A(5i32);
        let a3 = a1 + a2;
        println!("{}", (a3).0);
        let a4 = a1 - a2;
        println!("{}", (a4).0);
    }

    {
        let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子", 81, "wayslog", width=4, height=178);
        // < left, > right, ^ center
        println!("{}\n{}\n{}", s, format!("{:b}", 2), format!("{:?}", "Hello"));

        // format_string := <text> [ format <text> ] *
        // format := '{' [ argument ] [ ':' format_spec ] '}'
        // argument := integer | identifier
        // format_spec := [[fill]align][sign]['#'][0][width]['.' precision][type]
        // fill := character
        // align := '<' | '^' | '>'
        // sign := '+' | '-'
        // width := count
        // precision := count | '*'
        // type := identifier | ''
        // count := parameter | integer
        // parameter := integer '$'

        println!("rust.cc社区的唐{CaiNiao}眼睛度数足有{0:0>4$.1}度却还是每天辛苦码代码才能赚到100个{3}。\n但是{2}却只需睡{1:^8}个小时就可以迎娶白富美了。", 500.0, 12, "ELTON", "QB", 4, CaiNiao="Mike");
        // rust.cc社区的唐Mike眼睛度数足有0500.0度却还是每天辛苦码代码才能赚到100个QB。
        // 但是ELTON却只需睡  12  个小时就可以迎娶白富美了。
    }
}