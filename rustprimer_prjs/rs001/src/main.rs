use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn foo<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if true {
        x
    } else {
        y
    }
}

fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

fn main() {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::cell::Cell;
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    let shared_map1: Rc<RefCell<_>> = Rc::new(RefCell::new(Point{x: 1.1f32, y: 1.1f32}));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
    let b_shared_map = shared_map.borrow();
    println!("{:?}\n{:?}\n{:?}", shared_map.borrow(), b_shared_map, shared_map1.borrow_mut());
    let c = Cell::new(5);
    let five = c.get();
    println!("{:?}", five);
    c.set(10);
    println!("{:?} {:?}", c.get(), shared_map1);
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));
    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
    println!("{:?}", foo("arg1", "arg2"));
    let f = factory();
    let answer = f(1);
    // assert_eq!(7, answer);
    assert!(7 != answer);
}