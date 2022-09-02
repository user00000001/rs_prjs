#![allow(unused, deprecated, ellipsis_inclusive_range_patterns)]

fn main() {
    {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            }
            _ => println!("West"),
        }

        let d_panic = Direction::South;
        let d_west = Direction::West;
        let d_str = match d_west {
            Direction::East => "East",
            Direction::North | Direction::South => panic!("South or North"),
            _ => "West",
        };
        println!("{}", d_str);

        enum Action {
            Say(String),
            MoveTo(i32, i32),
            ChangeColorRGB(u16, u16, u16),
        }
        let action = Action::Say("Hello Rust".to_string());
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g);
            }
        }
    }

    {
        let tup = (0u8, 1u8);
        let (x, y) = tup;

        let x = 1;
        let c = 'c';
        match c {
            x => println!("x: {}, c: {}", x, c)
        }
        println!("x: {}", x);

        struct Point {
            x: i64,
            y: i64,
        }
        let point = Point { x: 0, y: 0 };
        match point {
            // Point { x, y } => println!("({},{})", x, y)
            // Point { x: x1, y: y1 } => println!("({},{})", x1, y1)
            Point { y, .. } => println!("y is {}", y)
        }

        let tuple: (u32, String) = (5, String::from("five"));
        let (x, s) = tuple; // tuple moved
        // println!("Tuple is: {:?}", tuple); // compile error
        let tuple = (5, String::from("five"));
        let (x, _) = tuple; // u32 implements Copy Trait; _ or .. not moved. so tuple was not moved.
        println!("Tuple is: {:?}", tuple);

        let x = 1;
        match x {
            1 ... 10 => println!("一到十"),
            _ => println!("其它")
        }

        let c = 'w';
        match c {
            'a' ... 'z' => println!("小写字母"),
            'A' ... 'Z' => println!("大写字母"),
            _ => println!("其他字符"),
        }

        let x = 1;
        match x {
            1 | 2 => println!("一或二"),
            _ => println!("其他")
        }

        let mut x = 5;
        match x {
            ref mut mr => println!("mut ref: {}", mr),
        }
        let ref mut mrx = x;

        let x = 1u32;
        match x {
            e @ 1 ... 5 | e @ 10 ... 15 => println!("get: {}", e),
            _ => (),
        }

        #[derive(Debug)]
        struct Persion {
            name: Option<String>
        }
        let name = "Steve".to_string();
        let x: Option<Persion> = Some(Persion{name: Some(name)});
        match x {
            // Some(Persion{name: ref a @ Some(_), ..}) => println!("{:?}", a),
            Some(Persion{name: ref a @ Some(ref b), ..}) => println!("{:?}, {}", a, b),
            _ => {}
        }

        let x = 4;
        let y = false;
        match x {
            4|5 if y => println!("yes"), // match guards
            _ => println!("no"),
        }
    }
}