use rs025; 

// #[cfg(test)] no need in tests directory.

mod common;

#[test]
fn it_adds_two_in_tests() {
    common::setup();
    assert_eq!(4, rs025::add_two(2));
}