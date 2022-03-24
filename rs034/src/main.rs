fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
//------------------
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
//-------------------
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
//------------------
    let (a, b, c) = (v[0], v[1], v[2]);
    print_coordinates(&(a, b, c));
//------------------
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
//------------------
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"), 
    }
//------------------
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
//------------------
    let x = 'c';

    match x {
        // 'a'..'j' => println!("early ASCII letter"), // ERROR: exclusive range pattern syntax is experimental
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
//------------------
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y1) => println!("Matched, y = {:?}", y1),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
//-------------------
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);
//-------------------
    let Point { x, y } = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);
//-------------------
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: {} {}", x, y),
    }
//-------------------
    #[allow(unused)]
    enum Message {
        Quit,
        Move { x:i32, y:i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        ChangeColor1(Color),
    }
    #[allow(unused)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor1(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => println!(r#"The Quit variant has 
        no data to destructure."#),
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}", 
                x, y
            )
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to 
            red {}, 
            green {}, 
            and blue {}",
            r, g, b
        ),
        Message::ChangeColor1(Color::Rgb(r, g, b)) => println!(
            "Change the color to 
            red {}, 
            green {}, 
            and blue {}",
            r, g, b
        ),
        Message::ChangeColor1(Color::Hsv(h, s, v)) => println!(
            "Change the color to 
            hue {}, 
            saturation {}, 
            and value {}",
            h, s, v
        ),
    } 
//---------------------
    let ((feet, inches), Point { x, y }) =((3, 10), Point { x: 3, y: -10 });
    println!("x: {2: >10}, y: {3:_<10}, feet: {0}, inches: {1}, float: {4:3.2}", feet, inches, x, y, 1000.0);
//---------------------
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    let new_setting_value1: Option<i32> = None;

    match (setting_value, new_setting_value, new_setting_value1) {
        (Some(_), Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);
//---------------------
    let numbers = (2,4,8,16,32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    match numbers {
        (first,..,last) => {
        // (..,second,..) => { // ERROR: cannot index second
            println!("Some numbers: {}, {}", first, last);
        }
    }
//--------------------
    let _unused_var = 1000;
//--------------------
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
    // if let Some(_s) = s { // s is moved to _s, s should not be use again.
        println!("found a string");
    }
    println!("{:?}", s);
//-------------------
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point {x,..} => println!("x axis: {}", x),
    }
//-------------------
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        _ => (),
    }
//-------------------
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n == y, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);
//------------------
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // (4|5|6) if true => ...
        _ => println!("no"),
    }
//-----------------
    enum Message1 {
        Hello { id: i32},
    }
    let msg = Message1::Hello { id: 5 };
    match msg {
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message1::Hello { id: 10..=12 } => println!("Found an id in another range"),
        // Message1::Hello { id } => println!("Found some other id: {}", id),
        Message1::Hello { id: _id } => println!("Found some other id: {}", _id),
    }
}

fn print_coordinates(&(a,_,c): &(char, char, char)) {
    println!("Current location: ({}, _, {})", a, c);
}