use std::thread;
use std::time::Duration;

struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a|a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2); // c.value is not None, it is 1;
}
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x; // Closures can capture values from their environment
    // fn equal_to_x(z: i32) -> bool {
    //     z == x // error[E0434]: can't capture dynamic environment in a fn item
    // }

    // let equal_to_x = move |z| z == x;
    // println!("Can't use x here: {:?}", x); // value borrowed here after move
    let y = 4;
    assert!(equal_to_x(y));
}

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // // let expensive_closure = |num| {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // ERROR: x type is String now, type mismatched

    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // simulated_expensive_calculation(intensity),
            // expensive_result,
            // expensive_closure(intensity),
            expensive_result.value(intensity),
        );
        println!(
            "Next, do {} situps!",
            // simulated_expensive_calculation(intensity),
            // expensive_result,
            // expensive_closure(intensity),
            expensive_result.value(intensity),
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                // simulated_expensive_calculation(intensity),
                // expensive_result,
                // expensive_closure(intensity),
                expensive_result.value(intensity),
            );
        }
    }
}