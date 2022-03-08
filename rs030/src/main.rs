use std::fmt::{ Display, Formatter, Result };
use std::ops::{ Deref,DerefMut };

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{ Cons, Nil };


#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Display> Display for MyBox<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop mybox's instance: {}", self.0);
    }
}

struct T1 {
    name: String,
}

impl Drop for T1 {
    fn drop(&mut self) {
        println!("drop t type's instance: {}", self.name)
    }
}

impl Display for T1 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

fn main() {
    let data_in_heap = Box::new(5);
    println!("Hello, `{}` from heap!", data_in_heap);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);
    println!("Result: {}, {}", x == *y, x == *(y.deref()));
    drop(y);

    let m = MyBox::new("Rust".to_owned());
    hello(&m);
    hello(&(*m));
    hello(&(*m)[..]);
    drop(m);

    let mut t = MyBox::new(T1{ name: "t1".to_owned()});
    hello_t(&t);
    hello_m_t(&mut t);
}

fn hello(w: &str) {
    println!("Hello, {}!", w);
}

fn hello_t(t: &T1) {
    println!("Hello T, {}!", t.name)
}

fn hello_m_t(t: &mut T1) {
    println!("Hello mut T, {}!", t.name)
}