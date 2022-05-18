use std::fmt;
use std::ops::Add;

struct Counter {
    count:i32
}

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator1<T> {
    fn next1(&mut self) -> Option<T>;
}

impl Iterator for Counter { // implement once
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 5 {
            self.count -= 1;
            Some(self.count as Self::Item)
        } else {
            self.count -= 1;
            None
        }
    }
}

impl Iterator1<u32> for Counter { // multiple implements for more types, u32
    fn next1(&mut self) -> Option<u32> {
        if self.count > 5 {
            self.count -= 1;
            Some(self.count as u32)
        } else {
            self.count -= 1;
            None
        }
    }
}

impl Iterator1<i32> for Counter { // multiple implements for more types, i32
    fn next1(&mut self) -> Option<i32> {
        if self.count > 5 {
            self.count -= 1;
            Some(self.count)
        } else {
            self.count -= 1;
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0+(other.0*1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let mut count = Counter { count: 10};
    while let Some(c) = count.next() {
        println!("count by next: {}", c);
        break
    }
    // while let Some(c) = count.next1() { // which implement to choose?
    //     println!("count by next1: {}", c);
    // }
//-----------------
    println!("{:#?}\n{:#?}", Point {x: 1, y: 0} + Point {x: 2, y:3}, Point {x: 3, y: 3});
//-----------------
    let meters = Meters(10);
    let millimeters = Millimeters(10);

    println!("{:?}", millimeters + meters);
//-----------------
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
//-----------------
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
//-----------------
    let p = Point { x: 32, y: 23};
    p.outline_print();
//-----------------
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
