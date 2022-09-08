#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]
use std::fmt::Debug;

fn main() {
    {
      trait HasArea {
        fn area(&self) -> f64;
      }
      struct Circle {
        x: f64,
        y: f64,
        radius: f64,
      }
      impl HasArea for Circle {
        fn area(&self) -> f64 {
          std::f64::consts::PI * (self.radius * self.radius)
        }
      }
      let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
      };
      println!("circle c has an area of {}", c.area());

      fn foo<T: Debug>(s: T) {
        println!("{:?}", s);
      }
      foo("&str");
      foo(0);

      fn foo1<T: Debug + Clone>(s: T) {
        s.clone();
        println!("{:?}", s);
      }
      foo1("&str");
      foo1(0);

      fn foo2<T: Clone, K: Clone + Debug>(x: T, y: K) {
        x.clone();
        y.clone();
        println!("{:?}", y);
      }

      fn foo3<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
      }

      fn foo4<T, K>(x: T, y: K)
      where T: Clone,
            K: Clone + Debug
      {
        x.clone();
        y.clone();
        println!("{:?}", y);
      }

      impl HasArea for i32 {
        fn area(&self) -> f64 {
          *self as f64
        }
      }
      println!("{}", 5.area());

      use std::io::Write;
      let mut f = std::fs::File::open("../../target/foo.txt").expect("Couldn't open foo.txt");
      let buf = b"whatever";
      let result = f.write(buf);
      // result.unwrap(); // ignore the error

      trait Foo {
        fn is_valid(&self) -> bool;
        fn is_invalid(&self) -> bool { !self.is_valid() }
        fn foo(&self);
      }
      trait FooBar: Foo {
        fn foobar(&self);
      }
      #[derive(Debug)]
      struct Baz;
      impl Foo for Baz {
        fn is_valid(&self) -> bool { true }
        fn foo(&self) { println!("foo"); }
      }
      impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
      }
      // let baz = Baz{};
      let baz = Baz;
      println!("{:?} {}", baz, baz.is_invalid());
      baz.foo();
      baz.foobar();
    }

    {
      trait Foo { fn method(&self) -> String; }
      impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
      impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
      fn do_something(x: &dyn Foo) {
        println!("{}", x.method());
      }
      let x = "Hello".to_string();
      do_something(&x);
      let y = 8u8;
      do_something(&y);

      pub struct TraitObject { // &SomeTrait
        pub data: *mut (), // to obj pointer
        pub vtable: *mut (), // to obj's method array
      }
      struct FooVtable { // trait Foo's &Foo
        destructor: fn(*mut ()),
        size: usize,
        align: usize,
        method: fn(*const()) -> String,
      }
      // impl Foo for u8
      fn call_method_on_u8(x: *const ()) -> String {
        let byte: &u8 = unsafe { &*(x as *const u8) };
        byte.method()
      }
      fn destructor_at_u8(x: *mut()) {
        println!("destructor of u8: {:?} called", unsafe { &*(x as *const u8) })
      }
      static Foo_for_u8_vtable: FooVtable = FooVtable {
        destructor: destructor_at_u8 as fn(*mut ()),
        size: 1,
        align: 1,
        method: call_method_on_u8 as fn(*const()) -> String,
      };

      // impl Foo for String
      fn call_method_on_String(x: *const ()) -> String {
        let string: &String = unsafe { &*(x as *const String) };
        string.method()
      }
      fn destructor_at_String(x: *mut()) {
        println!("destructor of String: {:?} called", unsafe { &*(x as *const String) })
      }
      static Foo_for_String_vtable: FooVtable = FooVtable {
        destructor: destructor_at_String,
        size: 24,
        align: 8,
        method: call_method_on_String as fn(*const ()) -> String,
      };

      let a: String = "foo".to_string();
      let b = TraitObject {
        data: &a as *const String as *mut(),
        vtable: &Foo_for_String_vtable as *const FooVtable as *mut()
      };
      // b.method();
      let StringOutput = unsafe { ((*(b.vtable as *mut FooVtable)).method)(b.data) };
      println!("Raw Operation: {}", StringOutput);
      unsafe { ((*(b.vtable as *mut FooVtable)).destructor)(b.data); }

      let x: u8 = 1;
      // let y: &Foo = x;
      let y = TraitObject {
        data: &x as *const u8 as *mut(),
        vtable: &Foo_for_u8_vtable as *const FooVtable as *mut()
      };
      // y.method();
      let u8Output = unsafe { ((*(y.vtable as *mut FooVtable)).method)(y.data) };
      println!("Raw Operation: {}", u8Output);
      unsafe { ((*(y.vtable as *mut FooVtable)).destructor)(y.data); }
    }

    {
      // let v = vec![1,2,3];
      // let o = &v as &dyn Clone;
      pub trait Clone: Sized {
        fn clone(&self) -> Self; // Self not Sized
        fn clone_from(&mut self, source: &Self) { /* ... */ }
      }
      
      // trait method is object safe
      // 1, Self: Sized constraint
      // or 
      // 1, no generic parameters & not static function & not using Self type (exclude self and return parameter position)

      // trait object is safe
      // 1, all trait methods is object safe,
      // 2, trait no need for Self: Sized constraint
    }
}