#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {  
  use std::ops::Add;
  {
    // override + operator
    #[derive(Debug)]
    struct Complex {
      a: f64,
      b: f64,
    }
    impl Add for Complex {
      type Output = Complex; // explict output type
      fn add(self, other: Complex) -> Self::Output /* or Complex */ {
        Complex{a: self.a+other.a, b: self.b+other.b}
      }
    }

    impl Add<i32> for Complex {
      type Output = f64;
      fn add(self, rhs: i32) -> Self::Output {
          self.a + self.b + (rhs as f64)
      }
    }

    let cp1 = Complex{a: 1f64, b: 2.0};
    let cp2 = Complex{a: 5.0, b: 8.1};
    let cp3 = cp1 + cp2; // impl Add
    println!("{:?}", cp3);
    println!("{}", cp3 + 10i32); // impl Add<i32>
  }

  use std::ops::Mul;
  {
    // trait contraint
    trait HasArea<T> {
      fn area(&self) -> T;
    }
    struct Square<T> {
      x: T,
      y: T,
      side: T,
    }
    impl<T> HasArea<T> for Square<T>
      where T: Mul<Output=T> + Copy
    {
      fn area(&self) -> T {
          self.side * self.side // Copy won't move value
      }
    }
    let s = Square {
      x: 0.0f64,
      y: 0.0f64,
      side: 12.0f64,
    };
    println!("Area of s: {}", s.area());
  }

  Ok(())
}