use std::fs::File;

#[allow(unused_mut, unused_variables)]
fn main() {
    let fp = File::open("filename.extname").expect("Failed to open file");
}