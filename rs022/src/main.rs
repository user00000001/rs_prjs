use std::cmp::{ PartialOrd, Ordering };
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Mul<Output=T> + PartialEq + Copy> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        (self.x * self.y).eq(&(other.x * other.y))
    }
}

impl<T: Mul<Output=T> + PartialOrd + Copy> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        (self.x * self.y).partial_cmp(&(other.x * other.y))
    }
}

impl<T: Mul<Output=T> + Copy> Point<T> {
    fn area(&self) -> T {
        self.x * self.y
    }
}

impl Point<f64> {
    fn show_me(&self) {
        println!("F64: {:?}", self)
    }
}

impl Point<i32> {
    fn show_me(&self) {
        println!("I32: {:?}", self)
    }
}

fn largest<T>(list: &[T]) -> T 
where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![324, 501, 325, 1000, 165];
    println!("{:?}'s largest element is {}", number_list, largest(&number_list));
    let char_list = vec!['9', 'A', 'c', 'b', 'a', 'z', '\n'];
    println!("{:?}'s largest char is {}", char_list, largest(&char_list));
    let point_float_list = vec![Point{x: 100.0, y: 30.0}, Point{x:15.0, y: 50.1}, Point{x: 42.1, y: 100.1}, Point{x: 22.7, y: 44.8}];
    let lpfl = largest(&point_float_list);
    println!("{:?}'s largest char is {:#?}, area is {}", point_float_list, lpfl, lpfl.area());
    lpfl.show_me();
    let point_float_list = vec![Point{x: 100, y: 30}, Point{x:15, y: 50}, Point{x: 42, y: 100}, Point{x: 22, y: 44}];
    let lpfl = largest(&point_float_list);
    println!("{:?}'s largest char is {:#?}, area is {}", point_float_list, lpfl, lpfl.area());
    lpfl.show_me();
}
