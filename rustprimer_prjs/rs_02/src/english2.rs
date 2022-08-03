pub mod greetings {
    pub fn hello() -> String {
        "Hello!".to_string()
    }
}

pub mod farewells {
    pub fn goodbye() -> String {
        "Goodbye!".to_string()
    }
}

pub use self::greetings::hello;