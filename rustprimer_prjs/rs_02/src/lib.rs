#![allow(unused)]

pub mod chinese {
    mod greetings {

    }
    mod farewells {

    }
}

pub mod english {
    mod greetings {

    }
    mod farewells {

    }
}

pub mod chinese1;

#[path="english2.rs"]
pub mod english1;

use foo::baz::foobaz; // foo is at the root of the crate

mod foo {
    use crate::foo::bar::foobar; // foo is at crate root
    use self::baz::foobaz; // self refers to module 'foo
    pub mod bar {
        pub fn foobar() {}
    }
    pub mod baz {
        use super::bar::foobar; // super refers to module 'foo'
        pub fn foobaz() {}
    }
}