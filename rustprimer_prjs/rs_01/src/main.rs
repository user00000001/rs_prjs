#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]
use std::cell::Cell;

fn main() {
    {
        println!("hello, world");
        let rust = "Rust";
        println!("Hello, {}!", rust);
    }

    {
        let a1 = 5;
        let a2: i32 = 5;
        assert_eq!(a1, a2);
        let b1: u32 = 5;
        // assert_eq!(a1, b1) // error: mismatched types

        let mut a: f64 = 1.0;
        let b = 2.0f32;
        a = 2.0;
        println!("{:?}", a);
        // let a = a;
        // a = 3.0 // error: not mutable
        // assert_eq!(a, b) // error: type not matched

        let (a, mut b): (bool, bool) = (true, false);
        println!("a = {:?}, b = {:?}", a, b);
        // a = false // error: not mutable
        b = true;
        assert_eq!(a, b);
        
        // boolean type
        let t = true;
        let f: bool = false;

        // char type
        let c = 'c';

        // numeric types
        let x = 42;
        let y: u32 = 123_456;
        let z: f64 = 1.23e+2;
        let zero = z.abs_sub(123.4);
        let bin = 0b1111_0000;
        let oct = 0o7320_1546;
        let hex = 0xf23a_b049u32;

        // string types
        let str = "Hello, world!";
        let mut string = str.to_string();

        // arrays and slices
        let a = [0,1,2,3,4];
        let middle = &a[1..4];
        let mut ten_zeros: [i64; 10] = [0; 10];

        // tuple
        let tuple: (i32, &str) = (50, "hello");
        let (fifty, _) = tuple;
        let hello = tuple.1;

        // raw pointers
        let x = 5;
        let raw = &x as *const i32;
        let points_at = unsafe { *raw };

        // functions
        fn foo(x: i32) -> i32 { x }
        let bar: fn(i32)->i32 = foo;

        // explict conversion
        let decimal = 65.4321_f32;
        let integer = decimal as u8;
        let character = integer as char;

        // types aliases
        type NanoSecond = u64;
        type Point = (u8, u8);        
    }

    {
        let mut array: [i32; 3] = [0; 3];
        array[1] = 1;
        array[2] = 2;
        assert_eq!([1, 2], &array[1..]);
        // This loop prints: 0 1 2
        for x in &array {
            println!("{} ", x);
        }

        let v: Vec<i32> = Vec::new();
        let v: Vec<i32> = vec![];
        let v = vec![1,2,3,4,5];
        let v = vec![0; 10];
        let mut v = vec![1,2];
        let two = v.pop();
        let mut v = vec![1,2,3];
        let three = v[2];
        v[1] = v[1] + 5;

        let hello = "Hello, world!";
        let hello: &'static str = "Hello, world!";

        let mut s = String::new();
        let mut hello = String::from("Hello, ");
        hello.push('w');
        hello.push_str("orld!");

        let mut s = String::from("foo");
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('o'));
        assert_eq!(s.pop(), Some('f'));
        assert_eq!(s.pop(), None);
    }

    {
        // structs
        struct Point {
            x: i32,
            // mut x: i32, // not allowed 
            y: i32,
        }
        let point = Point { x: 0, y: 0 };

        // tuple structs
        struct Color(u8, u8, u8);
        let android_green = Color(0xa4, 0xc6, 0x39);
        let Color(red, green, blue) = android_green; 

        // A tuple struct's constructors can be used as functions.
        struct Digit(i32);
        let v = vec![0,1,2];
        let d: Vec<Digit> = v.into_iter().map(Digit).collect();

        // newtype: a tuple struct with only one element
        struct Inched(i32);
        let length = Inched(10);
        let Inched(integer_length) = length;

        // unit-like structs
        struct EmptyStruct;
        let empty = EmptyStruct;

        #[derive(Default)]
        struct Point3d {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point3d::default();
        let point = Point3d{ y: 1, ..origin };
        let Point3d { x: x0, y: y0, .. } = point;

        struct Point1 {
            x: i32,
            y: Cell<i32>,
        }
        let point = Point1 { x: 5, y: Cell::new(6) };
        point.y.set(7);

        mod graph {
            #[derive(Default)]
            pub struct Point {
                pub x: i32,
                y: i32,
            }

            pub fn inside_fn() {
                let p = Point { x: 1, y: 2 };
                println!("{}, {}", p.x, p.y);
            }
        }

        fn outside_fn() {
            let p = graph::Point::default();
            println!("{}", p.x);
            // println!("{}", p.y); // error: y is private
        }
        outside_fn();
        graph::inside_fn();

        // enums
        enum Message {
            Quit,
            ChangeColor(i32, i32, i32),
            Move { x: i32, y: i32 },
            Write(String),
        }
        let x: Message = Message::Move{x: 3, y: 4};
    }

    {
        let x = 5;
        let y = if x == 5 { 10 } else { 15 };
        // let y = (let x = 5); // error: `let x = 5` is a statement, not an expression 
        // let z: i32 = if x == 5 { 10; } else { 15; }; // error: line end with `;` return `unit ()` not match with i32

        for i in [0,1,2].iter() {
            println!("{}", i);
        }

        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");
                break 'outer;
            }
            println!("This point will never be reached");
        }
        println!("Exited the outer loop");

        let day = 5;
        match day {
            0 | 6 => println!("weekend"),
            1 ... 5 => println!("weekday"),
            _ => println!("invalid"),
        }

        let x = 1;
        match x {
            e @ 1...5 => println!("got a range element {}", e),
            _ => println!("anything"),
        }

        let pair = (0, -2);
        match pair {
            (0, y) => println!("x is `0` and `y` is `{:?}`", y),
            (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
            _ => println!("It doesn't matter what they are"),
        }

        struct Point {
            x: i32,
            y: i32,
        }
        let origin = Point { x: 0, y: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        enum OptionalInt {
            Value(i32),
            Missing,
        }
        let x = OptionalInt::Value(5);
        match x {
            OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
            OptionalInt::Value(..) => println!("Got an int!"),
            OptionalInt::Missing => println!("No such luck"),
        }

        let number = Some(7);
        let mut optional = Some(0);
        if let Some(i) = number {
            println!("Matched {:?}", i);
        } else {
            println!("Didn't match a number!");
        }

        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
    }

    {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn diverges() -> ! {
            panic!("This function never returns!");
        }
        // let x: i32 = diverges();
        // let y: String = diverges();

        let num = 5;
        let plus_num = |x: i32| x + num;

        let mut num = 5;
        {
            let mut add_num = move |x: i32| num += x;
            add_num(5);
        }
        assert_eq!(5, num); // num has Copy trait

        fn apply<F>(f: F, y: i32) -> i32
            where F: Fn(i32) -> i32
        {
            f(y) * y
        }

        fn factory(x: i32) -> Box<dyn Fn(i32)->i32> {
            Box::new(move|y|x+y)
        }

        let transform: fn(i32) -> i32 = add_one;
        let f0 = add_one(2i32) * 2;
        let f1 = apply(add_one, 2);
        let f2 = apply(transform, 2);
        println!("{}, {}, {}", f0, f1, f2);
        let closure = |x: i32| x + 1;
        let c0 = closure(2i32) * 2;
        let c1 = apply(closure, 2);
        let c2 = apply(|x| x + 1, 2);
        println!("{}, {}, {}", c0, c1, c2);
        let box_fn = factory(1i32);
        let b0 = box_fn(2i32) * 2;
        let b1 = (*box_fn)(2i32) * 2;
        let b2 = (&box_fn)(2i32) * 2;
        println!("{}, {}, {}", b0, b1, b2);
        let add_num = &(*box_fn);
        let translate: &dyn Fn(i32) -> i32 = add_num;
        let z0 = add_num(2i32) * 2;
        let z1 = apply(add_num, 2);
        let z2 = apply(translate, 2);
        println!("{}, {}, {}", z0, z1, z2);

        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }

        impl Circle {
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: radius
                }
            }
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
        let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
        println!("{}", c.area());
        // use associated function and method chaining
        println!("{}", Circle::new(0.0, 0.0, 2.0).area());
    }

    {
        trait HasArea {
            fn area(&self) -> f64;
        }

        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl HasArea for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            } 
        }

        struct Square {
            x: f64,
            y: f64,
            side: f64,
        }
        impl HasArea for Square {
            fn area(&self) -> f64 {
                self.side * self.side
            }
        }

        fn print_area<T: HasArea>(shape: T) {
            println!("This shape has an area of {}", shape.area());
        }
        print_area(Circle{x: 0.0, y: 0.0, radius: 2.0});
        print_area(Square{x: 0.0, y: 0.0, side: 2.0});

        use std::fmt::Debug;
        fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }
        fn bar<T, K>(x: T, y: K)
            where T: Clone,
                K: Clone + Debug
        {
            x.clone();
            y.clone();
            println!("{:?}", y);
        }

        trait Foo {
            fn foo(&self);
            fn bar(&self) {println!("We called bar.");}
        }
        trait FooBar : Foo {
            fn foobar(&self);
        }
        struct Baz;
        impl Baz {
            fn foo(&self) {
                println!("baz's foo.")
            }
        }
        impl Foo for Baz {
            fn foo(&self) {
                println!("foo");
            }
        }
        impl FooBar for Baz {
            fn foobar(&self) { println!("foobar"); }
        }
        let baz = Baz;
        baz.foo();
        <Baz as Foo>::foo(&baz);
        Foo::foo(&baz);
        baz.foobar();

        let x: Option<i32> = Some(5);
        let y: Option<f64> = Some(5.0f64);

        // generic functions
        fn make_pair<T, U>(a: T, b: U) -> (T, U) {
            (a, b)
        }
        let couple = make_pair("man", "female");

        // generic structs
        struct Point<T> {
            x: T,
            y: T,
        }
        let int_origin = Point { x: 0, y: 0 };
        let float_origin = Point { x: 0.0, y: 0.0 };

        // use generic parameters
        {
            trait Graph<N, E> {
                fn has_edge(&self, n1: &N, n2: &N) -> bool;
                fn edges(&self, n: &N) -> Vec<E>;
            }
            fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
                1
            }
        }
        
        // use associated types
        trait Graph {
            type N;
            type E;
            fn has_edge(&self, n1: &Self::N, n2: &Self::N) -> bool;
            fn edges(&self, n: &Self::N) -> Vec<Self::E>;
        }
        fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 {
            1
        }
        struct Node;
        #[derive(Debug)]
        struct Edge;
        struct SimpleGraph;
        impl Graph for SimpleGraph {
            type N = Node;
            type E = Edge;
            fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
                false
            }
            fn edges(&self, n: &Node) -> Vec<Edge> {
                Vec::new()
            }
        }
        // use std::fmt;
        // impl fmt::Display for SimpleGraph {
        //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //         write!(f, "")
        //     }
        // }
        let graph = SimpleGraph;
        println!("{}, {:#?}", graph.has_edge(&Node, &Node), graph.edges(&Node));
        let object = Box::new(graph) as Box<dyn Graph<N=Node, E=Edge>>;
        println!("{}, {:#?}", object.has_edge(&Node, &Node), object.edges(&Node));
    }
}