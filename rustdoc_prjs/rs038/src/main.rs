fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32)->i32, arg: i32) -> i32 { // fn is a type rather than a trait
    f(arg) + f(arg)
}

fn do_twice1<F: Fn(i32)->i32>(f: F, arg: i32) -> i32 { // Fn is a trait
    f(arg) + f(arg)
}

#[allow(unused)]
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    let a = |x|x+1;
    Box::new(a)
}

fn main() {
    println!("The answer is: {}, {}", do_twice(add_one, 5), do_twice1(add_one, 5));
    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i|i.to_string()).collect();
    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?} {:?} {:?}", list_of_numbers, list_of_strings, list_of_strings1);
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}, {}, {}", list_of_statuses, returns_closure()(10), returns_closure()(100));
}
