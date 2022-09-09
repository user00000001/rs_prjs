#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    let x: i32; // not binding
    let y: i32 = 100; // binding value
    println!("{}", y);

    // println!("{}", x); // error: variable x is not initialized
    x = 100; // binding value
    println!("{}", x);

    { let a: i32 = 100; }
    // println!("{}", a); // error: a is out of scope.

    let a: String = String::from("xyz");
    let b = a;
    // println!("{}", a) // error: a is moved

    let a: i32 = 100;
    let b = a;
    println!("{}", a); // type of a implements Copy Trait

    let a: String = String::from("xyz");
    let b = a.clone(); // type of a implements Clone Trait
    println!("{}", a);

    let a: i32 = 100;
    // a = 200;  // error: a is an immutable variable

    let mut a: i32 = 100;
    a = 200;

    let a = vec![1,2,3]; // immutable binding
    let mut a = a; // mutable binding
    a.push(4);
    println!("{:?}", a);

    let mut a: &str = "abc"; // mutable binding
    a = "xyz";
    println!("{:?}", a);

    const PI: f32 = 3.14; // not binding for const
  }

  {
    // pub trait Copy: Clone {}

    #[derive(Copy, Clone)] // 1, use macro to auto implement.
    struct Foo { // can implement Copy Trait. i32,bool have implemented Copy Trait
      a: i32,
      b: bool,
    }
    struct Bar { // can not implement Copy Trait, Vec<T> haven't implemented Copy Trait
      l: Vec<i32>
    }

    #[derive(Debug)]
    struct Foo1 {
      a: i32,
      b: bool,
    }
    impl Copy for Foo1 {}
    impl Clone for Foo1 {
      fn clone(&self) -> Foo1 {
        Foo1 {a: self.a, b: self.b}
      }
    }
    let x = Foo1{a: 100, b: true};
    let mut y = x;
    y.b = false;
    println!("{:?}", x); // b: true
    println!("{:?}", y); // b: false

    let x: i32 = 100;
    let some_closure = move |i: i32| i+x; // i32 implements Copy Trait
    let y = some_closure(2);
    println!("x={}, y={}", x, y);

    let mut x: String = String::from("abc");
    let mut some_closure = move |c: char| x.push(c);
    let y = some_closure('d');
    // println!("x={:?}", x); // error: String doesn't implements Copy Trait

    let mut x: String = String::from("abc");
    {
      let mut some_closure = |c: char| x.push(c); // mutable borrow
      some_closure('d');
    } // mutable borrow out of scope
    println!("x={:?}", x); // then we can access x variable(borrow rules)
  }

  {
    // borrow rules:
    // 1, multiple immutable borrows(or access) can exist
    // 2, if one(only one) mutable borrow exists in current scope, not allow another borrow(or access first)

    let x: Vec<i32> = vec!(1i32, 2, 3);
    let y = &x; // immutable borrow
    println!("x={:?}, y={:?}", x, y);

    let mut x: i32 = 100;
    {
      let y: &mut i32 = &mut x; // mutable borrow
      *y += 2;
    }
    println!("{}", x);

    let mut x: Vec<i32> = vec!(1i32, 2, 3);
    let y = &mut x; // mutable borrow
    y.push(100);
    println!("{:?}", y);
    drop(y); // destory y first; or not to access x until y out of scope
    println!("{:?}", x);

    let mut x: Vec<i32> = vec!(1i32, 2, 3);
    x.push(10);
    {
      let mut y = &mut x; // mutable borrow of x
      // let y1 = &x; // not allowed, mutable borrow y exists
      y.push(100);

      let z = &mut y; // mutable borrow of y; not break the rules
      // let z1 = &mut y; // not allowed, mutable borrow z exists
      z.push(1000);
      // println!("{:?}", y); // y can not access before the z access;
      println!("{:?}", z); 
      println!("{:?}", y); 
      // all borrows out of scope
    }
    println!("{:?}", x); // x has no borrows
  }

  {
    // lifetime rule:
    // Lifetime(R) ⊆ ( Lifetime(X) ∩ Lifetime(Y) ∩ Lifetime(Z) ∩ Lifetime(...) )

    let a = 100_i32;
    {
      let x = &a;
      println!("{}", x);
      // x is out of scope or lifetime
    }
    // println!("{}", x); // error: x not exists any more.

    fn foo(x: &str) -> &str { // implict: fn foo<'a>(x: &'a str) -> &'a str
      x
      // "hello, world!" // pass: &'static str is longger existence than &'a str
    }

    // fn foo1(x: &str, y: &str) -> &str { // error: can not implict, can not compare a,b,c 's lifetime: fn foo1(x: &'a str, y: &'b str) -> &'c str
    //   if true {
    //     x
    //   } else {
    //     y
    //   }
    // }

    fn foo1<'a>(x: &'a str, y: &'a str) -> &'a str { // explict lifetime
      if true {
        x
      } else {
        y
      }
    }

    // fn foo2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { // error: can not compare 'a, 'b lifetime
    //   if true {
    //     x
    //   } else {
    //     y
    //   }
    // }

    fn foo2<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str { // lifetime: 'a <= 'b, match the lifetime rules
      if true {
        x
      } else {
        y
      }
    }

    struct Person<'a> { // rule: struct object lifetime <= fields in struct lifetime
    // struct Person { // error: need explict lifetime
      // age: &u8,
      age: &'a u8,
    }
    struct Person1<'a, 'b: 'a> {
      age: &'a u8,
      age1: &'b u8,
    }
    let x = 20_u8;
    let stormgbs = Person {
      age: &x
    };
    impl<'a, 'b: 'a> Person<'a> { // rule like funciton, the same with enum
      fn print_age(&self) {
        println!("Persion.age = {}", self.age);
      }
      fn get_age(&self) -> &u8 { // implict: fn get_age(&'a self) -> &'a u8
        self.age
      }
      fn get_max_age(&'a self, p: &'b Person) -> &'a u8 {
        if self.age > p.age {
          self.age
        } else {
          p.age
        }
      }
    }
  }
}