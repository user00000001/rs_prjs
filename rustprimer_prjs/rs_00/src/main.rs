#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // FFI - Foreign Function Interface: interact with other program language (FFI,IPC/RPC...)
  use libc;
  use libc::c_int;
  use libc::c_void;
  use libc::size_t;
  // {
  //   #[link(name="yourlib")] // i.e libyourlib.so / libyourlib.dll / libyourlib.dylib / libyourlib.a
  //   extern {
  //     fn your_func(arg1: c_int, arg2: *mut c_void) -> size_t; // declares ffi function
  //     fn your_func2(arg1: c_int, arg2: *mut c_void) -> size_t;
  //     static ffi_global: c_int; // declares ffi global variable
  //   }
  //   let result: size_t = unsafe { your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void) };
  //   pub fn your_func_wrapper(arg1: i32, arg2: &mut i32) -> isize {
  //     (unsafe { your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void) }) as isize
  //   }

  //   // C Compat: struct / enum
  //   //
  //   // #[repr(C)] // C compat struct 
  //   #[repr(C, packed)] // mem align
  //   struct RustObject {
  //     a: c_int,
  //     // other members
  //   }

  //   // callback function
  //   //
  //   extern "C" fn callback(a: c_int) { // provide to C to invoke
  //     println!{"hello {}!", a};
  //   }
  //   #[link(name="yourlib")]
  //   extern {
  //     fn run_callback(data: i32, cb: extern fn(i32));
  //     // typedef void (*rust_callback)(int32_t); // define callback function type
  //     // void run_callback(int32_t data, rust_callback callback) {
  //     //   callback(data); // invoke callback function
  //     // }
  //   }
  //   unsafe {
  //     run_callback(1 as i32, callback); // print 1
  //   }
  // }

  use std::ffi::{CStr, CString};
  use libc::c_char;
  // {
  //   // CStr: C string to rust str
  //   // 
  //   #[link(name="yourlib")]
  //   extern {
  //     fn char_func() -> *mut c_char;
  //   }
  //   fn get_string() -> String {
  //     unsafe {
  //       // all chars are represented by utf8 in rust
  //       let raw_string: *mut c_char = char_func();
  //       let cstr = CStr::from_ptr(my_string());
  //       cstr.to_string_lossy() // all utf8 chars, zero copy to &str; not all utf8 chars, copy once, turn illegal char to U+FFFD
  //         .into_owned()
  //     }
  //   }

  //   // CString: rust str ot c string
  //   //
  //   use std::os::raw::c_char;
  //   extern {
  //     fn my_printer(s: *const c_char);
  //   }
  //   let c_to_print = CString::new("Hello, world!").unwrap(); // should not include \0 (which represents end of the string in c), will return Error
  //   unsafe {
  //     my_printer(c_to_print.as_ptr()) // turn CString to char pointer, then provide to c
  //   }
  // }

  // {
  //   // opaque struct
  //   //
  //   enum OpaqueStruct {}
  //   extern "C" { pub fn foo(arg: *mut OpaqueStruct); }
  //   // struct OpaqueStruct;
  //   // void foo(struct OpaqueStruct *arg);

  //   // empty pointer
  //   //
  //   0 as *const i32 as *const _;
  //   std::ptr::null::<i32>();

  //   // destruct c type need to implement Drop trait

  //   // Option<extern "C" fn(c_int) -> c_int> // optimzie to a emptiable function pointer

  //   extern {
  //     pub fn foo(arg: extern fn()-> *const c_char);
  //   }
  //   extern "C" fn danger() -> *const c_char {
  //     let cstring = CString::new("I'm a danger string").unwrap();
  //     // cstring.as_ptr() // as_ptr() take &self, CString is owned type, will drop when out of scope
  //     cstring.into_raw() // into_raw() take self, take the ownership, CString won't drop
  //   }
  //   unsafe {
  //     foo(danger); // pointer to an unknown mem space
  //   }

  //   // panic in ffi(i.e cffi) is unknown, should not use panic!/unimplemented!/unwrap...

  //   #[link(name="foo", kind="static")] // static libray
  //   #[link(name="CoreFoundation", kind="framework")] // mac os framework libray

  //   // extern "C" fn; // stdcall / appcs / cdecl / fastcall / vectorcall / Rust / rust-intrinsic / system / C / win64

  //   // ./bindgen [options] input.h // rust-bindgen: generate extern from declare head file
  
  //   #[no_mangle] // C can link this with "test" name
  //   extern "C" fn test() {} // nm [lib/bin]
  // }

  {
    // crate default type rlib, can be specified as follow 
    // 1, #![crate_type="foo"] at the source file top // foo might be bin / lib / rlib / dylib / staticlib 
    // 2, rustc --crate-type foo, in command
    // 3, cargo in Cargo.toml, crate-type = ["foo"]

    use std::mem::transmute;
    #[derive(Debug)]
    struct Foo<T> {
      t: T
    }
    #[no_mangle]
    extern "C" fn new_foo_vec() -> *const c_void {
      Box::into_raw(Box::new(Foo {t: vec![1,2,3]})) as *const c_void
    }
    #[no_mangle]
    extern "C" fn new_foo_int() -> *const c_void {
      Box::into_raw(Box::new(Foo {t: 1})) as *const c_void
    }
    fn push_foo_element(t: &mut Foo<Vec<i32>>) { t.t.push(1); }
    #[no_mangle]
    extern "C" fn push_foo_element_c(foo: *mut c_void) {
      let foo2 = unsafe {
        &mut *(foo as *mut Foo<Vec<i32>>) // Foo<Vec<i32>>? Foo<i32> in case
      };
      push_foo_element(foo2);
    }

    use std::any::Any;
    #[no_mangle]
    extern "C" fn new_foo_vec1() -> *const c_void {
      Box::into_raw(Box::new(Box::new(Foo{t: vec![1,2,3]}) as Box<dyn Any>)) as *const c_void
    }
    #[no_mangle]
    extern "C" fn new_foo_int1() -> *const c_void {
      Box::into_raw(Box::new(Box::new(Foo{t: 1}) as Box<dyn Any>)) as *const c_void
    }
    fn push_foo_element1(t: &mut Foo<Vec<i32>>) { t.t.push(1) }
    #[no_mangle]
    extern "C" fn push_foo_element_c1(foo: *mut c_void) {
      let foo2 = unsafe {
        &mut *(foo as *mut Box<dyn Any>)
      };
      let foo3: Option<&mut Foo<Vec<i32>>> = foo2.downcast_mut(); // error if foo2 is not *const Box<Foo<Vec<i32>>>
      if let Some(value) = foo3 {
        push_foo_element1(value);
      }
    }
  }
  

  Ok(())
}