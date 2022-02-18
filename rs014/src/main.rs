#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle_tuple = (100, 200);
    println!("{}", area_for_tuple(rectangle_tuple));
    println!("{}", area(Rectangle{width: rectangle_tuple.0, height: rectangle_tuple.1}));
    println!("{:#?}", Rectangle{..Rectangle{width: 100, height: 200}});
    println!("{:#?}", Rectangle{..Rectangle{width: 100, height: 200}}.area());
    let mut r = Rectangle{width: 0, height: 0};
    println!("{:#?}", r.init().area());
    println!("{:#?}", Rectangle::copy(&Rectangle{width: 500, height: 400}).area());
}

fn area_for_tuple(t: (u32, u32)) -> u32 {
    t.0 * t.1
}

fn area(r: Rectangle) -> u32 {
    r.height * r.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn init(&mut self) -> &Rectangle {
        self.height = 200;
        self.width = 100;
        self
    }
}

impl Rectangle {
    fn copy(other: &Rectangle) -> Rectangle {
        Rectangle {
            ..*other
        }
    }
}