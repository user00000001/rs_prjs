// cargo test --help
// cargo test -- --help
// cargo test -- --test-threads=1
// cargo test -- --show-output
// cargo test it
// cargo test -- --ignored
// cargo test -- --include-ignored
// cargo test --test integration_test
// cargo test // not include integration tests in tests directory ?

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)] // compiled when cargo test, not when cargo build/run
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("It works!");
    }
    #[test]
    #[should_panic(expected = "Make this test fail")] // turn panic failure to pass.
    fn anthor() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    #[ignore]
    fn it_adds_three() {
        assert_ne!(5, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol1"),
            "Greeting did not contain name, value was: `{}`",
            result
        );
    }

    #[test]
    fn it_works_too() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_owned())
        }
    }

    #[test]
    fn it_works_too1() {
        let value: Result<(), String> = Err("Some Exceptions happened.".to_owned());
        assert!(value.is_err());
    }
    
    #[test]
    fn it_failed() -> Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_owned())
        }
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: i32) -> i32 {
    greeting("haha"); // avoid private function dead_code
    a + 2
}

fn greeting(name: &str) -> String { // private function
    format!("Hello {}!", name)
}