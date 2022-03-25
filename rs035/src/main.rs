use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut _num = 5;
    let r1 = &_num as *const i32;
    let r2 = &mut _num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r1 is: {}, {:#?}", *r1, r1);
        println!("r2 is: {}, {:#?}", *r2, r2);
        let changed = 100;
        _num = changed;
        println!("num is set to {}, r2 is: {}", changed, *r2);
        println!("r is: {:?} {:#?}", r, r);
    }
//------------------
    unsafe {
        dangerous();
    }
//------------------
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);
    // let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
//-----------------
    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) }; // Segmentation fault (core dumped)
    // println!("{:?}", slice);
//-----------------
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
//-----------------
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
//-----------------
    let mut a: i32 = 100;
    a.foo();
    unsafe {
        a.unsafe_foo();
    }
}

unsafe fn dangerous() {
    println!("Dangerous Function");
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..]) // borrow as mutable more than once
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
}

#[no_mangle] // link with no mangle function name: call_from_c
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // method go here
    fn foo(&self);
    unsafe fn unsafe_foo(&mut self);
}

unsafe impl Foo for i32 {
    // method implementations go here
    fn foo(&self) {
        println!("foo here, {}, {:?}", self, &self)
    }
    unsafe fn unsafe_foo(&mut self) {
        *self = 1000;
        println!("foo here, {}, {:?}", self, &self);
        #[allow(unused_unsafe)]
        unsafe {
            let s = self as *mut i32;
            println!("s: {:?}", s);
        }
        println!("foo here, {}, {:?}", self, &self);
    }
}
