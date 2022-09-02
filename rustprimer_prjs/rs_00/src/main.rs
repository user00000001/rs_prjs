#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]

fn main() {
  {
    say_hi("ruster");
    fn say_hi(name: &str) {
        println!("Hi, {}", name);
    }

    fn hi(name: &str) {
        println!("Hi, {}.", name);
    }
    fn hello(name: &str) {
        println!("Hello, {}.", name)
    }
    fn say_what(name: &str, func: fn(&str)) {
        func(name)
    }
    let xm = "xiaoming";
    let xh = "xiaohong";
    say_what(xm, hi);
    say_what(xh, hello);

    fn print_id((name, age): (&str, i32)){
        println!("I'm {}, age {}", name, age);
    }
    fn print_age((_, age):(&str, i32)) {
        println!("My age is {}", age);
    }
    fn print_name((name,_):(&str, i32)) {
        println!("I am {}", name);
    }
    let xm = ("xiaoming", 54);
    let xh = ("xiaohong", 66);
    print_id(xm);
    print_id(xh);
    print_name(xm);
    print_age(xm);
    print_name(xh);
    print_age(xh)
  }

  {
    let a = 3;
    println!("{}", inc(a));
    fn inc(n:i32)-> i32 { n + 1 }

    let a = [1,3,2,5,9,8];
    println!("There is 7 in the array: {}", find(7, &a));
    println!("There is 8 in the array: {}", find(8, &a));
    fn find(n: i32, a: &[i32]) -> bool {
        for i in a {
            if *i == n {
                return true;
            }
        }
        false
    }

    fn pow_2_3(n: i32) -> (i32, i32) {
        (n*n, n*n*n)
    }
    let (p2, p3) = pow_2_3(789);
    println!("pow 2 of 789 is {}.", p2);
    println!("pow 3 of 789 is {}.", p3);

    println!("hello");
    // diverging();
    println!("world");
    fn diverging() -> ! {
        panic!("This function will never return")
    }
  }

  {
    // Statement

    // variable declaration
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);

    // item declaration
    // function/structure/type/static/trait/implementation/module


    // Expression

    // literal expression
    // (); // unit type
    // "hello"; // string type
    // '1'; // character type
    // 15; // integer type

    // Tuple expression
    // (0.0, 4.5);
    // ("a", 4usize, true);
    // (0,); // still be tuple, single-element tuple
    // (0); // zero

    // structure expression
    // Point {x: 10.0, y: 20.0};
    // TuplePoint(10.0,20.0);
    // let u = game::User {name: "Joe", age: 35, score: 100_000};
    // some_fn::<Cookie>(Cookie);
    // let base = Point3d {x: 1, y: 2, z: 3};
    // Point3d {y: 0, z: 10, ..base};

    // block expression
    let x: i32 = { println!("Hello."); 5};
    let x: () = { println!("Hello."); };
    
    // range expression
    // 1..2;    // std::ops::Range
    // 3..;     // std::ops::RangeFrom
    // ..4;     // std::ops::RangeTo
    // ..;      // std::ops::RangeFull

    // if expression
    let a = 9;
    let b = if a%2 == 0 { "even" } else { "odd" };

    // path expression
    // mehond-call expression
    // field expression
    // array expression
    // index expression
    // unary operator expression
    // binary operator expression
    // return expression
    // grouped expression
    // match expression
    // if expression
    // lambda expression
    // ... ...
  }

  {
    fn inc(n: i32) -> i32 { n + 1 } // function defination
    type IncType = fn(i32) -> i32;  // function type
    let func: IncType = inc;
    println!("3 + 1 = {}", func(3));
    println!("3 + 1 = {}", inc(3));

    fn dec(n: i32) -> i32 { n - 1 }
    fn process(n: i32, func: fn(i32)->i32) -> i32 {
        func(n)
    }
    println!("3 + 1 = {}", process(3, inc));
    println!("3 - 1 = {}", process(3, dec));

    fn process1<F>(n: i32, func: F) -> i32
    where F: Fn(i32) -> i32 {
        func(n)
    }
    println!("3 + 1 = {}", process1(3, inc));
    println!("3 - 1 = {}", process1(3, dec));

    let a = [1,2,3,4,5,6,7];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i));
    }
    println!("{:?}", b);
    fn get_func(n: i32) -> fn(i32) -> i32 {
        fn inc(n: i32) -> i32 { n + 1 }
        fn dec(n: i32) -> i32 { n - 1 }
        if n%2 == 0 { inc } else { dec } // function should not use the variable outside of fucntion scope 
    }
  }
}