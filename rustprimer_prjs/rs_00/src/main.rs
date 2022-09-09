#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    fn add_i8(a: i8, b: i8) -> i8 { a + b }
    fn add_i16(a: i16, b: i16) -> i16 { a + b }
    fn add_f64(a: f64, b: f64) -> f64 { a + b }
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i16: {}", add_i16(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));

    enum Option<T> { Some(T), None }
    let a = Some(100.111f32);
    let a: Option<f32> = Option::Some(100.111);
    let b: Option<f32> = Option::Some(100.111f32);
    let c: Option<f64> = Option::Some(100.111);
    let d: Option<f64> = Option::Some(100.111f64);

    use std::ops::Add;
    #[derive(Debug)]
    struct Point { x: i32, y: i32 }
    impl Add for Point {
      type Output = Point;
      fn add(self, p: Point) -> Point { Point { x: self.x + p.x, y: self.y + p.y } }
    }
    fn add<T: Add<T, Output=T>>(a: T, b: T) -> T { a + b }
    println!("{}", add(100i32, 1i32));
    println!("{}", add(100.11f32, 100.22f32));

    let p1 = Point{x: 1, y: 1};
    let p2 = Point{x: 2, y: 2};
    println!("{:?}", add(p1, p2));
  }

  {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> { x: T, y: T }
    impl<T: Add<T, Output=T>> Add for Point<T> {
      type Output = Point<T>;
      fn add(self, p: Point<T>) -> Point<T> { Point { x: self.x + p.x, y: self.y + p.y } }
    }
    fn add<T: Add<T, Output=T>>(a: T, b: T) -> T { a + b }

    let p1 = Point { x: 1.1f32, y: 1.1f32 };
    let p2 = Point { x: 2.1f32, y: 2.1f32 };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
  }

  {
    fn do_something_or_nothing(lines: std::str::Lines) -> Vec<String> {
      let mut result: Vec<String> = vec![];
      for line in lines {
        result.push(line.to_string());
      }
      result
    }
    fn do_some_other_thing_or_nothing<B: std::io::BufRead>(lines: std::io::Lines<B>) -> Vec<String> {
      let mut result: Vec<String> = vec![];
      for line in lines {
        if let Ok(line) = line {
          result.push(line.to_string());
        }
      }
      result
    }
    fn parse<T: std::fmt::Display>(ts: Vec<T>) { // #TODO lines iterator ?
      for line in ts {
        println!("{}", line);
      }
    }
    let lines = "some\nlong\ntext".lines();
    parse(do_something_or_nothing(lines));

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    let lines = BufReader::new(File::open("/etc/hosts").unwrap()).lines();
    parse(do_some_other_thing_or_nothing(lines));
  }
}