#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List1 {
    Cons1(Rc<RefCell<i32>>, Rc<List1>),
    Nil1,
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, RefCell<Rc<List2>>),
    Nil2,
}

use crate::List::{Cons,Nil};
use crate::List1::{Cons1,Nil1};
use crate::List2::{Cons2,Nil2};
use std::rc::{ Rc,Weak };
use std::cell::RefCell;

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.

// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Nil2 => None,
            Cons2(_, item) => Some(item),
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        #[allow(unused)]
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("{:?}", (a, b));
    
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons1(Rc::clone(&value), Rc::new(Nil1)));

    let b = Cons1(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons1(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a = Rc::new(Cons2(5, RefCell::new(Rc::new(Nil2))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next time = {:?}", a.tail());

    let b = Rc::new(Cons2(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    // // crash, overflow the stack
    // println!("a next time = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    }); // leaf s 1, w 0

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // leaf s 2, w 0
        }); // branch s 1, w 0

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // branch(downgrade to weak) s 1, w 1

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    } // branch(dropped) s 0, w 0; leaf s 1, w 0

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // branch(Option) now None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
